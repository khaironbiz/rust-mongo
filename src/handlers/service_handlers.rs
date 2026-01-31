use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::ServiceService,
    repository::ServiceRepository,
    dto::service::{CreateServiceRequest, UpdateServiceRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_services(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = ServiceRepository::new(state.db.clone());
    let service = ServiceService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((services, meta)) => PaginatedResponse::ok("Services retrieved successfully", services, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve services", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_service(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateServiceRequest>,
) -> impl IntoResponse {
    let repo = ServiceRepository::new(state.db.clone());
    let service = ServiceService::new(repo);
    
    match service.create(payload).await {
        Ok((status, service)) => ApiResponse::success(status, "Service created successfully", service).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create service", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_service(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = ServiceRepository::new(state.db.clone());
    let service = ServiceService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(service)) => ApiResponse::ok("Service retrieved successfully", service).into_response(),
        Ok(None) => ErrorResponse::not_found("Service not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve service", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn update_service(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateServiceRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = ServiceRepository::new(state.db.clone());
    let service = ServiceService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(service) => ApiResponse::ok("Service updated successfully", service).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update service", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_service(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = ServiceRepository::new(state.db.clone());
    let service = ServiceService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Service not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete service", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
