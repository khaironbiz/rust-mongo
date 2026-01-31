use crate::models::Nurse;
use crate::repository::NurseRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use crate::dto::nurse::{CreateNurseRequest, UpdateNurseRequest, NurseResponse};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;

pub struct NurseService {
    repository: NurseRepository,
}

impl NurseService {
    pub fn new(repository: NurseRepository) -> Self {
        Self { repository }
    }

    /// Map Nurse model to NurseResponse DTO
    fn map_to_response(nurse: Nurse) -> NurseResponse {
        NurseResponse {
            id: nurse.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: nurse.name,
            nip: nurse.nip,
            status: nurse.status,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<NurseResponse>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(nurses) => Ok(nurses.into_iter().map(Self::map_to_response).collect()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<NurseResponse>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((nurses, total)) => {
                let responses = nurses.into_iter().map(Self::map_to_response).collect();
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((responses, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, request: CreateNurseRequest) -> Result<(StatusCode, NurseResponse), (StatusCode, String)> {
        let nurse = Nurse {
            id: Some(ObjectId::new()),
            name: request.name,
            nip: request.nip,
            status: request.status,
        };

        match self.repository.insert(nurse).await {
            Ok(created) => Ok((StatusCode::CREATED, Self::map_to_response(created))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<NurseResponse>, (StatusCode, String)> {
        match self.repository.find_by_id(id).await {
            Ok(Some(nurse)) => Ok(Some(Self::map_to_response(nurse))),
            Ok(None) => Ok(None),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn update(&self, id: ObjectId, request: UpdateNurseRequest) -> Result<NurseResponse, (StatusCode, String)> {
        let mut nurse = self.repository.find_by_id(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::NOT_FOUND, "Nurse not found".to_string()))?;

        if let Some(name) = request.name {
            nurse.name = name;
        }
        if let Some(nip) = request.nip {
            nurse.nip = nip;
        }
        if let Some(status) = request.status {
            nurse.status = status;
        }

        match self.repository.update(id, nurse).await {
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
