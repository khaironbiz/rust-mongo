use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct InterpretationCodingDto {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(length(min = 1))]
    pub system: String,
    #[validate(length(min = 1))]
    pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateInterpretationRequest {
    #[validate(length(min = 1))]
    pub code: String,
    pub min: f64,
    pub max: f64,
    pub coding: InterpretationCodingDto,
    pub text: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateInterpretationRequest {
    pub code: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub coding: Option<InterpretationCodingDto>,
    pub text: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterpretationResponse {
    pub id: String,
    pub code: String,
    pub min: f64,
    pub max: f64,
    pub coding: InterpretationCodingDto,
    pub text: String,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
}
