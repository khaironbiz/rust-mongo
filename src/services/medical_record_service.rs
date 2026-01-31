use crate::models::MedicalRecord;
use crate::repository::MedicalRecordRepository;
use crate::validation;
use crate::pagination::{PaginationParams, PaginationMeta};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;

pub struct MedicalRecordService {
    repository: MedicalRecordRepository,
}

impl MedicalRecordService {
    pub fn new(repository: MedicalRecordRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<MedicalRecord>, String> {
        self.repository.find_all().await
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<MedicalRecord>, PaginationMeta), String> {
        let (records, total) = self.repository.find_all_paginated(pagination.clone()).await?;
        let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
        Ok((records, meta))
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<MedicalRecord>, String> {
        self.repository.find_by_id(id).await
    }

    pub async fn create(&self, mut record: MedicalRecord) -> Result<(StatusCode, MedicalRecord), (StatusCode, String)> {
        // Validate NIK format
        if let Err(_) = validation::validate_nik(&record.nik) {
            return Err((StatusCode::BAD_REQUEST, "Invalid NIK format".to_string()));
        }

        // Check NIK uniqueness
        match self.repository.find_by_nik(&record.nik).await {
            Ok(Some(_)) => {
                return Err((StatusCode::CONFLICT, "NIK already exists".to_string()));
            }
            Ok(None) => {}
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }

        // Insert record
        match self.repository.insert(record).await {
            Ok(created) => Ok((StatusCode::CREATED, created)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn update(&self, id: ObjectId, record: MedicalRecord) -> Result<MedicalRecord, (StatusCode, String)> {
        match self.repository.update(id, record).await {
            Ok(updated) => Ok(updated),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, (StatusCode, String)> {
        match self.repository.delete(id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
