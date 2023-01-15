//create structs for interfacing with the database
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tracing;
use chrono::NaiveDateTime;

use crate::db_structs::*;

pub struct Database {
    connection: Pool<Postgres>,
}

pub async fn init() -> Option<Database> {
    dotenv().ok();
    match env::var("DATABASE_URL") {
        Ok(url) => {
            tracing::debug!("Found database URL: {}", url);
            if let Ok(pool) = PgPoolOptions::new().connect(&url).await {
                tracing::debug!("Connected to database!");
                return Some(Database { connection: pool });
            } else {
                tracing::error!("Could not connect using URL {}", url);
                return None;
            }
        }
        Err(_) => {
            eprintln!("Failed to connect to database!");
            return None;
        }
    }
}

impl Database {
    pub async fn view_prev_appointments(&self, patient_id: i64) -> Vec<PrevAppointments> {
        let query = format!("
                    select d.name as docname, TO_CHAR(a.date_time, 'YYYY/MM/DD HH12:MM:SS') as timestamp, a.type as apptype, a.status as appstatus, a.prescription as prescription, p.name as appname
                    from patients_previous_appointments a
                    join doctors d on d.id = a.doctor_id
                    join specialities p on p.id = a.appointment_type
                    where a.patient_id = {}
                    order by timestamp desc
                    ;", patient_id);
        let result = sqlx::query_as::<_, PrevAppointments>(&query)
            .fetch_all(&self.connection)
            .await
            .expect("Error in database");
        result
    }

    pub async fn view_same_city_doctors(&self, city: String) -> Vec<DoctorInfo> {
        let query = format!("
                    select d.id as docid, d.name as docname, s.name as specname, d.address as address
                    from doctors d
                    join specialities s on s.id = d.speciality_id
                    where d.city = '{}'
                    ;", city);
        let result = sqlx::query_as::<_, DoctorInfo>(&query)
            .fetch_all(&self.connection)
            .await
            .expect("Error in database");
        result
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
        let result = sqlx::query_as::<_, PatientInfo>(&query)
            .fetch_all(&self.connection)
            .await
            .expect("Error in database");
        result
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
        let result = sqlx::query_as::<_, DoctorPrices>(&query)
            .fetch_all(&self.connection)
            .await
            .expect("Error in database");
        result
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

    pub async fn add_new_patient(&self, name: &String, email: &String, phone: &String) -> bool {
        let query = format!("
                    insert into patients(name, email, phone) values ('{}','{}','{}');
                            ", name, email, phone);
        match sqlx::query(&query).execute(&self.connection).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn add_new_doctor(&self, name: &String, speciality: i64, city: &String, address: &String) -> bool {
        let query = format!("
                    insert into doctors(name, speciality_id, city, address) values ('{}','{}','{}', '{}');
                            ", name, speciality, city, address);
        match sqlx::query(&query).execute(&self.connection).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn add_new_appointment(&self, docid: i64, patid: i64, apptype: i64, datetime: &String, phyorvirt: &String, status: &String, prescription: &String) -> bool {
        let naivedatetime = NaiveDateTime::parse_from_str(datetime, "%Y/%m/%d %H:%M:%S").unwrap();
        let query = format!("
                    insert into appointments (doctor_id, patient_id, appointment_type, date_time, type, status, prescription) values ({},{},{},'{}','{}','{}', '{}')
                            ", docid, patid, apptype, naivedatetime, phyorvirt, status, prescription);
        match sqlx::query(&query).execute(&self.connection).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

}
