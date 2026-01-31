pub mod auth_handlers;
pub mod user_handlers;

use axum::{
    extract::{Path, State, Multipart, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::{
    db::AppState,
    models::*,
    services::*,
    repository::*,
    response::{ApiResponse, ErrorResponse, no_content, PaginatedResponse},
    pagination::PaginationParams,
};

// Re-export auth handlers
pub use auth_handlers::{register, login, refresh_token, forgot_password, reset_password, get_me};
pub use user_handlers::{get_users, get_user, create_user, update_user, delete_user};

// --- Medical Records ---

pub async fn get_medical_records(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((records, meta)) => PaginatedResponse::ok("Medical records retrieved successfully", records, meta).into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve medical records", Some(e)).into_response(),
    }
}

pub async fn get_medical_record(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.get_by_id(oid).await {
        Ok(Some(record)) => ApiResponse::ok("Medical record retrieved successfully", record).into_response(),
        Ok(None) => ErrorResponse::not_found("Medical record not found").into_response(),
        Err(e) => ErrorResponse::internal_error("Failed to retrieve medical record", Some(e)).into_response(),
    }
}

pub async fn create_medical_record(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<MedicalRecord>,
) -> impl IntoResponse {
    if payload.id.is_none() {
        payload.id = Some(ObjectId::new());
    }

    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.create(payload).await {
        Ok((status, record)) => ApiResponse::success(status, "Medical record created successfully", record).into_response(),
        Err((status, msg)) => {
            let error_code = match status {
                StatusCode::CONFLICT => "DUPLICATE_NIK",
                StatusCode::UNPROCESSABLE_ENTITY => "VALIDATION_ERROR",
                _ => "VALIDATION_FAILED",
            };
            ErrorResponse::new(status, "Failed to create medical record", error_code, Some(msg)).into_response()
        }
    }
}

pub async fn update_medical_record(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut payload): Json<MedicalRecord>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    if payload.id.is_none() {
        payload.id = Some(oid);
    }

    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.update(oid, payload).await {
        Ok(record) => ApiResponse::ok("Medical record updated successfully", record).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to update medical record", "UPDATE_FAILED", Some(msg)).into_response(),
    }
}

pub async fn delete_medical_record(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return ErrorResponse::bad_request("Invalid ID format", Some("ID must be a valid MongoDB ObjectId".to_string())).into_response();
    };

    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.delete(oid).await {
        Ok(true) => no_content().into_response(),
        Ok(false) => ErrorResponse::not_found("Medical record not found").into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to delete medical record", "DELETE_FAILED", Some(msg)).into_response(),
    }
}

// --- Doctors ---

pub async fn get_doctors(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((doctors, meta)) => PaginatedResponse::ok("Doctors retrieved successfully", doctors, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve doctors", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_doctor(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<Doctor>,
) -> impl IntoResponse {
    if payload.id.is_none() {
        payload.id = Some(ObjectId::new());
    }

    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    match service.create(payload).await {
        Ok((status, doctor)) => ApiResponse::success(status, "Doctor created successfully", doctor).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create doctor", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

// --- Nurses ---

pub async fn get_nurses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = NurseRepository::new(state.db.clone());
    let service = NurseService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((nurses, meta)) => PaginatedResponse::ok("Nurses retrieved successfully", nurses, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve nurses", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

// --- Medicines ---

pub async fn get_medicines(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = MedicineRepository::new(state.db.clone());
    let service = MedicineService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((medicines, meta)) => PaginatedResponse::ok("Medicines retrieved successfully", medicines, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve medicines", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

// --- Appointments ---

pub async fn get_appointments(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = AppointmentRepository::new(state.db.clone());
    let service = AppointmentService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((appointments, meta)) => PaginatedResponse::ok("Appointments retrieved successfully", appointments, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve appointments", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

pub async fn create_appointment(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<Appointment>,
) -> impl IntoResponse {
    if payload.id.is_none() {
        payload.id = Some(ObjectId::new());
    }

    let repo = AppointmentRepository::new(state.db.clone());
    let service = AppointmentService::new(repo);
    
    match service.create(payload).await {
        Ok((status, appointment)) => ApiResponse::success(status, "Appointment created successfully", appointment).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to create appointment", "CREATE_FAILED", Some(msg)).into_response(),
    }
}

// --- Services (Healthcare Services) ---

pub async fn get_services(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = ServiceRepository::new(state.db.clone());
    let service = ServiceService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((services, meta)) => PaginatedResponse::ok("Services retrieved successfully", services, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve services", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

// --- Insurances ---

pub async fn get_insurances(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let repo = InsuranceRepository::new(state.db.clone());
    let service = InsuranceService::new(repo);
    
    match service.get_all_paginated(params.clone()).await {
        Ok((insurances, meta)) => PaginatedResponse::ok("Insurances retrieved successfully", insurances, meta).into_response(),
        Err((status, msg)) => ErrorResponse::new(status, "Failed to retrieve insurances", "FETCH_FAILED", Some(msg)).into_response(),
    }
}

// --- Files ---

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
