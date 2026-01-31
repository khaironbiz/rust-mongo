use axum::{
    extract::{Path, State, Query, Multipart},
    response::IntoResponse,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use crate::{
    db::AppState,
    services::FileService,
    repository::FileRepository,
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

pub async fn get_files(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = FileRepository::new(state.db.clone());
    let service = FileService::new(repo, state.s3_client.clone());
    
    match service.get_all_paginated(params.clone()).await {
        Ok((files, meta)) => PaginatedResponse::ok("Files retrieved successfully", files, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve files", Some(e)).into_response(),
    }
}

pub async fn get_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = FileRepository::new(state.db.clone());
    let service = FileService::new(repo, state.s3_client.clone());
    
    match service.get_by_id(oid).await {
        Ok(Some(file)) => ApiResponse::ok("File retrieved successfully", file).into_response(),
        Ok(None) => ErrorResponse::not_found("File not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve file", Some(e)).into_response(),
    }
}

pub async fn create_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_name = String::new();
    let mut file_bytes: Vec<u8> = Vec::new();
    let mut uploader = String::from("unknown");

    // Extract file and uploader from multipart form data
    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name: String = field.name().unwrap_or_default().to_string();
        
        match field_name.as_str() {
            "file" => {
                file_name = field.file_name().unwrap_or("unnamed").to_string();
                match field.bytes().await {
                    Ok(bytes) => {
                        file_bytes = bytes.to_vec();
                    },
                    Err(_) => return ErrorResponse::bad_request("Failed to read file content", None).into_response(),
                }
            },
            "uploader" => {
                match field.text().await {
                    Ok(text) => uploader = text,
                    Err(_) => {}
                }
            },
            _ => {}
        }
    }

    // Validate file was provided
    if file_name.is_empty() || file_bytes.is_empty() {
        return ErrorResponse::bad_request("No file provided", Some("Please upload a valid file".to_string())).into_response();
    }

    let repo = FileRepository::new(state.db.clone());
    let service = FileService::new(repo, state.s3_client.clone());
    
    match service.create(file_name, file_bytes, uploader).await {
        Ok((status, file)) => ApiResponse::success(status, "File uploaded successfully", file).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to upload file", "UPLOAD_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = FileRepository::new(state.db.clone());
    let service = FileService::new(repo, state.s3_client.clone());
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("File not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete file", "DELETE_FAILED", Some(msg)).into_response(),
    }
}
