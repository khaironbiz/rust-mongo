use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::repository::CodeRepository;
use crate::models::{Code, CodeCategoryEmbed};
use crate::dto::code::{CreateCodeDto, UpdateCodeDto};
use chrono::Local;

pub struct CodeService {
    repo: Arc<CodeRepository>,
}

impl CodeService {
    pub fn new(repo: Arc<CodeRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_code(&self, dto: CreateCodeDto) -> Result<Code, String> {
        // Validation: Unique Code
        if self.repo.find_by_code(&dto.code).await?.is_some() {
            return Err(format!("Code '{}' already exists", dto.code));
        }

        // Fetch Category by ID (Self-referential or from another collection if Code is Category?)
        // Assuming Category is also a Code
        let category_oid = ObjectId::parse_str(&dto.category_id).map_err(|_| "Invalid Category ID format")?;
        let category_code = self.repo.find_by_id(category_oid).await?
            .ok_or_else(|| "Category Code not found".to_string())?;

        let code = Code {
            id: None,
            code: dto.code,
            display: dto.display,
            system: dto.system,
            category: CodeCategoryEmbed {
                code: category_code.code,
                system: category_code.system,
                display: category_code.display,
            },
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        };

        self.repo.create(code).await
    }

    pub async fn get_all_codes(&self) -> Result<Vec<Code>, String> {
        self.repo.find_all().await
    }

    pub async fn get_code_by_id(&self, id: &str) -> Result<Option<Code>, String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID format")?;
        self.repo.find_by_id(oid).await
    }

    pub async fn update_code(&self, id: &str, dto: UpdateCodeDto) -> Result<Code, String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID format")?;
        
        // Check if exists
        let mut existing = self.repo.find_by_id(oid).await?
            .ok_or_else(|| "Code not found".to_string())?;

        // Validation: Unique Code if changed
        if let Some(new_code) = &dto.code {
            if new_code != &existing.code {
                if self.repo.find_by_code(new_code).await?.is_some() {
                    return Err(format!("Code '{}' already exists", new_code));
                }
                existing.code = new_code.clone();
            }
        }

        if let Some(display) = dto.display {
            existing.display = display;
        }
        if let Some(system) = dto.system {
            existing.system = system;
        }
        if let Some(category) = dto.category {
            existing.category = CodeCategoryEmbed {
                code: category.code,
                system: category.system,
                display: category.display,
            };
        }

        existing.updated_at = Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        self.repo.update(oid, existing).await
    }

    pub async fn delete_code(&self, id: &str) -> Result<bool, String> {
        let oid = ObjectId::parse_str(id).map_err(|_| "Invalid ID format")?;
        self.repo.delete(oid).await
    }
}
