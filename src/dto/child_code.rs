use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateChildCodeRequest {
    #[validate(length(min = 24, max = 24, message = "Parent Code ID must be 24 characters"))]
    pub parent_code_id: String,
    #[validate(length(min = 24, max = 24, message = "Child Code ID must be 24 characters"))]
    pub child_code_id: String,
    #[validate(range(min = 1, message = "Sequence number (norut) must be positive"))]
    pub norut: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateChildCodeRequest {
    #[serde(default)]
    #[validate(length(min = 24, max = 24, message = "Parent Code ID must be 24 characters"))]
    pub parent_code_id: Option<String>,
    #[serde(default)]
    #[validate(length(min = 24, max = 24, message = "Child Code ID must be 24 characters"))]
    pub child_code_id: Option<String>,
    #[serde(default)]
    #[validate(range(min = 1, message = "Sequence number (norut) must be positive"))]
    pub norut: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParentCodeEmbedDto {
    pub code_id: String,
    pub code: String,
    pub system: String,
    pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildCodeResponse {
    pub id: String,
    pub parent: ParentCodeEmbedDto,
    pub code_id: String,
    pub code: String,
    pub system: String,
    pub display: String,
    pub norut: i32,
    pub created_at: String,
    pub updated_at: Option<String>,
}
