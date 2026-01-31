use crate::models::MedicalRecord;
use crate::repository::MedicalRecordRepository;
use crate::validation;
use crate::pagination::{PaginationParams, PaginationMeta};
use crate::dto::medical_record::{CreateMedicalRecordRequest, UpdateMedicalRecordRequest, MedicalRecordResponse};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;

pub struct MedicalRecordService {
    repository: MedicalRecordRepository,
}

impl MedicalRecordService {
    pub fn new(repository: MedicalRecordRepository) -> Self {
        Self { repository }
    }

    /// Map MedicalRecord model to MedicalRecordResponse DTO
    fn map_to_response(record: MedicalRecord) -> MedicalRecordResponse {
        MedicalRecordResponse {
            id: record.id.map(|id| id.to_hex()).unwrap_or_default(),
            nik: record.nik,
            nrme: record.nrme,
            name: record.name,
            dob: record.dob,
            gender: record.gender,
            hp: record.hp,
            email: record.email,
            last_visit_date: record.last_visit_date,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<MedicalRecordResponse>, String> {
        let records = self.repository.find_all().await?;
        Ok(records.into_iter().map(Self::map_to_response).collect())
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<MedicalRecordResponse>, PaginationMeta), String> {
        let (records, total) = self.repository.find_all_paginated(pagination.clone()).await?;
        let responses = records.into_iter().map(Self::map_to_response).collect();
        let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
        Ok((responses, meta))
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<MedicalRecordResponse>, String> {
        let record = self.repository.find_by_id(id).await?;
        Ok(record.map(Self::map_to_response))
    }

    pub async fn create(&self, request: CreateMedicalRecordRequest) -> Result<(StatusCode, MedicalRecordResponse), (StatusCode, String)> {
        // Validate NIK format
        if let Err(_) = validation::validate_nik(&request.nik) {
            return Err((StatusCode::BAD_REQUEST, "Invalid NIK format".to_string()));
        }

        // Check NIK uniqueness
        match self.repository.find_by_nik(&request.nik).await {
            Ok(Some(_)) => {
                return Err((StatusCode::CONFLICT, "NIK already exists".to_string()));
            }
            Ok(None) => {}
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }

        // Create record model
        let record = MedicalRecord {
            id: Some(ObjectId::new()),
            nik: request.nik,
            nrme: request.nrme,
            name: request.name,
            dob: request.dob,
            gender: request.gender,
            hp: request.hp,
            email: request.email,
            last_visit_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        };

        // Insert record
        match self.repository.insert(record).await {
            Ok(created) => Ok((StatusCode::CREATED, Self::map_to_response(created))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn update(&self, id: ObjectId, request: UpdateMedicalRecordRequest) -> Result<MedicalRecordResponse, (StatusCode, String)> {
        // Find existing record
        let mut record = self.repository.find_by_id(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::NOT_FOUND, "Medical record not found".to_string()))?;

        // Update fields if provided
        if let Some(nrme) = request.nrme { record.nrme = nrme; }
        if let Some(name) = request.name { record.name = name; }
        if let Some(dob) = request.dob { record.dob = dob; }
        if let Some(gender) = request.gender { record.gender = gender; }
        if let Some(hp) = request.hp { record.hp = hp; }
        if let Some(email) = request.email { record.email = email; }

        record.last_visit_date = chrono::Local::now().format("%Y-%m-%d").to_string();

        match self.repository.update(id, record).await {
            Ok(updated) => Ok(Self::map_to_response(updated)),
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
