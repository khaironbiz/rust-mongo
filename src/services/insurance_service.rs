use crate::models::Insurance;
use crate::repository::InsuranceRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use crate::dto::insurance::{CreateInsuranceRequest, UpdateInsuranceRequest, InsuranceResponse};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;

pub struct InsuranceService {
    repository: InsuranceRepository,
}

impl InsuranceService {
    pub fn new(repository: InsuranceRepository) -> Self {
        Self { repository }
    }

    /// Map Insurance model to InsuranceResponse DTO
    fn map_to_response(insurance: Insurance) -> InsuranceResponse {
        InsuranceResponse {
            id: insurance.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: insurance.name,
            insurance_type: insurance.insurance_type,
            code: insurance.code,
            status: insurance.status,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<InsuranceResponse>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(insurances) => Ok(insurances.into_iter().map(Self::map_to_response).collect()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<InsuranceResponse>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((insurances, total)) => {
                let responses = insurances.into_iter().map(Self::map_to_response).collect();
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((responses, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, request: CreateInsuranceRequest) -> Result<(StatusCode, InsuranceResponse), (StatusCode, String)> {
        let insurance = Insurance {
            id: Some(ObjectId::new()),
            name: request.name,
            insurance_type: request.insurance_type,
            code: request.code,
            status: request.status,
        };

        match self.repository.insert(insurance).await {
            Ok(created) => Ok((StatusCode::CREATED, Self::map_to_response(created))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<InsuranceResponse>, (StatusCode, String)> {
        match self.repository.find_by_id(id).await {
            Ok(Some(insurance)) => Ok(Some(Self::map_to_response(insurance))),
            Ok(None) => Ok(None),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn update(&self, id: ObjectId, request: UpdateInsuranceRequest) -> Result<InsuranceResponse, (StatusCode, String)> {
        let mut insurance = self.repository.find_by_id(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::NOT_FOUND, "Insurance not found".to_string()))?;

        if let Some(name) = request.name {
            insurance.name = name;
        }
        if let Some(insurance_type) = request.insurance_type {
            insurance.insurance_type = insurance_type;
        }
        if let Some(code) = request.code {
            insurance.code = code;
        }
        if let Some(status) = request.status {
            insurance.status = status;
        }

        match self.repository.update(id, insurance).await {
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
