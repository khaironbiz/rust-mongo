use std::sync::Arc;
use mongodb::bson::{oid::ObjectId, doc};
use chrono::Local;
use crate::repository::InterpretationRepository;
use crate::models::{Interpretation, InterpretationCoding};
use crate::dto::interpretation::{CreateInterpretationRequest, UpdateInterpretationRequest};

pub struct InterpretationService {
    repo: Arc<InterpretationRepository>,
}

impl InterpretationService {
    pub fn new(repo: Arc<InterpretationRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, dto: CreateInterpretationRequest) -> Result<Interpretation, String> {
        // Unique Code Check
        if self.repo.find_by_code(&dto.code).await?.is_some() {
            return Err("Interpretation with this code already exists".to_string());
        }

        let interpretation = Interpretation {
            id: None,
            code: dto.code,
            min: dto.min,
            max: dto.max,
            coding: InterpretationCoding {
                code: dto.coding.code,
                system: dto.coding.system,
                display: dto.coding.display,
            },
            text: dto.text.unwrap_or_default(),
            created_at: Some(dto.created_at.unwrap_or_else(|| Local::now().format("%Y-%m-%d %H:%M:%S").to_string())),
            updated_at: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        };

        self.repo.create(interpretation).await
    }

    pub async fn get_all(&self) -> Result<Vec<Interpretation>, String> {
        self.repo.find_all().await
    }

    pub async fn get_all_paginated(&self, params: crate::pagination::PaginationParams) -> Result<(Vec<Interpretation>, crate::pagination::PaginationMeta), String> {
        let (interpretations, total) = self.repo.find_all_paginated(params.clone()).await?;
        let meta = crate::pagination::PaginationMeta::new(params.page, params.limit, total);
        Ok((interpretations, meta))
    }

    pub async fn get_by_code_paginated(&self, code: &str, params: crate::pagination::PaginationParams) -> Result<(Vec<Interpretation>, crate::pagination::PaginationMeta), String> {
        let filter = doc! { "code": code };
        let (interpretations, total) = self.repo.find_by_filter_paginated(filter, params.clone()).await?;
        let meta = crate::pagination::PaginationMeta::new(params.page, params.limit, total);
        Ok((interpretations, meta))
    }

    pub async fn get_by_coding_code_paginated(&self, coding_code: &str, params: crate::pagination::PaginationParams) -> Result<(Vec<Interpretation>, crate::pagination::PaginationMeta), String> {
        let filter = doc! { "coding.code": coding_code };
        let (interpretations, total) = self.repo.find_by_filter_paginated(filter, params.clone()).await?;
        let meta = crate::pagination::PaginationMeta::new(params.page, params.limit, total);
        Ok((interpretations, meta))
    }

    pub async fn get_by_code_and_coding_code(&self, code: &str, coding_code: &str) -> Result<Option<Interpretation>, String> {
        self.repo.find_by_code_and_coding_code(code, coding_code).await
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<Interpretation>, String> {
        self.repo.find_by_id(id).await
    }

    pub async fn update(&self, id: ObjectId, dto: UpdateInterpretationRequest) -> Result<Interpretation, String> {
        let mut existing = self.repo.find_by_id(id).await?
            .ok_or_else(|| "Interpretation not found".to_string())?;

        if let Some(code) = dto.code {
            if code != existing.code {
                // Code is changing, check uniqueness
                if self.repo.find_by_code(&code).await?.is_some() {
                    return Err("Interpretation with this code already exists".to_string());
                }
                existing.code = code;
            }
        }

        if let Some(min) = dto.min {
            existing.min = min;
        }

        if let Some(max) = dto.max {
            existing.max = max;
        }

        if let Some(coding_dto) = dto.coding {
            existing.coding = InterpretationCoding {
                code: coding_dto.code,
                system: coding_dto.system,
                display: coding_dto.display,
            };
        }

        if let Some(text) = dto.text {
            existing.text = text;
        }

        if let Some(created_at) = dto.created_at {
            existing.created_at = Some(created_at);
        }

        existing.updated_at = Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        self.repo.update(id, existing).await
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, String> {
        self.repo.delete(id).await
    }
}
