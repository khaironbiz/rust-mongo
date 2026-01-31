use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateDoctorRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "NIP is required"))]
    pub nip: String,
    #[validate(length(min = 1, message = "SIP is required"))]
    pub sip: String,
    #[validate(length(min = 1, message = "Specialization is required"))]
    pub specialization: String,
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateDoctorRequest {
    #[serde(default)]
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "NIP is required"))]
    pub nip: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "SIP is required"))]
    pub sip: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Specialization is required"))]
    pub specialization: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoctorResponse {
    pub id: String,
    pub name: String,
    pub nip: String,
    pub sip: String,
    pub specialization: String,
    pub status: String,
}
