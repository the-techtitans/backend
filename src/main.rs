use axum::{
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use db_structs::{PatientID, PrevAppointments, City, DoctorInfo};
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
        .route("/doctors", post(doctors));


    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> String {
    format!("Hello world")
}

async fn prevapp(Json(payload): Json<PatientID>) -> Response {
    tracing::debug!("Got request to view previous appointments for patient ID {}", payload.patient_id);
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
