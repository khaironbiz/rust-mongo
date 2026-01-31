use crate::models::Doctor;
use crate::repository::DoctorRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use crate::dto::doctor::{CreateDoctorRequest, UpdateDoctorRequest, DoctorResponse};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;

pub struct DoctorService {
    repository: DoctorRepository,
}

impl DoctorService {
    pub fn new(repository: DoctorRepository) -> Self {
        Self { repository }
    }

    /// Map Doctor model to DoctorResponse DTO
    fn map_to_response(doctor: Doctor) -> DoctorResponse {
        DoctorResponse {
            id: doctor.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: doctor.name,
            nip: doctor.nip,
            sip: doctor.sip,
            specialization: doctor.specialization,
            status: doctor.status,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<DoctorResponse>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(doctors) => Ok(doctors.into_iter().map(Self::map_to_response).collect()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<DoctorResponse>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((doctors, total)) => {
                let responses = doctors.into_iter().map(Self::map_to_response).collect();
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((responses, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, request: CreateDoctorRequest) -> Result<(StatusCode, DoctorResponse), (StatusCode, String)> {
        let doctor = Doctor {
            id: Some(ObjectId::new()),
            name: request.name,
            nip: request.nip,
            sip: request.sip,
            specialization: request.specialization,
            status: request.status,
        };

        match self.repository.insert(doctor).await {
            Ok(created) => Ok((StatusCode::CREATED, Self::map_to_response(created))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<DoctorResponse>, (StatusCode, String)> {
        match self.repository.find_by_id(id).await {
            Ok(Some(doctor)) => Ok(Some(Self::map_to_response(doctor))),
            Ok(None) => Ok(None),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn update(&self, id: ObjectId, request: UpdateDoctorRequest) -> Result<DoctorResponse, (StatusCode, String)> {
        let mut doctor = self.repository.find_by_id(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::NOT_FOUND, "Doctor not found".to_string()))?;

        if let Some(name) = request.name {
            doctor.name = name;
        }
        if let Some(nip) = request.nip {
            doctor.nip = nip;
        }
        if let Some(sip) = request.sip {
            doctor.sip = sip;
        }
        if let Some(specialization) = request.specialization {
            doctor.specialization = specialization;
        }
        if let Some(status) = request.status {
            doctor.status = status;
        }

        match self.repository.update(id, doctor).await {
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
