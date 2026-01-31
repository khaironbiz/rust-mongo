use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RoleCategoryDto {
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "System is required"))]
    pub system: String,
    #[validate(length(min = 1, message = "Display is required"))]
    pub display: String,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "System is required"))]
    pub system: String,
    #[validate(length(min = 1, message = "Display is required"))]
    pub display: String,
    #[validate] // Validates nested struct
    pub category: RoleCategoryDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateRoleRequest {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub system: Option<String>,
    #[serde(default)]
    pub display: Option<String>,
    #[serde(default)]
    #[validate] // Validates nested struct if present
    pub category: Option<RoleCategoryDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleResponse {
    pub id: String,
    pub code: String,
    pub system: String,
    pub display: String,
    pub category: RoleCategoryDto,
}
