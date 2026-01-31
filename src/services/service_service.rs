use crate::models::Service;
use crate::repository::ServiceRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use crate::dto::service::{CreateServiceRequest, UpdateServiceRequest, ServiceResponse};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;

pub struct ServiceService {
    repository: ServiceRepository,
}

impl ServiceService {
    pub fn new(repository: ServiceRepository) -> Self {
        Self { repository }
    }

    /// Map Service model to ServiceResponse DTO
    fn map_to_response(service: Service) -> ServiceResponse {
        ServiceResponse {
            id: service.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: service.name,
            category: service.category,
            sub_category: service.sub_category,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<ServiceResponse>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(services) => Ok(services.into_iter().map(Self::map_to_response).collect()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<ServiceResponse>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((services, total)) => {
                let responses = services.into_iter().map(Self::map_to_response).collect();
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((responses, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, request: CreateServiceRequest) -> Result<(StatusCode, ServiceResponse), (StatusCode, String)> {
        let service = Service {
            id: Some(ObjectId::new()),
            name: request.name,
            category: request.category,
            sub_category: request.sub_category,
        };

        match self.repository.insert(service).await {
            Ok(created) => Ok((StatusCode::CREATED, Self::map_to_response(created))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<ServiceResponse>, (StatusCode, String)> {
        match self.repository.find_by_id(id).await {
            Ok(Some(service)) => Ok(Some(Self::map_to_response(service))),
            Ok(None) => Ok(None),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn update(&self, id: ObjectId, request: UpdateServiceRequest) -> Result<ServiceResponse, (StatusCode, String)> {
        let mut service = self.repository.find_by_id(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::NOT_FOUND, "Service not found".to_string()))?;

        if let Some(name) = request.name {
            service.name = name;
        }
        if let Some(category) = request.category {
            service.category = category;
        }
        if let Some(sub_category) = request.sub_category {
            service.sub_category = sub_category;
        }

        match self.repository.update(id, service).await {
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
