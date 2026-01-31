use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::KitService,
    repository::KitRepository,
    dto::kit::{CreateKitRequest, UpdateKitRequest},
    response::{ApiResponse, ErrorResponse, no_content},
};

pub async fn get_kits(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let repo = Arc::new(KitRepository::new(state.db.clone()));
    let service = KitService::new(repo);
    
    match service.get_all().await {
        Ok(kits) => ApiResponse::ok("Kits retrieved successfully", kits).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve kits", Some(e)).into_response(),
    }
}

pub async fn create_kit(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateKitRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(KitRepository::new(state.db.clone()));
    let service = KitService::new(repo);
    
    match service.create(payload).await {
        Ok(kit) => ApiResponse::success(axum::http::StatusCode::CREATED, "Kit created successfully", kit).into_response(),
        Err(e) => ErrorResponse::bad_request("Failed to create kit", Some(e)).into_response(),
    }
}

pub async fn get_kit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = Arc::new(KitRepository::new(state.db.clone()));
    let service = KitService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(kit)) => ApiResponse::ok("Kit retrieved successfully", kit).into_response(),
        Ok(None) => ErrorResponse::not_found("Kit not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve kit", Some(e)).into_response(),
    }
}

pub async fn update_kit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateKitRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(KitRepository::new(state.db.clone()));
    let service = KitService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(kit) => ApiResponse::ok("Kit updated successfully", kit).into_response(),
        Err(e) => ErrorResponse::bad_request("Failed to update kit", Some(e)).into_response(),
    }
}

pub async fn delete_kit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = Arc::new(KitRepository::new(state.db.clone()));
    let service = KitService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Kit not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to delete kit", Some(e)).into_response(),
    }
}
