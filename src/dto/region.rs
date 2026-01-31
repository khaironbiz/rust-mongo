use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateRegionRequest {
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateRegionRequest {
    #[serde(default)]
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegionResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}
