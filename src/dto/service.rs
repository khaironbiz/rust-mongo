use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateServiceRequest {
    pub name: String,
    pub category: String,
    pub sub_category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateServiceRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub sub_category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceResponse {
    pub id: String,
    pub name: String,
    pub category: String,
    pub sub_category: String,
}
