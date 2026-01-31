use crate::models::File;
use crate::repository::FileRepository;
use crate::validation;
use crate::pagination::{PaginationParams, PaginationMeta};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;
use std::sync::Arc;
use aws_sdk_s3::Client as S3Client;

pub struct FileService {
    repository: FileRepository,
    s3_client: Arc<S3Client>,
}

impl FileService {
    pub fn new(repository: FileRepository, s3_client: Arc<S3Client>) -> Self {
        Self {
            repository,
            s3_client,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<File>, String> {
        self.repository.find_all().await
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<File>, PaginationMeta), String> {
        let (files, total) = self.repository.find_all_paginated(pagination.clone()).await?;
        let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
        Ok((files, meta))
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<File>, String> {
        self.repository.find_by_id(id).await
    }

    pub async fn create(
        &self,
        file_name: String,
        file_bytes: Vec<u8>,
        uploader: String,
    ) -> Result<(StatusCode, File), (StatusCode, String)> {
        // Validate file
        let file_size = file_bytes.len() as u64;
        if let Err(_) = validation::validate_file_upload(&file_name, file_size) {
            return Err((StatusCode::BAD_REQUEST, "Invalid file".to_string()));
        }

        // Upload to S3
        let s3_key = crate::s3::generate_s3_key(&file_name);
        let bucket = std::env::var("AWS_BUCKET").unwrap_or_else(|_| "atm-sehat".to_string());

        let s3_url = match crate::s3::upload_file_to_s3(&self.s3_client, &bucket, &s3_key, file_bytes).await {
            Ok(url) => url,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        };

        // Create file record
        let mut file_record = File {
            id: Some(ObjectId::new()),
            name: file_name.clone(),
            file_type: mime_guess::from_path(&file_name)
                .first_raw()
                .unwrap_or("application/octet-stream")
                .to_string(),
            extension: file_name.split('.').last().unwrap_or("").to_string(),
            size: file_size,
            path: s3_key,
            url: s3_url,
            uploader,
            created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };

        // Save to database
        match self.repository.insert(file_record).await {
            Ok(created) => Ok((StatusCode::CREATED, created)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, (StatusCode, String)> {
        // Get file record to retrieve S3 path
        let file = match self.repository.find_by_id(id).await {
            Ok(Some(f)) => f,
            Ok(None) => return Err((StatusCode::NOT_FOUND, "File not found".to_string())),
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        };

        // Delete from S3
        let bucket = std::env::var("AWS_BUCKET").unwrap_or_else(|_| "atm-sehat".to_string());
        if let Err(e) = crate::s3::delete_file_from_s3(&self.s3_client, &bucket, &file.path).await {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e));
        }

        // Delete from database
        match self.repository.delete(id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
