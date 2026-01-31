use crate::models::Medicine;
use crate::repository::MedicineRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use axum::http::StatusCode;

pub struct MedicineService {
    repository: MedicineRepository,
}

impl MedicineService {
    pub fn new(repository: MedicineRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Medicine>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(medicines) => Ok(medicines),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Medicine>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((medicines, total)) => {
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((medicines, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
