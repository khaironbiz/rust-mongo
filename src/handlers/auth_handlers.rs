use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    db::AppState,
    dto::{
        RegisterRequest, LoginRequest, ForgotPasswordRequest, ResetPasswordRequest,
        RefreshTokenRequest,
    },
    response::{ApiResponse, ErrorResponse},
    repository::UserRepository,
    services::AuthService,
};

/// Register a new user
/// 
/// POST /auth/register
/// 
/// Request body:
/// ```json
/// {
///     "email": "user@example.com",
///     "password": "password123",
///     "name": "John Doe"
/// }
/// ```
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let repo = UserRepository::new(state.db.clone());
    let service = AuthService::new(repo);
    
    match service.register(payload).await {
        Ok((status, response)) => ApiResponse::success(status, "User registered successfully", response).into_response(),
        Err((status, msg)) => {
            let error_code = match status.as_u16() {
                409 => "EMAIL_EXISTS",
                400 => "VALIDATION_ERROR",
                _ => "REGISTRATION_FAILED",
            };
            ErrorResponse::new(status, "Registration failed", error_code, Some(msg)).into_response()
        }
    }
}

/// Login user
/// 
/// POST /auth/login
/// 
/// Request body:
/// ```json
/// {
///     "email": "user@example.com",
///     "password": "password123"
/// }
/// ```
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let repo = UserRepository::new(state.db.clone());
    let service = AuthService::new(repo);
    
    match service.login(payload).await {
        Ok(response) => ApiResponse::ok("Login successful", response).into_response(),
        Err((status, msg)) => {
            let error_code = match status.as_u16() {
                401 => "INVALID_CREDENTIALS",
                _ => "LOGIN_FAILED",
            };
            ErrorResponse::new(status, "Login failed", error_code, Some(msg)).into_response()
        }
    }
}

/// Refresh access token using refresh token
///
/// POST /auth/refresh
///
/// Request body:
/// ```json
/// {
///     "refresh_token": "<refresh_token_here>"
/// }
/// ```
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    let repo = UserRepository::new(state.db.clone());
    let service = AuthService::new(repo);

    match service.refresh_token(payload).await {
        Ok(response) => ApiResponse::ok("Token refreshed successfully", response).into_response(),
        Err((status, msg)) => {
            let error_code = match status.as_u16() {
                401 => "INVALID_REFRESH_TOKEN",
                _ => "REFRESH_TOKEN_FAILED",
            };
            ErrorResponse::new(status, "Failed to refresh token", error_code, Some(msg)).into_response()
        }
    }
}

/// Request password reset
/// 
/// POST /auth/forgot-password
/// 
/// Request body:
/// ```json
/// {
///     "email": "user@example.com"
/// }
/// ```
pub async fn forgot_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> impl IntoResponse {
    let repo = UserRepository::new(state.db.clone());
    let service = AuthService::new(repo);
    
    match service.forgot_password(payload).await {
        Ok(response) => ApiResponse::ok("Password reset initiated", response).into_response(),
        Err((status, msg)) => {
            ErrorResponse::new(status, "Failed to initiate password reset", "FORGOT_PASSWORD_FAILED", Some(msg)).into_response()
        }
    }
}

/// Reset password using token
/// 
/// POST /auth/reset-password
/// 
/// Request body:
/// ```json
/// {
///     "token": "reset_token_here",
///     "password": "new_password123"
/// }
/// ```
pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> impl IntoResponse {
    let repo = UserRepository::new(state.db.clone());
    let service = AuthService::new(repo);
    
    match service.reset_password(payload).await {
        Ok(response) => ApiResponse::ok("Password reset successful", response).into_response(),
        Err((status, msg)) => {
            let error_code = match status.as_u16() {
                400 => "INVALID_TOKEN",
                _ => "RESET_PASSWORD_FAILED",
            };
            ErrorResponse::new(status, "Failed to reset password", error_code, Some(msg)).into_response()
        }
    }
}

/// Get current user info (protected route example)
/// 
/// GET /auth/me
/// 
/// Requires Authorization: Bearer <token>
pub async fn get_me(
    axum::Extension(user): axum::Extension<crate::middleware::AuthUser>,
) -> impl IntoResponse {
    ApiResponse::ok("User info retrieved", serde_json::json!({
        "id": user.id,
        "email": user.email,
        "name": user.name
    })).into_response()
}
