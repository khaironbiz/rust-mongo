use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::{ChildCodeService},
    repository::{ChildCodeRepository, CodeRepository},
    dto::child_code::{CreateChildCodeRequest, UpdateChildCodeRequest},
    response::{ApiResponse, ErrorResponse, no_content},
};

pub async fn get_child_codes(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repo = Arc::new(ChildCodeRepository::new(state.db.clone()));
    let code_repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = ChildCodeService::new(repo, code_repo);
    
    match service.get_all().await {
        Ok(child_codes) => ApiResponse::ok("Child codes retrieved successfully", child_codes).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve child codes", Some(e)).into_response(),
    }
}

pub async fn create_child_code(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateChildCodeRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(ChildCodeRepository::new(state.db.clone()));
    let code_repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = ChildCodeService::new(repo, code_repo);
    
    match service.create(payload).await {
        Ok(child_code) => ApiResponse::success(axum::http::StatusCode::CREATED, "Child code created successfully", child_code).into_response(),
        Err(e) => ErrorResponse::bad_request("Failed to create child code", Some(e)).into_response(),
    }
}

pub async fn get_child_code(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = Arc::new(ChildCodeRepository::new(state.db.clone()));
    let code_repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = ChildCodeService::new(repo, code_repo);

    match service.get_by_id(oid).await {
        Ok(Some(child_code)) => ApiResponse::ok("Child code retrieved successfully", child_code).into_response(),
        Ok(None) => ErrorResponse::not_found("Child code not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve child code", Some(e)).into_response(),
    }
}

pub async fn update_child_code(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateChildCodeRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(ChildCodeRepository::new(state.db.clone()));
    let code_repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = ChildCodeService::new(repo, code_repo);
    
    match service.update(oid, payload).await {
        Ok(child_code) => ApiResponse::ok("Child code updated successfully", child_code).into_response(),
        Err(e) => ErrorResponse::bad_request("Failed to update child code", Some(e)).into_response(),
    }
}

pub async fn delete_child_code(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = Arc::new(ChildCodeRepository::new(state.db.clone()));
    let code_repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = ChildCodeService::new(repo, code_repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Child code not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to delete child code", Some(e)).into_response(),
    }
}
