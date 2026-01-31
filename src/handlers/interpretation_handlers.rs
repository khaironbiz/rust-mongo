use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::InterpretationService,
    repository::InterpretationRepository,
    dto::interpretation::{CreateInterpretationRequest, UpdateInterpretationRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_interpretations(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = Arc::new(InterpretationRepository::new(state.db.clone()));
    let service = InterpretationService::new(repo);
    
    match service.get_all_paginated(params).await {
        Ok((interpretations, meta)) => PaginatedResponse::ok("Interpretations retrieved successfully", interpretations, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve interpretations", Some(e)).into_response(),
    }
}

pub async fn get_interpretation_by_code(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = Arc::new(InterpretationRepository::new(state.db.clone()));
    let service = InterpretationService::new(repo);
    
    match service.get_by_code_paginated(&code, params).await {
        Ok((interpretations, meta)) => PaginatedResponse::ok("Interpretations retrieved successfully", interpretations, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve interpretations", Some(e)).into_response(),
    }
}

pub async fn get_interpretations_by_coding_code(
    State(state): State<Arc<AppState>>,
    Path(coding_code): Path<String>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = Arc::new(InterpretationRepository::new(state.db.clone()));
    let service = InterpretationService::new(repo);
    
    match service.get_by_coding_code_paginated(&coding_code, params).await {
        Ok((interpretations, meta)) => PaginatedResponse::ok("Interpretations retrieved successfully", interpretations, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve interpretations", Some(e)).into_response(),
    }
}

pub async fn get_interpretation_by_code_and_coding_code(
    State(state): State<Arc<AppState>>,
    Path((code, coding_code)): Path<(String, String)>,
) -> impl IntoResponse {
    let repo = Arc::new(InterpretationRepository::new(state.db.clone()));
    let service = InterpretationService::new(repo);
    
    match service.get_by_code_and_coding_code(&code, &coding_code).await {
        Ok(Some(interpretation)) => ApiResponse::ok("Interpretation retrieved successfully", interpretation).into_response(),
        Ok(None) => ErrorResponse::not_found("Interpretation not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve interpretation", Some(e)).into_response(),
    }
}

pub async fn create_interpretation(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateInterpretationRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(InterpretationRepository::new(state.db.clone()));
    let service = InterpretationService::new(repo);
    
    match service.create(payload).await {
        Ok(interpretation) => ApiResponse::success(axum::http::StatusCode::CREATED, "Interpretation created successfully", interpretation).into_response(),
        Err(e) => ErrorResponse::bad_request("Failed to create interpretation", Some(e)).into_response(),
    }
}

pub async fn get_interpretation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = Arc::new(InterpretationRepository::new(state.db.clone()));
    let service = InterpretationService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(interpretation)) => ApiResponse::ok("Interpretation retrieved successfully", interpretation).into_response(),
        Ok(None) => ErrorResponse::not_found("Interpretation not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve interpretation", Some(e)).into_response(),
    }
}

pub async fn update_interpretation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateInterpretationRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(InterpretationRepository::new(state.db.clone()));
    let service = InterpretationService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(interpretation) => ApiResponse::ok("Interpretation updated successfully", interpretation).into_response(),
        Err(e) => ErrorResponse::bad_request("Failed to update interpretation", Some(e)).into_response(),
    }
}

pub async fn delete_interpretation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = Arc::new(InterpretationRepository::new(state.db.clone()));
    let service = InterpretationService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Interpretation not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to delete interpretation", Some(e)).into_response(),
    }
}
