use crate::models::Insurance;
use crate::repository::InsuranceRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use axum::http::StatusCode;

pub struct InsuranceService {
    repository: InsuranceRepository,
}

impl InsuranceService {
    pub fn new(repository: InsuranceRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Insurance>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(insurances) => Ok(insurances),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Insurance>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((insurances, total)) => {
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((insurances, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
