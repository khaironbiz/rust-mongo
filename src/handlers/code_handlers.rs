use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use axum::http::StatusCode;

use crate::{
    db::AppState,
    dto::code::{CreateCodeDto, UpdateCodeDto},
    response::{ApiResponse, ErrorResponse, no_content},
    repository::CodeRepository,
    services::CodeService,
};

pub async fn get_codes(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = CodeService::new(repo);
    
    match service.get_all_codes().await {
        Ok(codes) => ApiResponse::ok("Codes retrieved successfully", codes).into_response(),
        Err(msg) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve codes", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn get_code(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = CodeService::new(repo);
    
    match service.get_code_by_id(&id).await {
        Ok(Some(code)) => ApiResponse::ok("Code retrieved successfully", code).into_response(),
        Ok(None) => ErrorResponse::not_found("Code not found").into_response(),
        Err(msg) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve code", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_code(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCodeDto>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = CodeService::new(repo);
    
    match service.create_code(payload).await {
        Ok(code) => ApiResponse::success(StatusCode::CREATED, "Code created successfully", code).into_response(),
        Err(msg) => {
            if msg.contains("exists") {
                ErrorResponse::new(StatusCode::CONFLICT, "Code already exists", "DUPLICATE_CODE", Some(msg)).into_response()
            } else {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create code", "CREATE_FAILED", Some(msg)).into_response()
            }
        },
    }
}

pub async fn update_code(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateCodeDto>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = CodeService::new(repo);
    
    match service.update_code(&id, payload).await {
        Ok(code) => ApiResponse::ok("Code updated successfully", code).into_response(),
        Err(msg) => {
            if msg.contains("not found") {
                ErrorResponse::not_found("Code not found").into_response()
            } else if msg.contains("exists") {
                ErrorResponse::new(StatusCode::CONFLICT, "Code already exists", "DUPLICATE_CODE", Some(msg)).into_response()
            } else {
                 ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to update code", "UPDATE_FAILED", Some(msg)).into_response()
            }
        },
    }
}

pub async fn delete_code(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let repo = Arc::new(CodeRepository::new(state.db.clone()));
    let service = CodeService::new(repo);
    
    match service.delete_code(&id).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Code not found").into_response(),
        Err(msg) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete code", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
