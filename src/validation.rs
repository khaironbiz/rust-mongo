use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;

/// Standard validation error response
#[derive(Debug)]
pub struct ValidationError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message,
            "status": self.status.as_u16()
        }));
        (self.status, body).into_response()
    }
}

/// Validate NIK: must be exactly 16 digits
pub fn validate_nik(nik: &str) -> Result<(), ValidationError> {
    if nik.len() != 16 {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: "NIK must be exactly 16 characters".to_string(),
        });
    }

    if !nik.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: "NIK must contain only digits".to_string(),
        });
    }

    Ok(())
}

/// Validate NIP: must be numeric
pub fn validate_nip(nip: &str) -> Result<(), ValidationError> {
    if nip.is_empty() {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: "NIP cannot be empty".to_string(),
        });
    }

    if !nip.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: "NIP must contain only digits".to_string(),
        });
    }

    Ok(())
}

/// Validate email format (basic check)
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if !email.contains('@') || !email.contains('.') {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: "Invalid email format".to_string(),
        });
    }

    Ok(())
}

/// Validate required string field
pub fn validate_required(field_name: &str, value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: format!("{} cannot be empty", field_name),
        });
    }

    Ok(())
}

/// Validate phone number (basic: at least 10 digits)
pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 10 {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: "Phone number must contain at least 10 digits".to_string(),
        });
    }

    Ok(())
}

/// Validate file upload: size and allowed file types
pub fn validate_file_upload(filename: &str, file_size: u64) -> Result<(), ValidationError> {
    // Max 256KB = 256 * 1024 bytes
    const MAX_FILE_SIZE: u64 = 256 * 1024;
    
    if file_size == 0 {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: "File size cannot be empty".to_string(),
        });
    }

    if file_size > MAX_FILE_SIZE {
        return Err(ValidationError {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            message: format!("File size exceeds maximum of {} KB", MAX_FILE_SIZE / 1024),
        });
    }

    // Extract file extension
    let extension = filename
        .split('.')
        .last()
        .unwrap_or("")
        .to_lowercase();

    // Allowed extensions
    let allowed = [
        "pdf",
        "jpg", "jpeg", "png", "gif", "bmp", "webp",
        "xlsx", "xls",
        "csv",
    ];

    if !allowed.contains(&extension.as_str()) {
        return Err(ValidationError {
            status: StatusCode::BAD_REQUEST,
            message: "File type not allowed. Allowed: PDF, Images (JPG, PNG, GIF, BMP, WebP), Excel (XLSX, XLS), CSV".to_string(),
        });
    }

    Ok(())
}
