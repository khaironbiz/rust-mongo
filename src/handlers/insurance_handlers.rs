use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::InsuranceService,
    repository::InsuranceRepository,
    dto::insurance::{CreateInsuranceRequest, UpdateInsuranceRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_insurances(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = InsuranceRepository::new(state.db.clone());
    let service = InsuranceService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((insurances, meta)) => PaginatedResponse::ok("Insurances retrieved successfully", insurances, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve insurances", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_insurance(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateInsuranceRequest>,
) -> impl IntoResponse {
    let repo = InsuranceRepository::new(state.db.clone());
    let service = InsuranceService::new(repo);
    
    match service.create(payload).await {
        Ok((status, insurance)) => ApiResponse::success(status, "Insurance created successfully", insurance).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create insurance", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_insurance(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = InsuranceRepository::new(state.db.clone());
    let service = InsuranceService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(insurance)) => ApiResponse::ok("Insurance retrieved successfully", insurance).into_response(),
        Ok(None) => ErrorResponse::not_found("Insurance not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve insurance", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn update_insurance(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateInsuranceRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = InsuranceRepository::new(state.db.clone());
    let service = InsuranceService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(insurance) => ApiResponse::ok("Insurance updated successfully", insurance).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update insurance", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_insurance(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = InsuranceRepository::new(state.db.clone());
    let service = InsuranceService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Insurance not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete insurance", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
