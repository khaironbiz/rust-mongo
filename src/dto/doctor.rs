use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDoctorRequest {
    pub name: String,
    pub nip: String,
    pub sip: String,
    pub specialization: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateDoctorRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub nip: Option<String>,
    #[serde(default)]
    pub sip: Option<String>,
    #[serde(default)]
    pub specialization: Option<String>,
    #[serde(default)]
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
