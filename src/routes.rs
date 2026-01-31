use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use tower_http::cors::{Any, CorsLayer};
use crate::{handlers::*, db::AppState, middleware::auth_middleware};
use crate::docs;
use std::sync::Arc;

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes (no authentication required)
    let public_routes = Router::new()
        // Auth routes
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
        // Documentation routes
        .route("/docs", get(docs::docs_html))
        .route("/openapi.json", get(docs::openapi_json));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        // Auth - Get current user
        .route("/auth/me", get(get_me))
        // Medical Records
        .route("/medical-records", get(get_medical_records).post(create_medical_record))
        .route("/medical-records/:id", get(get_medical_record).put(update_medical_record).delete(delete_medical_record))
        // Doctors
        .route("/doctors", get(get_doctors).post(create_doctor))
        // Nurses
        .route("/nurses", get(get_nurses))
        // Medicines
        .route("/medicines", get(get_medicines))
        // Appointments
        .route("/appointments", get(get_appointments).post(create_appointment))
        // Services
        .route("/services", get(get_services))
        // Insurances
        .route("/insurances", get(get_insurances))
        // Files
        .route("/files", get(get_files).post(create_file))
        .route("/files/:id", get(get_file).delete(delete_file))
        // Apply auth middleware to all protected routes
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // Combine public and protected routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(cors)
}
