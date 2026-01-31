use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateInsuranceRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[serde(rename = "type")]
    #[validate(length(min = 1, message = "Type is required"))]
    pub insurance_type: String,
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateInsuranceRequest {
    #[serde(default)]
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    #[validate(length(min = 1, message = "Type is required"))]
    pub insurance_type: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub insurance_type: String,
    pub code: String,
    pub status: String,
}
