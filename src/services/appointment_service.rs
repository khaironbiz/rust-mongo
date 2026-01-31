use crate::models::Appointment;
use crate::repository::AppointmentRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use crate::dto::appointment::{CreateAppointmentRequest, UpdateAppointmentRequest, AppointmentResponse};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;

pub struct AppointmentService {
    repository: AppointmentRepository,
}

impl AppointmentService {
    pub fn new(repository: AppointmentRepository) -> Self {
        Self { repository }
    }

    /// Map Appointment model to AppointmentResponse DTO
    fn map_to_response(appointment: Appointment) -> AppointmentResponse {
        AppointmentResponse {
            id: appointment.id.map(|id| id.to_hex()).unwrap_or_default(),
            patient_id: appointment.patient_id,
            doctor_id: appointment.doctor_id,
            date: appointment.date,
            time: appointment.time,
            status: appointment.status,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<AppointmentResponse>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(appointments) => Ok(appointments.into_iter().map(Self::map_to_response).collect()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<AppointmentResponse>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((appointments, total)) => {
                let responses = appointments.into_iter().map(Self::map_to_response).collect();
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((responses, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, request: CreateAppointmentRequest) -> Result<(StatusCode, AppointmentResponse), (StatusCode, String)> {
        let appointment = Appointment {
            id: Some(ObjectId::new()),
            patient_id: request.patient_id,
            doctor_id: request.doctor_id,
            date: request.date,
            time: request.time,
            status: request.status,
        };

        match self.repository.insert(appointment).await {
            Ok(created) => Ok((StatusCode::CREATED, Self::map_to_response(created))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<AppointmentResponse>, (StatusCode, String)> {
        match self.repository.find_by_id(id).await {
            Ok(Some(appointment)) => Ok(Some(Self::map_to_response(appointment))),
            Ok(None) => Ok(None),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn update(&self, id: ObjectId, request: UpdateAppointmentRequest) -> Result<AppointmentResponse, (StatusCode, String)> {
        let mut appointment = self.repository.find_by_id(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::NOT_FOUND, "Appointment not found".to_string()))?;

        if let Some(val) = request.patient_id { appointment.patient_id = val; }
        if let Some(val) = request.doctor_id { appointment.doctor_id = val; }
        if let Some(val) = request.date { appointment.date = val; }
        if let Some(val) = request.time { appointment.time = val; }
        if let Some(val) = request.status { appointment.status = val; }

        match self.repository.update(id, appointment).await {
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
