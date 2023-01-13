use axum::{
    response::{Response, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use database::{PatientID, PrevAppointments};
use std::net::SocketAddr;
use tokio;
use tracing;
use tracing_subscriber;

mod database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/prevapp", post(prevapp));

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
    let res = match database::init().await {
        Some(conn) => conn.view_prev_appointments(payload.patient_id).await,
        None => {
            let res: Vec<PrevAppointments> = Vec::new();
            res
        }
    };
    Json(res).into_response()
}
