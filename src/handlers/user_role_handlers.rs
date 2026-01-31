use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::UserRoleService,
    repository::UserRoleRepository,
    dto::user_role::{CreateUserRoleRequest, UpdateUserRoleRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_user_roles(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = UserRoleRepository::new(state.db.clone());
    let service = UserRoleService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((roles, meta)) => PaginatedResponse::ok("User roles retrieved successfully", roles, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve user roles", Some(e)).into_response(),
    }
}

pub async fn create_user_role(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRoleRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = UserRoleRepository::new(state.db.clone());
    let service = UserRoleService::new(repo);
    
    match service.create(payload).await {
        Ok((status, role)) => ApiResponse::success(status, "User role created successfully", role).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create user role", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_user_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = UserRoleRepository::new(state.db.clone());
    let service = UserRoleService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(role)) => ApiResponse::ok("User role retrieved successfully", role).into_response(),
        Ok(None) => ErrorResponse::not_found("User role not found").into_response(),
        Err(msg) => ErrorResponse::internal_error("Failed to retrieve user role", Some(msg)).into_response(),
    }
}

pub async fn update_user_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRoleRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = UserRoleRepository::new(state.db.clone());
    let service = UserRoleService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(role) => ApiResponse::ok("User role updated successfully", role).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update user role", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_user_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = UserRoleRepository::new(state.db.clone());
    let service = UserRoleService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("User role not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete user role", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
