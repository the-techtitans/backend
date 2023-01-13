use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::Display;
use std::str::FromStr;

#[derive(FromRow, Serialize)]
pub struct PrevAppointments {
    docname: String,
    timestamp: String,
    apptype: String,
    appstatus: String,
    prescription: String,
    appname: String,
}

#[derive(FromRow, Serialize)]
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

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}
