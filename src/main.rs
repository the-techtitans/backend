use axum::{
    extract::Query,
    http::{
        header::{HeaderMap, AUTHORIZATION},
        Method, StatusCode,
    },
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use db_structs::*;
use std::net::SocketAddr;
use tokio;
use tower_http::cors::{Any, CorsLayer};
use tracing;
use tracing_subscriber;

mod database;
mod db_structs;

async fn authenticate(
    conn: &database::Database,
    headers: HeaderMap,
    given_id: &i64,
    isdoctor: bool,
) -> bool {
    match conn.verify_jwt(headers[AUTHORIZATION].to_str().unwrap()) {
        Some(jwt) => {
            tracing::debug!("Verified and parsed JWT");
            if *given_id == jwt.id && isdoctor == jwt.isdoctor {
                tracing::debug!("Correct JWT is given!");
                return true;
            } else {
                tracing::error!("Incorrect JWT!");
                return false;
            }
        }
        None => {
            tracing::debug!("Could not verify JWT!");
            false
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .allow_methods([Method::GET, Method::POST]);
    let app = Router::new()
        .route("/", get(root))
        .route("/prevapp", post(prevapp))
        .route("/doctors", post(doctors))
        .route("/patient", post(patient))
        .route("/find", get(find))
        .route("/login", post(login))
        .route("/newpatient", post(newpatient))
        .route("/newdoctor", post(newdoctor))
        .route("/newappointment", post(newappointment))
        .route("/specialities", get(specialities))
        .route("/prescriptions", post(prescriptions))
        .layer(cors);

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

async fn prescriptions(headers: HeaderMap, Json(payload): Json<PatientID>) -> Response {
    tracing::debug!(
        "Got request to view previous appointments for patient ID {}",
        payload.patient_id
    );
    let res = match database::init().await {
        Some(conn) => {
            if authenticate(&conn, headers, &payload.patient_id, false).await {
                let res = conn.view_prescriptions(payload.patient_id).await;
                res
            } else {
                let res: Vec<Prescriptions> = Vec::new();
                res
            }
        }
        None => {
            let res: Vec<Prescriptions> = Vec::new();
            res
        }
    };
    Json(res).into_response()
}


async fn prevapp(headers: HeaderMap, Json(payload): Json<PatientID>) -> Response {
    tracing::debug!(
        "Got request to view previous appointments for patient ID {}",
        payload.patient_id
    );
    let res = match database::init().await {
        Some(conn) => {
            if authenticate(&conn, headers, &payload.patient_id, false).await {
                let res = conn.view_prev_appointments(payload.patient_id).await;
                res
            } else {
                let res: Vec<PrevAppointments> = Vec::new();
                res
            }
        }
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

async fn patient(headers: HeaderMap, Json(payload): Json<PatientID>) -> Response {
    tracing::debug!(
        "Got request to view patient info corresponding to patient ID {}",
        payload.patient_id
    );
    let res = match database::init().await {
        Some(conn) => {
            if authenticate(&conn, headers, &payload.patient_id, false).await {
                conn.view_patient_info(payload.patient_id).await
            } else {
                let res: Vec<PatientInfo> = Vec::new();
                res
            }
        }
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
        payload.apptype,
        payload.city
    );
    let res = match database::init().await {
        Some(conn) => {
            conn.view_doctor_prices(&payload.city, &payload.apptype)
                .await
        }
        None => {
            let res: Vec<DoctorPrices> = Vec::new();
            res
        }
    };
    Json(res).into_response()
}

async fn newpatient(Json(payload): Json<Patient>) -> Response {
    tracing::debug!("Got request to insert new patient info");
    match database::init().await {
        Some(conn) => {
            let res = conn
                .add_new_patient(&payload.name, &payload.email, &payload.phone)
                .await
                && conn
                    .register(&payload.email, &payload.password, false)
                    .await;
            if res {
                tracing::debug!("Record inserted successfully");
                return (StatusCode::OK, Json("Inserted")).into_response();
            } else {
                return (StatusCode::BAD_REQUEST, Json("Error while inserting")).into_response();
            }
        }
        None => {
            return (StatusCode::BAD_REQUEST, Json("Error while inserting")).into_response();
        }
    }
}

async fn newdoctor(Json(payload): Json<Doctor>) -> Response {
    tracing::debug!("Got request to insert new doctor info");
    match database::init().await {
        Some(conn) => {
            let res = conn
                .add_new_doctor(
                    &payload.name,
                    payload.speciality,
                    &payload.city,
                    &payload.address,
                    &payload.email,
                    &payload.phone,
                )
                .await
                && conn.register(&payload.email, &payload.password, true).await;
            if res {
                tracing::debug!("Record inserted successfully");
                return (StatusCode::OK, Json("Inserted")).into_response();
            } else {
                return (StatusCode::BAD_REQUEST, Json("Error while inserting")).into_response();
            }
        }
        None => {
            return (StatusCode::BAD_REQUEST, Json("Error while inserting")).into_response();
        }
    }
}

async fn newappointment(headers: HeaderMap, Json(payload): Json<Appointment>) -> Response {
    tracing::debug!("Got request to insert new appointment info");
    match database::init().await {
        Some(conn) => {
            if authenticate(&conn, headers, &payload.patient_id, false).await {
                let res = conn
                    .add_new_appointment(
                        payload.doctor_id,
                        payload.patient_id,
                        payload.apptype,
                        &payload.datetime,
                        &payload.phyorvirt,
                        &payload.status,
                        &payload.prescription,
                    )
                    .await;
                if res {
                    tracing::debug!("Record inserted successfully");
                    return (StatusCode::OK, Json("Inserted")).into_response();
                } else {
                    return (StatusCode::BAD_REQUEST, Json("Error while inserting"))
                        .into_response();
                }
            } else {
                return (StatusCode::BAD_REQUEST, Json("Error while inserting")).into_response();
            }
        }
        None => {
            return (StatusCode::BAD_REQUEST, Json("Error while inserting")).into_response();
        }
    }
}

async fn specialities() -> Response {
    tracing::debug!("Got request to fetch specialities");
    let res = match database::init().await {
        Some(conn) => conn.view_specialities().await,
        None => {
            let res: Vec<Specialities> = Vec::new();
            res
        }
    };
    Json(res).into_response()
}

async fn login(Json(payload): Json<Login>) -> Response {
    tracing::debug!("Got request to login");
    match database::init().await {
        Some(conn) => {
            let res = conn.login(&payload.email, &payload.password).await;
            match res {
                Some(jwt) => {
                    tracing::debug!("Generated JWT successfully! {}", jwt);
                    return (StatusCode::OK, Json(jwt)).into_response();
                }
                None => {
                    return (StatusCode::BAD_REQUEST, Json("Error while logging in"))
                        .into_response();
                }
            }
        }
        None => {
            return (StatusCode::BAD_REQUEST, Json("Error while logging in")).into_response();
        }
    }
}
