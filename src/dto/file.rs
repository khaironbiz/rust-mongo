use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub extension: String,
    pub size: u64,
    pub path: String,
    pub url: String,
    pub uploader: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
