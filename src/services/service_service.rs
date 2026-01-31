use crate::models::Service;
use crate::repository::ServiceRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use axum::http::StatusCode;

pub struct ServiceService {
    repository: ServiceRepository,
}

impl ServiceService {
    pub fn new(repository: ServiceRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Service>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(services) => Ok(services),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Service>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((services, total)) => {
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((services, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
