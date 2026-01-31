use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateInsuranceRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub insurance_type: String,
    pub code: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateInsuranceRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub insurance_type: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
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
