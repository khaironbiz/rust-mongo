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
        .route("/appointments", get(appointment_handlers::get_appointments).post(appointment_handlers::create_appointment))
        .route("/appointments/:id", get(appointment_handlers::get_appointment).put(appointment_handlers::update_appointment).delete(appointment_handlers::delete_appointment))
        // Services
        .route("/services", get(service_handlers::get_services).post(service_handlers::create_service))
        .route("/services/:id", get(service_handlers::get_service).put(service_handlers::update_service).delete(service_handlers::delete_service))
        // Insurances
        .route("/insurances", get(insurance_handlers::get_insurances).post(insurance_handlers::create_insurance))
        .route("/insurances/:id", get(insurance_handlers::get_insurance).put(insurance_handlers::update_insurance).delete(insurance_handlers::delete_insurance))
        // File
        .nest("/files", Router::new()
            .route("/", get(file_handlers::get_files).post(file_handlers::create_file))
            .route("/:id", get(file_handlers::get_file).delete(file_handlers::delete_file))
        )
        // Create Child Codes
        .nest("/child-codes", Router::new()
            .route("/", get(child_code_handlers::get_child_codes).post(child_code_handlers::create_child_code))
            .route("/:id", get(child_code_handlers::get_child_code).put(child_code_handlers::update_child_code).delete(child_code_handlers::delete_child_code))
        )
        // Regions
        .nest("/regions", Router::new()
            .route("/", get(region_handlers::get_regions).post(region_handlers::create_region))
            .route("/:id", get(region_handlers::get_region).put(region_handlers::update_region).delete(region_handlers::delete_region))
        )
        // Roles
        .route("/roles", get(role_handlers::get_roles).post(role_handlers::create_role))
        .route("/roles/:id", get(role_handlers::get_role).put(role_handlers::update_role).delete(role_handlers::delete_role))
        // User Roles
        .route("/user-roles", get(user_role_handlers::get_user_roles).post(user_role_handlers::create_user_role))
        .route("/user-roles/:id", get(user_role_handlers::get_user_role).put(user_role_handlers::update_user_role).delete(user_role_handlers::delete_user_role))
        // Codes
        .route("/codes", get(code_handlers::get_codes).post(code_handlers::create_code))
        .route("/codes/:id", get(code_handlers::get_code).put(code_handlers::update_code).delete(code_handlers::delete_code))
        // Child Codes
        .route("/child-codes", get(child_code_handlers::get_child_codes).post(child_code_handlers::create_child_code))
        .route("/child-codes/:id", get(child_code_handlers::get_child_code).put(child_code_handlers::update_child_code).delete(child_code_handlers::delete_child_code))
        // Apply auth middleware ONLY to these protected routes
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // Combine routes: public first, then protected
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(cors)
}
