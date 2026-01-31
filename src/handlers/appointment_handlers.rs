use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::AppointmentService,
    repository::AppointmentRepository,
    dto::appointment::{CreateAppointmentRequest, UpdateAppointmentRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_appointments(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = AppointmentRepository::new(state.db.clone());
    let service = AppointmentService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((appointments, meta)) => PaginatedResponse::ok("Appointments retrieved successfully", appointments, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve appointments", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_appointment(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAppointmentRequest>,
) -> impl IntoResponse {
    let repo = AppointmentRepository::new(state.db.clone());
    let service = AppointmentService::new(repo);
    
    match service.create(payload).await {
        Ok((status, appointment)) => ApiResponse::success(status, "Appointment created successfully", appointment).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create appointment", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_appointment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = AppointmentRepository::new(state.db.clone());
    let service = AppointmentService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(appointment)) => ApiResponse::ok("Appointment retrieved successfully", appointment).into_response(),
        Ok(None) => ErrorResponse::not_found("Appointment not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve appointment", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn update_appointment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAppointmentRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = AppointmentRepository::new(state.db.clone());
    let service = AppointmentService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(appointment) => ApiResponse::ok("Appointment updated successfully", appointment).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update appointment", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_appointment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = AppointmentRepository::new(state.db.clone());
    let service = AppointmentService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Appointment not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete appointment", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
