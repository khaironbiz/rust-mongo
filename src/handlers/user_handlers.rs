use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::{
    db::AppState,
    dto::{RegisterRequest, UpdateUserRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    repository::UserRepository,
    services::UserService,
    pagination::PaginationParams,
};

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = UserRepository::new(state.db.clone());
    let service = UserService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((users, meta)) => PaginatedResponse::ok("Users retrieved successfully", users, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve users", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = UserRepository::new(state.db.clone());
    let service = UserService::new(repo);
    
    match service.get_by_id(oid).await {
        Ok(Some(user)) => ApiResponse::ok("User retrieved successfully", user).into_response(),
        Ok(None) => ErrorResponse::not_found("User not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve user", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let repo = UserRepository::new(state.db.clone());
    let service = UserService::new(repo);
    
    match service.create(payload).await {
        Ok((status, user)) => ApiResponse::success(status, "User created successfully", user).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create user", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = UserRepository::new(state.db.clone());
    let service = UserService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(user) => ApiResponse::ok("User updated successfully", user).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update user", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = UserRepository::new(state.db.clone());
    let service = UserService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("User not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete user", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
