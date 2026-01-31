use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateUserRequest {
    #[serde(default)]
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[serde(default)]
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: Option<String>,
}
