use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::RoleService,
    repository::RoleRepository,
    dto::role::{CreateRoleRequest, UpdateRoleRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_roles(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = RoleRepository::new(state.db.clone());
    let service = RoleService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((roles, meta)) => PaginatedResponse::ok("Roles retrieved successfully", roles, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve roles", Some(e)).into_response(),
    }
}

pub async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRoleRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = RoleRepository::new(state.db.clone());
    let service = RoleService::new(repo);
    
    match service.create(payload).await {
        Ok((status, role)) => ApiResponse::success(status, "Role created successfully", role).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create role", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = RoleRepository::new(state.db.clone());
    let service = RoleService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(role)) => ApiResponse::ok("Role retrieved successfully", role).into_response(),
        Ok(None) => ErrorResponse::not_found("Role not found").into_response(),
        Err(msg) => ErrorResponse::internal_error("Failed to retrieve role", Some(msg)).into_response(),
    }
}

pub async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateRoleRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = RoleRepository::new(state.db.clone());
    let service = RoleService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(role) => ApiResponse::ok("Role updated successfully", role).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update role", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = RoleRepository::new(state.db.clone());
    let service = RoleService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Role not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete role", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
