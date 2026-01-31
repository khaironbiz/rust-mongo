use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateNurseRequest {
    pub name: String,
    pub nip: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateNurseRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub nip: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NurseResponse {
    pub id: String,
    pub name: String,
    pub nip: String,
    pub status: String,
}
