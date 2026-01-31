use crate::models::Nurse;
use crate::repository::NurseRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use axum::http::StatusCode;

pub struct NurseService {
    repository: NurseRepository,
}

impl NurseService {
    pub fn new(repository: NurseRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Nurse>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(nurses) => Ok(nurses),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Nurse>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((nurses, total)) => {
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((nurses, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
