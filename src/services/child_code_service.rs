use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use chrono::Local;
use crate::repository::{ChildCodeRepository, CodeRepository};
use crate::models::{ChildCode, ParentCodeEmbed};
use crate::dto::child_code::{CreateChildCodeRequest, UpdateChildCodeRequest};

pub struct ChildCodeService {
    repo: Arc<ChildCodeRepository>,
    code_repo: Arc<CodeRepository>,
}

impl ChildCodeService {
    pub fn new(repo: Arc<ChildCodeRepository>, code_repo: Arc<CodeRepository>) -> Self {
        Self { repo, code_repo }
    }

    pub async fn create(&self, dto: CreateChildCodeRequest) -> Result<ChildCode, String> {
        // Fetch Parent Code
        let parent_oid = ObjectId::parse_str(&dto.parent_code_id).map_err(|_| "Invalid Parent Code ID format")?;
        let parent_code = self.code_repo.find_by_id(parent_oid).await?
            .ok_or_else(|| "Parent Code not found".to_string())?;

        // Fetch Child Code (the one being linked)
        let child_oid = ObjectId::parse_str(&dto.child_code_id).map_err(|_| "Invalid Child Code ID format")?;
        let child_code_ref = self.code_repo.find_by_id(child_oid).await?
            .ok_or_else(|| "Child Code Reference not found".to_string())?;

        let child_code = ChildCode {
            id: None,
            parent: ParentCodeEmbed {
                code_id: parent_code.id.unwrap().to_hex(),
                code: parent_code.code,
                system: parent_code.system,
                display: parent_code.display,
            },
            code_id: child_code_ref.id.unwrap().to_hex(),
            code: child_code_ref.code,
            system: child_code_ref.system,
            display: child_code_ref.display,
            norut: dto.norut,
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        };

        self.repo.create(child_code).await
    }

    pub async fn get_all(&self) -> Result<Vec<ChildCode>, String> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<ChildCode>, String> {
        self.repo.find_by_id(id).await
    }

    pub async fn update(&self, id: ObjectId, dto: UpdateChildCodeRequest) -> Result<ChildCode, String> {
        let mut existing = self.repo.find_by_id(id).await?
            .ok_or_else(|| "Child Code entry not found".to_string())?;

        if let Some(parent_id) = dto.parent_code_id {
            let parent_oid = ObjectId::parse_str(&parent_id).map_err(|_| "Invalid Parent Code ID format")?;
            let parent_code = self.code_repo.find_by_id(parent_oid).await?
                 .ok_or_else(|| "Parent Code not found".to_string())?;
            
            existing.parent = ParentCodeEmbed {
                code_id: parent_code.id.unwrap().to_hex(),
                code: parent_code.code,
                system: parent_code.system,
                display: parent_code.display,
            };
        }

        if let Some(child_id) = dto.child_code_id {
             let child_oid = ObjectId::parse_str(&child_id).map_err(|_| "Invalid Child Code ID format")?;
             let child_code_ref = self.code_repo.find_by_id(child_oid).await?
                 .ok_or_else(|| "Child Code Reference not found".to_string())?;
            
             existing.code_id = child_code_ref.id.unwrap().to_hex();
             existing.code = child_code_ref.code;
             existing.system = child_code_ref.system;
             existing.display = child_code_ref.display;
        }

        if let Some(norut) = dto.norut {
            existing.norut = norut;
        }

        existing.updated_at = Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        self.repo.update(id, existing).await
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, String> {
        self.repo.delete(id).await
    }
}
