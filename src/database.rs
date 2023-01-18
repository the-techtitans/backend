//create structs for interfacing with the database
use argon_hash_password;
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::env;
use tracing;

use crate::db_structs::*;

pub struct Database {
    jwt_secret: Vec<u8>,
    connection: Pool<Postgres>,
}

pub async fn init() -> Option<Database> {
    dotenv().ok();
    let dburl = env::var("DATABASE_URL");
    let secret = env::var("SECRET");
    match secret {
        Ok(sec) => {
            tracing::debug!("Found secret");
            match dburl {
                Ok(url) => {
                    tracing::debug!("Found database URL: {}", url);
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
                Err(_) => {
                    eprintln!("Failed to connect to database!");
                    return None;
                }
            }
        }
        Err(_) => {
            eprintln!("Did not find a secret!");
            return None;
        }
    }
}

impl Database {
    pub async fn view_prev_appointments(&self, patient_id: i64) -> Vec<PrevAppointments> {
        let query = format!("
                    select d.name as docname, TO_CHAR(a.date_time, 'YYYY-MM-DD HH24:MM:SS') as timestamp, a.type as apptype, a.status as appstatus, a.prescription as prescription, p.name as appname
                    from patients_previous_appointments a
                    join doctors d on d.id = a.doctor_id
                    join specialities p on p.id = a.appointment_type
                    where a.patient_id = {}
                    order by timestamp desc
                    ;", patient_id);
        match sqlx::query_as::<_, PrevAppointments>(&query)
            .fetch_all(&self.connection)
            .await {
                Ok(result) => result,
                Err(_) => {
                    tracing::error!("Error while viewing previous appointments");
                    let empty : Vec<PrevAppointments> = Vec::new();
                    empty
                }
            }
    }

    pub async fn view_same_city_doctors(&self, city: String) -> Vec<DoctorInfo> {
        let query = format!("
                    select d.id as docid, d.name as docname, s.name as specname, d.address as address
                    from doctors d
                    join specialities s on s.id = d.speciality_id
                    where d.city = '{}'
                    ;", city);
        match sqlx::query_as::<_, DoctorInfo>(&query)
            .fetch_all(&self.connection)
            .await {
                Ok(result) => result,
                Err(_) => {
                    tracing::error!("Error while viewing doctors");
                    let empty : Vec<DoctorInfo> = Vec::new();
                    empty
                }
            }
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
        match sqlx::query_as::<_, PatientInfo>(&query)
            .fetch_all(&self.connection)
            .await {
                Ok(result) => result,
                Err(_) => {
                    tracing::error!("Error while viewing patient info");
                    let empty : Vec<PatientInfo> = Vec::new();
                    empty
                }
            }
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
        match sqlx::query_as::<_, DoctorPrices>(&query)
            .fetch_all(&self.connection)
            .await {
                Ok(result) => result,
                Err(_) => {
                    tracing::error!("Error while viewing doctors and prices");
                    let empty : Vec<DoctorPrices> = Vec::new();
                    empty
                }
            }
    }

    pub async fn view_specialities(&self) -> Vec<Specialities> {
        let query = "select id, name, description as desc
                    from specialities;";
        let result = sqlx::query_as::<_, Specialities>(&query)
            .fetch_all(&self.connection)
            .await
            .expect("Error in database");
        result
    }

    pub async fn register(&self, email: &String, password: &String, isdoctor: bool) -> bool {
        let (hash, salt) = argon_hash_password::create_hash_and_salt(&password).unwrap();
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
        let naivedatetime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").unwrap();
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
                let check = argon_hash_password::check_password_matches_hash(
                    password,
                    &result.hashedpass,
                    &result.salt,
                )
                .unwrap();
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
                    let id: i64 = sqlx::query(&query)
                        .fetch_one(&self.connection)
                        .await
                        .expect("Error in database")
                        .try_get("id")
                        .unwrap();
                    let jwt = InternalJWT {
                        isdoctor: result.isdoctor,
                        id: id.to_string(),
                        exp: 1000000,
                    };
                    let token = encode(
                        &Header::default(),
                        &jwt,
                        &EncodingKey::from_secret(&self.jwt_secret),
                    )
                    .unwrap();
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
                let id: i64 = token.claims.id.parse().unwrap();
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
