use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

use crate::db::AppState;
use crate::response::ErrorResponse;
use crate::services::AuthService;

/// Extension to hold authenticated user claims
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
}

/// JWT Authentication Middleware
/// 
/// This middleware extracts and validates the JWT token from the Authorization header.
/// If valid, it adds the user claims to the request extensions for use in handlers.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Get the Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return ErrorResponse::unauthorized("Missing or invalid Authorization header").into_response();
        }
    };

    // Validate the token
    match AuthService::validate_token(token) {
        Ok(claims) => {
            // Add user info to request extensions
            let auth_user = AuthUser {
                id: claims.sub,
                email: claims.email,
                name: claims.name,
            };
            request.extensions_mut().insert(auth_user);
            
            // Continue to the next handler
            next.run(request).await
        }
        Err(e) => {
            ErrorResponse::unauthorized(&format!("Invalid token: {}", e)).into_response()
        }
    }
}

/// Extractor for authenticated user from request extensions
/// 
/// Usage in handlers:
/// ```
/// use axum::{Extension, response::IntoResponse};
/// use rme_api_rust::AuthUser;
///
/// pub async fn protected_handler(
///     Extension(user): Extension<AuthUser>,
/// ) -> impl IntoResponse {
///     // user.id, user.email, user.name available here
/// }
/// ```
pub async fn require_auth(
    request: Request,
    next: Next,
) -> Response {
    // Check if AuthUser extension exists
    if request.extensions().get::<AuthUser>().is_none() {
        return ErrorResponse::unauthorized("Authentication required").into_response();
    }
    
    next.run(request).await
}
