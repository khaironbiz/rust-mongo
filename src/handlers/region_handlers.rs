use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::RegionService,
    repository::RegionRepository,
    dto::region::{CreateRegionRequest, UpdateRegionRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_regions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);
    
    match service.get_all_paginated(params).await {
        Ok((regions, meta)) => PaginatedResponse::ok("Regions retrieved successfully", regions, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve regions", Some(e)).into_response(),
    }
}

pub async fn create_region(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRegionRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);
    
    match service.create(payload).await {
        Ok(region) => ApiResponse::success(axum::http::StatusCode::CREATED, "Region created successfully", region).into_response(),
        Err(e) => ErrorResponse::bad_request("Failed to create region", Some(e)).into_response(),
    }
}

pub async fn get_region(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);

    match service.get_by_id(oid).await {
        Ok(Some(region)) => ApiResponse::ok("Region retrieved successfully", region).into_response(),
        Ok(None) => ErrorResponse::not_found("Region not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve region", Some(e)).into_response(),
    }
}

pub async fn update_region(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateRegionRequest>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(region) => ApiResponse::ok("Region updated successfully", region).into_response(),
        Err(e) => ErrorResponse::bad_request("Failed to update region", Some(e)).into_response(),
    }
}

pub async fn delete_region(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Region not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to delete region", Some(e)).into_response(),
    }
}

pub async fn get_regions_by_provinsi(
    State(state): State<Arc<AppState>>,
    Path(provinsi): Path<String>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);
    
    match service.get_by_provinsi_paginated(&provinsi, params).await {
        Ok((regions, meta)) => PaginatedResponse::ok("Regions retrieved successfully", regions, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve regions", Some(e)).into_response(),
    }
}

pub async fn get_regions_by_kota(
    State(state): State<Arc<AppState>>,
    Path(kota): Path<String>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);
    
    match service.get_by_kota_paginated(&kota, params).await {
        Ok((regions, meta)) => PaginatedResponse::ok("Regions retrieved successfully", regions, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve regions", Some(e)).into_response(),
    }
}

pub async fn get_regions_by_kecamatan(
    State(state): State<Arc<AppState>>,
    Path(kecamatan): Path<String>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);
    
    match service.get_by_kecamatan_paginated(&kecamatan, params).await {
        Ok((regions, meta)) => PaginatedResponse::ok("Regions retrieved successfully", regions, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve regions", Some(e)).into_response(),
    }
}

pub async fn get_region_by_code(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let repo = Arc::new(RegionRepository::new(state.db.clone()));
    let service = RegionService::new(repo);
    
    match service.get_by_code(&code).await {
        Ok(Some(region)) => ApiResponse::ok("Region retrieved successfully", region).into_response(),
        Ok(None) => ErrorResponse::not_found("Region not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve region", Some(e)).into_response(),
    }
}
