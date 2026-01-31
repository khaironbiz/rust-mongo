use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::NurseService,
    repository::NurseRepository,
    dto::nurse::{CreateNurseRequest, UpdateNurseRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_nurses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = NurseRepository::new(state.db.clone());
    let service = NurseService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((nurses, meta)) => PaginatedResponse::ok("Nurses retrieved successfully", nurses, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve nurses", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_nurse(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateNurseRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = NurseRepository::new(state.db.clone());
    let service = NurseService::new(repo);
    
    match service.create(payload).await {
        Ok((status, nurse)) => ApiResponse::success(status, "Nurse created successfully", nurse).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create nurse", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_nurse(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = NurseRepository::new(state.db.clone());
    let service = NurseService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(nurse)) => ApiResponse::ok("Nurse retrieved successfully", nurse).into_response(),
        Ok(None) => ErrorResponse::not_found("Nurse not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve nurse", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn update_nurse(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateNurseRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }


    let repo = NurseRepository::new(state.db.clone());
    let service = NurseService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(nurse) => ApiResponse::ok("Nurse updated successfully", nurse).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update nurse", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_nurse(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = NurseRepository::new(state.db.clone());
    let service = NurseService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Nurse not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete nurse", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
