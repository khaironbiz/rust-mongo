use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::DoctorService,
    repository::DoctorRepository,
    dto::doctor::{CreateDoctorRequest, UpdateDoctorRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_doctors(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((doctors, meta)) => PaginatedResponse::ok("Doctors retrieved successfully", doctors, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve doctors", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_doctor(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateDoctorRequest>,
) -> impl IntoResponse {
    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    match service.create(payload).await {
        Ok((status, doctor)) => ApiResponse::success(status, "Doctor created successfully", doctor).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create doctor", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_doctor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(doctor)) => ApiResponse::ok("Doctor retrieved successfully", doctor).into_response(),
        Ok(None) => ErrorResponse::not_found("Doctor not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve doctor", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn update_doctor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateDoctorRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(doctor) => ApiResponse::ok("Doctor updated successfully", doctor).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update doctor", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_doctor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Doctor not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete doctor", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
