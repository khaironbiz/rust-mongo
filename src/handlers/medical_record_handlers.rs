use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::MedicalRecordService,
    repository::MedicalRecordRepository,
    dto::medical_record::{CreateMedicalRecordRequest, UpdateMedicalRecordRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};
use axum::http::StatusCode;

pub async fn get_medical_records(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((records, meta)) => PaginatedResponse::ok("Medical records retrieved successfully", records, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve medical records", Some(e)).into_response(),
    }
}

pub async fn get_medical_record(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.get_by_id(oid).await {
        Ok(Some(record)) => ApiResponse::ok("Medical record retrieved successfully", record).into_response(),
        Ok(None) => ErrorResponse::not_found("Medical record not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve medical record", Some(e)).into_response(),
    }
}

pub async fn create_medical_record(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMedicalRecordRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.create(payload).await {
        Ok((status, record)) => ApiResponse::success(status, "Medical record created successfully", record).into_response(),
        Err((status, msg)) => {
            let error_code = match status {
                StatusCode::CONFLICT => "DUPLICATE_NIK",
                StatusCode::UNPROCESSABLE_ENTITY => "VALIDATION_ERROR",
                _ => "VALIDATION_FAILED",
            };
            ErrorResponse::new(status, "Failed to create medical record", error_code, Some(msg)).into_response()
        }
    }
}

pub async fn update_medical_record(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateMedicalRecordRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }


    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(record) => ApiResponse::ok("Medical record updated successfully", record).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update medical record", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_medical_record(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Medical record not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete medical record", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
