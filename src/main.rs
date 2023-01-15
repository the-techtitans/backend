use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use db_structs::*;
use std::net::SocketAddr;
use tokio;
use tracing;
use tracing_subscriber;

mod database;
mod db_structs;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/prevapp", post(prevapp))
        .route("/doctors", post(doctors))
        .route("/patient", post(patient))
        .route("/find", get(find));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}

async fn prevapp(Json(payload): Json<PatientID>) -> Response {
    tracing::debug!(
        "Got request to view previous appointments for patient ID {}",
        payload.patient_id
    );
    let res = match database::init().await {
        Some(conn) => conn.view_prev_appointments(payload.patient_id).await,
        None => {
            let res: Vec<PrevAppointments> = Vec::new();
            res
        }
    };
    Json(res).into_response()
}

async fn doctors(Json(payload): Json<City>) -> Response {
    tracing::debug!("Got request to view doctors in city {}", payload.city);
    let res = match database::init().await {
        Some(conn) => conn.view_same_city_doctors(payload.city).await,
        None => {
            let res: Vec<DoctorInfo> = Vec::new();
            res
        }
    };
    Json(res).into_response()
}

async fn patient(Json(payload): Json<PatientID>) -> Response {
    tracing::debug!(
        "Got request to view patient info corresponding to patient ID {}",
        payload.patient_id
    );
    let res = match database::init().await {
        Some(conn) => conn.view_patient_info(payload.patient_id).await,
        None => {
            let res: Vec<PatientInfo> = Vec::new();
            res
        }
    };
    Json(res).into_response()
}

async fn find(payload: Query<CityApptype>) -> Response {
    tracing::debug!(
        "Got request to view all doctors with appointment type {} in city {}",
        payload.apptype, payload.city
    );
    let res = match database::init().await {
        Some(conn) => conn.view_doctor_prices(&payload.city, &payload.apptype).await,
        None => {
            let res: Vec<DoctorPrices> = Vec::new();
            res
        }
    };
    Json(res).into_response()
}
