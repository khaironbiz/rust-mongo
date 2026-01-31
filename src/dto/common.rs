use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct BulkDeleteRequest {
    pub ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BulkDeleteResponse {
    pub deleted_count: usize,
    pub failed_count: usize,
    pub deleted_ids: Vec<String>,
    pub failed_ids: Vec<String>,
}
