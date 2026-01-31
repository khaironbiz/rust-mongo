use axum::http::StatusCode;
use bcrypt::{hash, DEFAULT_COST};
use mongodb::bson::oid::ObjectId;

use crate::dto::auth::RegisterRequest;
use crate::dto::user::{UserResponse, UpdateUserRequest};
use crate::models::User;
use crate::repository::UserRepository;
use crate::pagination::{PaginationParams, PaginationMeta};

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    /// Hash password using bcrypt
    fn hash_password(password: &str) -> Result<String, String> {
        hash(password, DEFAULT_COST).map_err(|e| format!("Failed to hash password: {}", e))
    }

    /// Map User model to UserResponse DTO
    fn map_to_response(user: User) -> UserResponse {
        UserResponse {
            id: user.id.map(|id| id.to_hex()).unwrap_or_default(),
            email: user.email,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    pub async fn get_all_paginated(&self, params: PaginationParams) -> Result<(Vec<UserResponse>, PaginationMeta), (StatusCode, String)> {
        match self.repo.find_all_paginated(params.clone()).await {
            Ok((users, total)) => {
                let responses = users.into_iter().map(Self::map_to_response).collect();
                let meta = PaginationMeta::new(params.page, params.limit, total);
                Ok((responses, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<UserResponse>, (StatusCode, String)> {
        match self.repo.find_by_id(id).await {
            Ok(Some(user)) => Ok(Some(Self::map_to_response(user))),
            Ok(None) => Ok(None),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, request: RegisterRequest) -> Result<(StatusCode, UserResponse), (StatusCode, String)> {
        // Check if email already exists
        if let Ok(Some(_)) = self.repo.find_by_email(&request.email).await {
            return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
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

        Ok((StatusCode::CREATED, Self::map_to_response(created_user)))
    }

    pub async fn update(&self, id: ObjectId, request: UpdateUserRequest) -> Result<UserResponse, (StatusCode, String)> {
        // Find existing user
        let mut user = self.repo.find_by_id(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

        // Update fields if provided
        if let Some(name) = request.name {
            user.name = name;
        }

        if let Some(email) = request.email {
            // Check if new email is already taken by another user
            if email != user.email {
                if let Ok(Some(_)) = self.repo.find_by_email(&email).await {
                    return Err((StatusCode::CONFLICT, "Email already in use".to_string()));
                }
                user.email = email.to_lowercase();
            }
        }

        if let Some(password) = request.password {
            user.password = Self::hash_password(&password)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
        }

        user.updated_at = Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        // Update in database
        let updated_user = self.repo.update(id, user).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        Ok(Self::map_to_response(updated_user))
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, (StatusCode, String)> {
        self.repo.delete(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}
