use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateAppointmentRequest {
    #[validate(length(min = 24, max = 24, message = "Patient IDs must be 24 characters"))]
    pub patient_id: String,
    #[validate(length(min = 24, max = 24, message = "Doctor IDs must be 24 characters"))]
    pub doctor_id: String,
    #[validate(length(min = 1, message = "Date is required"))]
    pub date: String,
    #[validate(length(min = 1, message = "Time is required"))]
    pub time: String,
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateAppointmentRequest {
    #[serde(default)]
    #[validate(length(min = 24, max = 24, message = "Patient IDs must be 24 characters"))]
    pub patient_id: Option<String>,
    #[serde(default)]
    #[validate(length(min = 24, max = 24, message = "Doctor IDs must be 24 characters"))]
    pub doctor_id: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Date is required"))]
    pub date: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Time is required"))]
    pub time: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppointmentResponse {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub date: String,
    pub time: String,
    pub status: String,
}
