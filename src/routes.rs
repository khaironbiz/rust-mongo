use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use crate::{handlers::*, db::AppState};
use std::sync::Arc;

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/medical-records", get(get_medical_records).post(create_medical_record))
        .route("/medical-records/:id", get(get_medical_record).put(update_medical_record).delete(delete_medical_record))
        .route("/doctors", get(get_doctors).post(create_doctor))
        .route("/nurses", get(get_nurses))
        .route("/medicines", get(get_medicines))
        .route("/appointments", get(get_appointments).post(create_appointment))
        .route("/services", get(get_services))
        .route("/insurances", get(get_insurances))
        .with_state(state)
        .layer(cors)
}
