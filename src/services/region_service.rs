use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use chrono::Local;
use crate::repository::RegionRepository;
use crate::models::Region;
use crate::dto::region::{CreateRegionRequest, UpdateRegionRequest};

pub struct RegionService {
    repo: Arc<RegionRepository>,
}

impl RegionService {
    pub fn new(repo: Arc<RegionRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, dto: CreateRegionRequest) -> Result<Region, String> {
        // Unique Code Check
        if self.repo.find_by_code(&dto.code).await?.is_some() {
            return Err("Region with this code already exists".to_string());
        }

        let region = Region {
            id: None,
            code: dto.code,
            name: dto.name,
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        };

        self.repo.create(region).await
    }

    pub async fn get_all(&self) -> Result<Vec<Region>, String> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<Region>, String> {
        self.repo.find_by_id(id).await
    }

    pub async fn update(&self, id: ObjectId, dto: UpdateRegionRequest) -> Result<Region, String> {
        let mut existing = self.repo.find_by_id(id).await?
            .ok_or_else(|| "Region not found".to_string())?;

        if let Some(code) = dto.code {
            if code != existing.code {
                // Code is changing, check uniqueness
                if self.repo.find_by_code(&code).await?.is_some() {
                    return Err("Region with this code already exists".to_string());
                }
                existing.code = code;
            }
        }

        if let Some(name) = dto.name {
            existing.name = name;
        }

        existing.updated_at = Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        self.repo.update(id, existing).await
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, String> {
        self.repo.delete(id).await
    }
}
