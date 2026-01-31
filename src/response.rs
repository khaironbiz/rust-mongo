use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// Standard API Response Structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    /// Success status
    pub success: bool,
    
    /// HTTP status code
    pub status: u16,
    
    /// Response message
    pub message: String,
    
    /// Response data (can be null for error responses)
    pub data: Option<T>,
    
    /// Error details (only present in error responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    
    /// Timestamp of the response
    pub timestamp: String,
}

/// Error details in response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorDetails {
    pub code: String,
    pub details: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    /// Create a successful response
    pub fn success(status: StatusCode, message: impl Into<String>, data: T) -> Self {
        ApiResponse {
            success: true,
            status: status.as_u16(),
            message: message.into(),
            data: Some(data),
            error: None,
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    /// Create a response with 200 OK
    pub fn ok(message: impl Into<String>, data: T) -> Self {
        Self::success(StatusCode::OK, message, data)
    }

    /// Create a response with 201 Created
    pub fn created(message: impl Into<String>, data: T) -> Self {
        Self::success(StatusCode::CREATED, message, data)
    }
}

/// Create a 204 No Content response
pub fn no_content() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NO_CONTENT,
        Json(serde_json::json!({
            "success": true,
            "status": 204,
            "message": "Resource deleted successfully",
            "data": serde_json::Value::Null,
            "timestamp": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }))
    )
}
/// Paginated API Response Structure
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub status: u16,
    pub message: String,
    pub data: Vec<T>,
    pub pagination: crate::pagination::PaginationMeta,
    pub timestamp: String,
}

impl<T: Serialize> PaginatedResponse<T> {
    /// Create a paginated response
    pub fn ok(message: impl Into<String>, data: Vec<T>, pagination: crate::pagination::PaginationMeta) -> Self {
        PaginatedResponse {
            success: true,
            status: StatusCode::OK.as_u16(),
            message: message.into(),
            data,
            pagination,
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

impl<T: Serialize> IntoResponse for PaginatedResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Error response (no generic data)
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub status: u16,
    pub message: String,
    pub error: ErrorDetails,
    pub timestamp: String,
}

impl ErrorResponse {
    /// Create an error response
    pub fn new(
        status: StatusCode,
        message: impl Into<String>,
        error_code: impl Into<String>,
        details: Option<String>,
    ) -> Self {
        ErrorResponse {
            success: false,
            status: status.as_u16(),
            message: message.into(),
            error: ErrorDetails {
                code: error_code.into(),
                details,
            },
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    /// Bad Request (400)
    pub fn bad_request(message: impl Into<String>, details: Option<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message, "BAD_REQUEST", details)
    }

    /// Unauthorized (401)
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message, "UNAUTHORIZED", None)
    }

    /// Forbidden (403)
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, message, "FORBIDDEN", None)
    }

    /// Not Found (404)
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message, "NOT_FOUND", None)
    }

    /// Conflict (409)
    pub fn conflict(message: impl Into<String>, details: Option<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message, "CONFLICT", details)
    }

    /// Validation Error (422)
    pub fn validation_error(message: impl Into<String>, details: Option<String>) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            message,
            "VALIDATION_ERROR",
            details,
        )
    }

    /// Internal Server Error (500)
    pub fn internal_error(message: impl Into<String>, details: Option<String>) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            message,
            "INTERNAL_ERROR",
            details,
        )
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

/// Macro for quick success responses
#[macro_export]
macro_rules! api_success {
    ($data:expr) => {
        $crate::response::ApiResponse::ok("Success", $data)
    };
    ($message:expr, $data:expr) => {
        $crate::response::ApiResponse::ok($message, $data)
    };
}

/// Macro for quick error responses
#[macro_export]
macro_rules! api_error {
    ($status:expr, $message:expr) => {
        $crate::response::ErrorResponse::new($status, $message, "ERROR", None)
    };
    ($status:expr, $message:expr, $code:expr) => {
        $crate::response::ErrorResponse::new($status, $message, $code, None)
    };
    ($status:expr, $message:expr, $code:expr, $details:expr) => {
        $crate::response::ErrorResponse::new($status, $message, $code, Some($details))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = ApiResponse::ok("Test message", serde_json::json!({"key": "value"}));
        assert!(response.success);
        assert_eq!(response.status, 200);
        assert_eq!(response.message, "Test message");
    }

    #[test]
    fn test_error_response() {
        let error = ErrorResponse::not_found("Resource not found");
        assert!(!error.success);
        assert_eq!(error.status, 404);
        assert_eq!(error.error.code, "NOT_FOUND");
    }

    #[test]
    fn test_created_response() {
        let response = ApiResponse::created("Created", serde_json::json!({"id": 1}));
        assert!(response.success);
        assert_eq!(response.status, 201);
    }
}
