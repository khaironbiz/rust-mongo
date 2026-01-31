use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAppointmentRequest {
    pub patient_id: String,
    pub doctor_id: String,
    pub date: String,
    pub time: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAppointmentRequest {
    #[serde(default)]
    pub patient_id: Option<String>,
    #[serde(default)]
    pub doctor_id: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub time: Option<String>,
    #[serde(default)]
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
