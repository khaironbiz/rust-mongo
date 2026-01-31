use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMedicalRecordRequest {
    pub nik: String,
    pub nrme: String,
    pub name: String,
    pub dob: String,
    pub gender: String,
    pub hp: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateMedicalRecordRequest {
    #[serde(default)]
    pub nrme: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub dob: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub hp: Option<String>,
    #[serde(default)]
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
