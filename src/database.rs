//create structs for interfacing with the database
use argon_hash_password;
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{postgres::PgPoolOptions, postgres::PgRow, Pool, Postgres, Row};
use std::env;
use tracing;

use crate::db_structs::*;

pub struct Database {
    jwt_secret: Vec<u8>,
    connection: Pool<Postgres>,
}

pub async fn init() -> Option<Database> {
    dotenv().ok();
    let Ok(url) = env::var("DATABASE_URL") else {
        tracing::error!("Couldn't find DATABASE_URL, aborting");
        return None;
    };
    let Ok(sec) = env::var("SECRET") else {
        tracing::error!("Couldn't find SECRET, aborting");
        return None;
    };
    tracing::debug!("Found database URL: {} and secret", url);
    match PgPoolOptions::new().connect(&url).await {
        Ok(pool) => {
            tracing::debug!("Connected to database!");
            return Some(Database {
                connection: pool,
                jwt_secret: sec.as_bytes().to_vec(),
            });
        }
        Err(e) => {
            tracing::error!("Could not connect using URL {}", url);
            tracing::error!("Error: {}", e);
            return None;
        }
    }
}

impl Database {
    async fn get_query_result<ResultStruct, DB>(&self, query: &String) -> Vec<ResultStruct>
    where
        ResultStruct: for<'r> sqlx::FromRow<'r, <DB as sqlx::Database>::Row>,
        ResultStruct: Unpin,
        ResultStruct: Send,
        DB: sqlx::Database<Row = PgRow>,
    {
        match sqlx::query_as::<_, ResultStruct>(&query)
            .fetch_all(&self.connection)
            .await
        {
            Ok(result) => result,
            Err(_) => {
                tracing::error!("Error while running query");
                let empty: Vec<ResultStruct> = Vec::new();
                empty
            }
        }
    }

