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
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
        // Documentation routes
        .route("/docs", get(docs::docs_html))
        .route("/openapi.json", get(docs::openapi_json));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        // Auth - Get current user
        .route("/auth/me", get(get_me))
        // Users
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        // Medical Records
        .route("/medical-records", get(get_medical_records).post(create_medical_record))
        .route("/medical-records/:id", get(get_medical_record).put(update_medical_record).delete(delete_medical_record))
        // Doctors
        .route("/doctors", get(get_doctors).post(create_doctor))
        .route("/doctors/:id", get(get_doctor).put(update_doctor).delete(delete_doctor))
        // Nurses
        .route("/nurses", get(get_nurses).post(create_nurse))
        .route("/nurses/:id", get(get_nurse).put(update_nurse).delete(delete_nurse))
        // Medicines
        .route("/medicines", get(get_medicines).post(create_medicine))
        .route("/medicines/:id", get(get_medicine).put(update_medicine).delete(delete_medicine))
        // Appointments
        .route("/appointments", get(get_appointments).post(create_appointment))
        .route("/appointments/:id", get(get_appointment).put(update_appointment).delete(delete_appointment))
        // Services
        .route("/services", get(get_services).post(create_service))
        .route("/services/:id", get(get_service).put(update_service).delete(delete_service))
        // Insurances
        .route("/insurances", get(get_insurances).post(create_insurance))
        .route("/insurances/:id", get(get_insurance).put(update_insurance).delete(delete_insurance))
        // Files
        .route("/files", get(get_files).post(create_file))
        .route("/files/:id", get(get_file).delete(delete_file))
        // Roles
        .route("/roles", get(get_roles).post(create_role))
        .route("/roles/:id", get(get_role).put(update_role).delete(delete_role))
        // User Roles
        .route("/user-roles", get(get_user_roles).post(create_user_role))
        .route("/user-roles/:id", get(get_user_role).put(update_user_role).delete(delete_user_role))
        // Codes
        .route("/codes", get(get_codes).post(create_code))
        .route("/codes/:id", get(get_code).put(update_code).delete(delete_code))
        // Apply auth middleware ONLY to these protected routes
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // Combine routes: public first, then protected
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(cors)
}
