use crate::models::Doctor;
use crate::repository::DoctorRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use axum::http::StatusCode;

pub struct DoctorService {
    repository: DoctorRepository,
}

impl DoctorService {
    pub fn new(repository: DoctorRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Doctor>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(doctors) => Ok(doctors),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Doctor>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((doctors, total)) => {
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((doctors, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, doctor: Doctor) -> Result<(StatusCode, Doctor), (StatusCode, String)> {
        match self.repository.insert(doctor).await {
            Ok(created) => Ok((StatusCode::CREATED, created)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
