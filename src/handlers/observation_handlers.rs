use axum::{
    extract::{Path, State, Query},
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::ObservationService,
    repository::ObservationRepository,
    dto::observation::{CreateObservationRequest, UpdateObservationRequest},
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_observations(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = ObservationRepository::new(state.db.clone());
    let service = ObservationService::new(repo);
    
    match service.get_observations(params.clone()).await {
        Ok((observations, total)) => {
            let meta = crate::pagination::PaginationMeta::new(
                params.page,
                params.limit,
                total
            );
            PaginatedResponse::ok("Observations retrieved successfully", observations, meta).into_response()
        },
        Err(e) => ErrorResponse::internal_error("Failed to retrieve observations", Some(e)).into_response(),
    }
}

pub async fn create_observation(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateObservationRequest>,
) -> impl IntoResponse {
    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = ObservationRepository::new(state.db.clone());
    let service = ObservationService::new(repo);
    
    match service.create_observation(payload).await {
        Ok(observation) => ApiResponse::success(axum::http::StatusCode::CREATED, "Observation created successfully", observation).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to create observation", Some(e)).into_response(),
    }
}

pub async fn get_observation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(_oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = ObservationRepository::new(state.db.clone());
    let service = ObservationService::new(repo);

    match service.get_observation_by_id(&id).await {
        Ok(Some(observation)) => ApiResponse::ok("Observation retrieved successfully", observation).into_response(),
        Ok(None) => ErrorResponse::not_found("Observation not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve observation", Some(e)).into_response(),
    }
}

pub async fn update_observation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateObservationRequest>,
) -> impl IntoResponse {
    let Ok(_oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if let Err(e) = crate::validation::validate_payload(&payload) {
        return e.into_response();
    }

    let repo = ObservationRepository::new(state.db.clone());
    let service = ObservationService::new(repo);
    
    match service.update_observation(&id, payload).await {
        Ok(observation) => ApiResponse::ok("Observation updated successfully", observation).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to update observation", Some(e)).into_response(),
    }
}

pub async fn delete_observation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(_oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = ObservationRepository::new(state.db.clone());
    let service = ObservationService::new(repo);
    
    match service.delete_observation(&id).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Observation not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to delete observation", Some(e)).into_response(),
    }
}
