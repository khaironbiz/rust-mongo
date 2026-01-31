use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CodeCategoryEmbedDto {
    #[validate(length(min = 1, message = "Category Code cannot be empty"))]
    pub code: String,
    #[validate(length(min = 1, message = "Category System cannot be empty"))]
    pub system: String,
    #[validate(length(min = 1, message = "Category Display cannot be empty"))]
    pub display: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateCodeDto {
    #[validate(length(min = 1, message = "Code cannot be empty"))]
    pub code: String,
    #[validate(length(min = 1, message = "Display cannot be empty"))]
    pub display: String,
    #[validate(length(min = 1, message = "System cannot be empty"))]
    pub system: String,
    #[validate]
    pub category: CodeCategoryEmbedDto,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UpdateCodeDto {
    pub code: Option<String>,
    pub display: Option<String>,
    pub system: Option<String>,
    #[validate]
    pub category: Option<CodeCategoryEmbedDto>,
}
