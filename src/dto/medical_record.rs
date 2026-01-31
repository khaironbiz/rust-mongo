use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateMedicalRecordRequest {
    #[validate(length(min = 16, max = 16, message = "NIK must be 16 characters"))]
    pub nik: String,
    #[validate(length(min = 1, message = "NRME is required"))]
    pub nrme: String,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "DOB is required"))]
    pub dob: String,
    #[validate(length(min = 1, message = "Gender is required"))]
    pub gender: String,
    #[validate(length(min = 1, message = "HP is required"))]
    pub hp: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateMedicalRecordRequest {
    #[serde(default)]
    #[validate(length(min = 1, message = "NRME is required"))]
    pub nrme: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "DOB is required"))]
    pub dob: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Gender is required"))]
    pub gender: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "HP is required"))]
    pub hp: Option<String>,
    #[serde(default)]
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalRecordResponse {
    pub id: String,
    pub nik: String,
    pub nrme: String,
    pub name: String,
    pub dob: String,
    pub gender: String,
    pub hp: String,
    pub email: String,
    pub last_visit_date: String,
}
