use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateServiceRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "Category is required"))]
    pub category: String,
    #[validate(length(min = 1, message = "Sub-category is required"))]
    pub sub_category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateServiceRequest {
    #[serde(default)]
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Category is required"))]
    pub category: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Sub-category is required"))]
    pub sub_category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceResponse {
    pub id: String,
    pub name: String,
    pub category: String,
    pub sub_category: String,
}
