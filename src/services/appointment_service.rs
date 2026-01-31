use crate::models::Appointment;
use crate::repository::AppointmentRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use axum::http::StatusCode;

pub struct AppointmentService {
    repository: AppointmentRepository,
}

impl AppointmentService {
    pub fn new(repository: AppointmentRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Appointment>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(appointments) => Ok(appointments),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Appointment>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((appointments, total)) => {
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((appointments, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, appointment: Appointment) -> Result<(StatusCode, Appointment), (StatusCode, String)> {
        match self.repository.insert(appointment).await {
            Ok(created) => Ok((StatusCode::CREATED, created)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