    pub async fn view_prescriptions(&self, patient_id: i64) -> Vec<Prescriptions> {
        let query = format!("
                    (select d.name as docname, TO_CHAR(a.date_time, 'YYYY-MM-DD HH24:MM:SS') as timestamp, a.prescription as prescription
                    from patients_previous_appointments a
                    join doctors d on d.id = a.doctor_id
                    where a.patient_id = {}
                    order by timestamp desc)
                    UNION
                    (select d.name as docname, TO_CHAR(a.date_time, 'YYYY-MM-DD HH24:MM:SS') as timestamp, a.prescription as prescription
                    from appointments a
                    join doctors d on d.id = a.doctor_id
                    where a.patient_id = {}
                    order by timestamp desc)
                    ;", patient_id, patient_id);
        self.get_query_result::<Prescriptions, Postgres>(&query)
            .await
    }

    pub async fn view_prev_appointments(&self, patient_id: i64) -> Vec<PrevAppointments> {
        let query = format!("
                    (select d.name as docname, TO_CHAR(a.date_time, 'YYYY-MM-DD HH24:MM:SS') as timestamp, a.type as apptype, a.status as appstatus, a.prescription as prescription, p.name as appname
                    from patients_previous_appointments a
                    join doctors d on d.id = a.doctor_id
                    join specialities p on p.id = a.appointment_type
                    where a.patient_id = {}
                    order by timestamp desc)
                    UNION
                    (select d.name as docname, TO_CHAR(a.date_time, 'YYYY-MM-DD HH24:MM:SS') as timestamp, a.type as apptype, a.status as appstatus, a.prescription as prescription, p.name as appname
                    from appointments a
                    join doctors d on d.id = a.doctor_id
                    join specialities p on p.id = a.appointment_type
                    where a.patient_id = {}
                    order by timestamp desc)
                    ;", patient_id, patient_id);
        self.get_query_result::<PrevAppointments, Postgres>(&query)
            .await
    }

    pub async fn view_same_city_doctors(&self, city: String) -> Vec<DoctorInfo> {
        let query = format!("
                    select d.id as docid, d.name as docname, s.name as specname, d.address as address
                    from doctors d
                    join specialities s on s.id = d.speciality_id
                    where d.city = '{}'
                    ;", city);
        self.get_query_result::<DoctorInfo, Postgres>(&query).await
    }

    pub async fn view_patient_info(&self, patient_id: i64) -> Vec<PatientInfo> {
        let query = format!(
            "
                    select name, email, phone
                    from patients
                    where id = {}
                    ;",
            patient_id
        );
        self.get_query_result::<PatientInfo, Postgres>(&query).await
    }

    pub async fn view_doctor_prices(&self, city: &String, apptype: &String) -> Vec<DoctorPrices> {
        let iscityspecified = match city.is_empty() {
            false => format!("and d.city = '{}'", city),
            true => String::new(),
        };
        let isapptypespecified = match apptype.is_empty() {
            false => format!("and t.name = '{}'", apptype),
            true => String::new(),
        };

        let query = format!(
            "
                    select d.name as docname, d.city as city, d.address as address, t.name as apptype, p.price
                    from doctors d
                    join appointment_types t on d.speciality_id = t.speciality_id
                    join appointment_prices p on d.id = p.doctor_id and t.id = p.appointment_type
                    where 1=1 {} {};
                    ",
            isapptypespecified, iscityspecified
        );
        self.get_query_result::<DoctorPrices, Postgres>(&query)
            .await
    }

    pub async fn view_specialities(&self) -> Vec<Specialities> {
        let query = String::from(
            "select id, name, description as desc
                    from specialities;",
        );
        self.get_query_result::<Specialities, Postgres>(&query)
            .await
    }

    pub async fn view_doctor_appointments(&self, doctor_id: i64) -> Vec<DoctorAppointments> {
        let query = format!(
            "select id, patient_id, appointment_type as apptype,
            TO_CHAR(date_time, 'YYYY-MM-DD HH24:MM:SS') as datetime,
            type as phyorvirt, status, prescription from appointments where doctor_id = {} order by date_time
            ", doctor_id
        );
        self.get_query_result::<DoctorAppointments, Postgres>(&query)
            .await
    }

    pub async fn register(&self, email: &String, password: &String, isdoctor: bool) -> bool {
        let Ok((hash, salt)) = argon_hash_password::create_hash_and_salt(&password) else {
            tracing::error!("Hash and salt were not able to be created, registration error");
            return false;
        };
        let query = format!(
            "
                    insert into login(email, password, salt, isdoctor) values ('{}', '{}', '{}', {})
                            ",
            email, hash, salt, isdoctor
        );
        match sqlx::query(&query).execute(&self.connection).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn add_new_patient(&self, name: &String, email: &String, phone: &String) -> bool {
        let query = format!(
            "
                    insert into patients(name, email, phone) values ('{}','{}','{}');
                            ",
            name, email, phone
        );
        match sqlx::query(&query).execute(&self.connection).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn add_new_doctor(
        &self,
        name: &String,
        speciality: i64,
        city: &String,
        address: &String,
        email: &String,
        phone: &String,
    ) -> bool {
        let query = format!("
                    insert into doctors(name, speciality_id, city, address, email, phone) values ('{}',{},'{}', '{}', '{}', '{}');
                            ", name, speciality, city, address, email, phone);
        match sqlx::query(&query).execute(&self.connection).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn add_new_appointment(
        &self,
        docid: i64,
        patid: i64,
        apptype: i64,
        datetime: &String,
        phyorvirt: &String,
        status: &String,
        prescription: &String,
    ) -> bool {
        let Ok(naivedatetime) = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S") else {
            tracing::error!("Couldn't parse date time into NaiveDateTime");
            return false;
        };
        let doctorapps = self.view_doctor_appointments(docid).await;
        for app in doctorapps.iter() {
            if app.datetime == *datetime && app.status != "cancelled" {
                tracing::error!("Appointment has already been booked");
                return false;
            }
        }
        let query = format!("
                    insert into appointments (doctor_id, patient_id, appointment_type, date_time, type, status, prescription) values ({},{},{},'{}','{}','{}', '{}')
                            ", docid, patid, apptype, naivedatetime, phyorvirt, status, prescription);
        match sqlx::query(&query).execute(&self.connection).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    //tries to find patient/doctor logging in with credentials and gives JWT if successful
    pub async fn login(&self, email: &String, password: &String) -> Option<String> {
        let query = format!(
            "
                    select salt, password as hashedpass, isdoctor from login where email = '{}';
                ",
            email
        );
        match sqlx::query_as::<_, LoginTable>(&query)
            .fetch_one(&self.connection)
            .await
        {
            Ok(result) => {
                let Ok(check) = argon_hash_password::check_password_matches_hash(
                    password,
                    &result.hashedpass,
                    &result.salt,
                ) else {
                    tracing::debug!("Couldn't check password matches hash");
                    return None;
                };
                if check {
                    let mut tablename = "patients";
                    if result.isdoctor {
                        tablename = "doctors";
                    }
                    let query = format!(
                        "
                                    select id from {} where email = '{}';
                                ",
                        tablename, email
                    );
                    let Ok(queryres) = sqlx::query(&query)
                        .fetch_one(&self.connection)
                        .await else {
                            tracing::error!("Error while checking login details in database");
                            return None;
                        };
                    let Ok(id): Result<i64, _> = queryres.try_get("id") else {
                        tracing::error!("Error while retrieving id from query result");
                        return None;
                    };
                    let jwt = InternalJWT {
                        isdoctor: result.isdoctor,
                        id: id.to_string(),
                        exp: 1000000,
                    };
                    let Ok(token) = encode(
                        &Header::default(),
                        &jwt,
                        &EncodingKey::from_secret(&self.jwt_secret),
                    ) else {
                        tracing::debug!("Error while trying to encode JWT");
                        return None;
                    };
                    Some(token)
                } else {
                    None
                }
            }
            Err(_) => {
                tracing::debug!("No such user found!");
                None
            }
        }
    }

    pub fn verify_jwt(&self, jwt: &str) -> Option<JWT> {
        let binding = match String::from(jwt)
            .split("Bearer")
            .collect::<Vec<&str>>()
            .get(1)
        {
            Some(x) => x.to_string(),
            None => jwt.to_string(),
        };
        let mut validation = Validation::default();
        validation.validate_exp = false;
        let token = binding.trim().to_string();
        tracing::debug!("jwt : '{}'", token);
        match decode::<InternalJWT>(
            &token,
            &DecodingKey::from_secret(&self.jwt_secret),
            &validation,
        ) {
            Ok(token) => {
                let Ok(id): Result<i64, _> = token.claims.id.parse() else {
                    tracing::error!("Could not parse id while verifiying JWT");
                    return None;
                };
                let res = JWT {
                    isdoctor: token.claims.isdoctor,
                    id,
                };
                Some(res)
            }
            Err(x) => {
                tracing::debug!("{}", x);
                None
            }
        }
    }
}
