use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use mongodb::bson::oid::ObjectId;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;

use crate::dto::{
    RegisterRequest, LoginRequest, ForgotPasswordRequest, ResetPasswordRequest,
    AuthResponse, LoginResponse, ForgotPasswordResponse, ResetPasswordResponse,
    RefreshTokenRequest, RefreshTokenResponse,
};
use crate::models::User;
use crate::repository::UserRepository;

/// JWT Claims structure for access token
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // Subject (user id)
    pub email: String,    // User email
    pub name: String,     // User name
    pub token_type: String, // "access" or "refresh"
    pub exp: usize,       // Expiration time
    pub iat: usize,       // Issued at
}

pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    /// Get JWT secret from environment variable
    fn get_jwt_secret() -> String {
        env::var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret_key_change_me_in_production".to_string())
    }

    /// Get refresh token secret from environment variable
    fn get_refresh_secret() -> String {
        env::var("REFRESH_TOKEN_SECRET").unwrap_or_else(|_| {
            // Use JWT_SECRET with a suffix if REFRESH_TOKEN_SECRET not set
            format!("{}_refresh", Self::get_jwt_secret())
        })
    }

    /// Get JWT expiration time in hours from environment variable
    fn get_jwt_expiration_hours() -> i64 {
        env::var("JWT_EXPIRATION_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1) // Default to 1 hour for access token
    }

    /// Get refresh token expiration time in days from environment variable
    fn get_refresh_expiration_days() -> i64 {
        env::var("REFRESH_TOKEN_EXPIRATION_DAYS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(7) // Default to 7 days for refresh token
    }

    /// Hash password using bcrypt
    fn hash_password(password: &str) -> Result<String, String> {
        hash(password, DEFAULT_COST).map_err(|e| format!("Failed to hash password: {}", e))
    }

    /// Verify password against hash
    fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
        verify(password, hash).map_err(|e| format!("Failed to verify password: {}", e))
    }

    /// Generate access token for a user
    pub fn generate_access_token(user: &User) -> Result<(String, i64), String> {
        let secret = Self::get_jwt_secret();
        let expiration_hours = Self::get_jwt_expiration_hours();
        
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::hours(expiration_hours)).timestamp() as usize;
        let iat = now.timestamp() as usize;
        let expires_in = expiration_hours * 3600; // Convert hours to seconds

        let user_id = user.id.as_ref()
            .map(|id| id.to_hex())
            .unwrap_or_default();

        let claims = Claims {
            sub: user_id,
            email: user.email.clone(),
            name: user.name.clone(),
            token_type: "access".to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| format!("Failed to generate token: {}", e))?;

        Ok((token, expires_in))
    }

    /// Generate refresh token for a user
    pub fn generate_refresh_token(user: &User) -> Result<String, String> {
        let secret = Self::get_refresh_secret();
        let expiration_days = Self::get_refresh_expiration_days();
        
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::days(expiration_days)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let user_id = user.id.as_ref()
            .map(|id| id.to_hex())
            .unwrap_or_default();

        let claims = Claims {
            sub: user_id,
            email: user.email.clone(),
            name: user.name.clone(),
            token_type: "refresh".to_string(),
            exp,
            iat,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| format!("Failed to generate refresh token: {}", e))
    }

    /// Validate access token and return claims
    pub fn validate_token(token: &str) -> Result<Claims, String> {
        let secret = Self::get_jwt_secret();
        
        let validation = Validation::new(Algorithm::HS256);
        
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid token: {}", e))?;

        // Verify it's an access token
        if claims.token_type != "access" {
            return Err("Invalid token type".to_string());
        }

        Ok(claims)
    }

    /// Validate refresh token and return claims
    pub fn validate_refresh_token(token: &str) -> Result<Claims, String> {
        let secret = Self::get_refresh_secret();
        
        let validation = Validation::new(Algorithm::HS256);
        
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid refresh token: {}", e))?;

        // Verify it's a refresh token
        if claims.token_type != "refresh" {
            return Err("Invalid token type".to_string());
        }

        Ok(claims)
    }

    /// Generate random reset token
    fn generate_reset_token() -> String {
        let mut rng = rand::thread_rng();
        let token: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'a' + idx - 10) as char
                }
            })
            .collect();
        token
    }

    /// Register a new user
    pub async fn register(&self, request: RegisterRequest) -> Result<(StatusCode, AuthResponse), (StatusCode, String)> {
        // Check if email already exists
        if let Ok(Some(_)) = self.repo.find_by_email(&request.email).await {
            return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
        }

        // Validate email format
        if !request.email.contains('@') || !request.email.contains('.') {
            return Err((StatusCode::BAD_REQUEST, "Invalid email format".to_string()));
        }

        // Validate password length
        if request.password.len() < 6 {
            return Err((StatusCode::BAD_REQUEST, "Password must be at least 6 characters".to_string()));
        }

        // Validate name
        if request.name.trim().is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Name cannot be empty".to_string()));
        }

        // Hash password
        let password_hash = Self::hash_password(&request.password)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Create user
        let user = User {
            id: Some(ObjectId::new()),
            email: request.email.to_lowercase(),
            password: password_hash,
            name: request.name,
            refresh_token: None,
            reset_token: None,
            reset_token_expiry: None,
            created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: None,
        };

        // Insert user
        let created_user = self.repo.insert(user).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Generate access token
        let (access_token, expires_in) = Self::generate_access_token(&created_user)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Generate refresh token
        let refresh_token = Self::generate_refresh_token(&created_user)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Store refresh token in database
        let user_id = created_user.id.ok_or((StatusCode::INTERNAL_SERVER_ERROR, "User ID not found".to_string()))?;
        self.repo.update_refresh_token(user_id, Some(refresh_token.clone())).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        let response = AuthResponse {
            id: created_user.id.map(|id| id.to_hex()).unwrap_or_default(),
            email: created_user.email,
            name: created_user.name,
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
            created_at: created_user.created_at,
        };

        Ok((StatusCode::CREATED, response))
    }

    /// Login user
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, (StatusCode, String)> {
        // Find user by email
        let user = self.repo.find_by_email(&request.email.to_lowercase()).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()))?;

        // Verify password
        let is_valid = Self::verify_password(&request.password, &user.password)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        if !is_valid {
            return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()));
        }

        // Generate access token
        let (access_token, expires_in) = Self::generate_access_token(&user)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Generate refresh token
        let refresh_token = Self::generate_refresh_token(&user)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Store refresh token in database
        let user_id = user.id.ok_or((StatusCode::INTERNAL_SERVER_ERROR, "User ID not found".to_string()))?;
        self.repo.update_refresh_token(user_id, Some(refresh_token.clone())).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        let response = LoginResponse {
            id: user.id.map(|id| id.to_hex()).unwrap_or_default(),
            email: user.email,
            name: user.name,
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
        };

        Ok(response)
    }

    /// Refresh access token using refresh token
    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> Result<RefreshTokenResponse, (StatusCode, String)> {
        // Validate the refresh token
        let claims = Self::validate_refresh_token(&request.refresh_token)
            .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

        // Find user by refresh token in database (to verify it's still valid/not revoked)
        let user = self.repo.find_by_refresh_token(&request.refresh_token).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid or revoked refresh token".to_string()))?;

        // Verify the user ID matches
        let user_id_hex = user.id.as_ref().map(|id| id.to_hex()).unwrap_or_default();
        if user_id_hex != claims.sub {
            return Err((StatusCode::UNAUTHORIZED, "Token mismatch".to_string()));
        }

        // Generate new access token
        let (access_token, expires_in) = Self::generate_access_token(&user)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Generate new refresh token (token rotation for security)
        let new_refresh_token = Self::generate_refresh_token(&user)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Update refresh token in database
        let user_id = user.id.ok_or((StatusCode::INTERNAL_SERVER_ERROR, "User ID not found".to_string()))?;
        self.repo.update_refresh_token(user_id, Some(new_refresh_token.clone())).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token: new_refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
        })
    }

    /// Forgot password - generate reset token
    pub async fn forgot_password(&self, request: ForgotPasswordRequest) -> Result<ForgotPasswordResponse, (StatusCode, String)> {
        // Find user by email
        let user = self.repo.find_by_email(&request.email.to_lowercase()).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // For security, always return success even if email doesn't exist
        if user.is_none() {
            return Ok(ForgotPasswordResponse {
                success: true,
                message: "If the email exists, a reset token has been sent".to_string(),
                reset_token: String::new(), // Empty token for non-existent email
            });
        }

        let user = user.unwrap();
        let user_id = user.id.ok_or((StatusCode::INTERNAL_SERVER_ERROR, "User ID not found".to_string()))?;

        // Generate reset token
        let reset_token = Self::generate_reset_token();
        
        // Set expiry to 1 hour from now
        let expiry = (chrono::Utc::now() + chrono::Duration::hours(1))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // Update user with reset token
        self.repo.update_reset_token(user_id, Some(reset_token.clone()), Some(expiry)).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // In production, you would send this token via email
        // For now, we return it in the response (for testing purposes)
        Ok(ForgotPasswordResponse {
            success: true,
            message: "Reset token generated. In production, this would be sent via email.".to_string(),
            reset_token, // In production, remove this from response
        })
    }

    /// Reset password using token
    pub async fn reset_password(&self, request: ResetPasswordRequest) -> Result<ResetPasswordResponse, (StatusCode, String)> {
        // Validate new password
        if request.password.len() < 6 {
            return Err((StatusCode::BAD_REQUEST, "Password must be at least 6 characters".to_string()));
        }

        // Find user by reset token
        let user = self.repo.find_by_reset_token(&request.token).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::BAD_REQUEST, "Invalid or expired reset token".to_string()))?;

        // Check if token is expired
        if let Some(expiry) = &user.reset_token_expiry {
            let expiry_time = chrono::NaiveDateTime::parse_from_str(expiry, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid expiry format".to_string()))?;
            
            let now = chrono::Utc::now().naive_utc();
            
            if now > expiry_time {
                return Err((StatusCode::BAD_REQUEST, "Reset token has expired".to_string()));
            }
        } else {
            return Err((StatusCode::BAD_REQUEST, "Invalid reset token".to_string()));
        }

        let user_id = user.id.ok_or((StatusCode::INTERNAL_SERVER_ERROR, "User ID not found".to_string()))?;

        // Hash new password
        let password_hash = Self::hash_password(&request.password)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Update password and clear reset token
        self.repo.update_password(user_id, &password_hash).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        Ok(ResetPasswordResponse {
            success: true,
            message: "Password has been reset successfully".to_string(),
        })
    }

    /// Get user by ID (for middleware validation)
    pub async fn get_user_by_id(&self, id: ObjectId) -> Result<Option<User>, String> {
        self.repo.find_by_id(id).await
    }

    /// Logout - revoke refresh token
    pub async fn logout(&self, user_id: ObjectId) -> Result<(), (StatusCode, String)> {
        self.repo.update_refresh_token(user_id, None).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
        Ok(())
    }
}
