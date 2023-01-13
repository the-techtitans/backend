//create structs for interfacing with the database
use dotenvy::dotenv;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use std::fmt::Display;
use std::str::FromStr;
use tracing;

#[derive(sqlx::FromRow, Serialize)]
pub struct PrevAppointments {
    docname: String,
    timestamp: String,
    apptype: String,
    status: String,
    prescription: String,
    appname: String,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct DoctorInfo {
    docid: i32,
    docname: String,
    specname: String,
    address: String,
}

#[derive(Deserialize)]
pub struct PatientID {
    #[serde(deserialize_with = "from_str")]
    pub patient_id: i32,
}

#[derive(Deserialize)]
pub struct City {
    pub city: String,
}

pub struct Database {
    connection: Pool<Postgres>,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub async fn init() -> Option<Database> {
    dotenv().ok();
    match env::var("DATABASE_URL") {
        Ok(url) => {
            tracing::debug!("Found database URL: {}", url);
            if let Ok(pool) = PgPoolOptions::new().max_connections(5).connect(&url).await {
                tracing::debug!("Connected to database!");
                return Some(Database { connection: pool });
            } else {
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

    pub async fn view_prev_appointments(&self, patient_id: i32) -> Vec<PrevAppointments> {
        let query = format!("
                    select d.name as docname, a.date_time as timestamp, a.type as apptype, a.status as appstatus, a.prescription as prescription, p.name as appname
                    from patients_previous_appointments a
                    join doctors d on d.id = a.doctor_id
                    join specialities p on p.id = a.appointment_type
                    where a.patient_id = {}
                    order by a.date_time desc
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
                    where d.city = {}
                    ;", city);
        let result = sqlx::query_as::<_, DoctorInfo>(&query)
            .fetch_all(&self.connection)
            .await
            .expect("Error in database");
        result
    }

}
