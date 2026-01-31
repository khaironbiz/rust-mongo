use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::MedicineService,
    repository::MedicineRepository,
    dto::medicine::{CreateMedicineRequest, UpdateMedicineRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_medicines(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = MedicineRepository::new(state.db.clone());
    let service = MedicineService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((medicines, meta)) => PaginatedResponse::ok("Medicines retrieved successfully", medicines, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve medicines", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_medicine(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMedicineRequest>,
) -> impl IntoResponse {
    let repo = MedicineRepository::new(state.db.clone());
    let service = MedicineService::new(repo);
    
    match service.create(payload).await {
        Ok((status, medicine)) => ApiResponse::success(status, "Medicine created successfully", medicine).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create medicine", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_medicine(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = MedicineRepository::new(state.db.clone());
    let service = MedicineService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(medicine)) => ApiResponse::ok("Medicine retrieved successfully", medicine).into_response(),
        Ok(None) => ErrorResponse::not_found("Medicine not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve medicine", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn update_medicine(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateMedicineRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = MedicineRepository::new(state.db.clone());
    let service = MedicineService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(medicine) => ApiResponse::ok("Medicine updated successfully", medicine).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update medicine", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_medicine(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = MedicineRepository::new(state.db.clone());
    let service = MedicineService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Medicine not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete medicine", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
