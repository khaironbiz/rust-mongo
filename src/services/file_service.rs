use crate::models::File;
use crate::repository::FileRepository;
use crate::validation;
use crate::pagination::{PaginationParams, PaginationMeta};
use crate::dto::file::FileResponse;
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

    /// Map File model to FileResponse DTO
    fn map_to_response(file: File) -> FileResponse {
        FileResponse {
            id: file.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: file.name,
            file_type: file.file_type,
            extension: file.extension,
            size: file.size,
            path: file.path,
            url: file.url,
            uploader: file.uploader,
            created_at: file.created_at,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<FileResponse>, String> {
        let files = self.repository.find_all().await?;
        Ok(files.into_iter().map(Self::map_to_response).collect())
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<FileResponse>, PaginationMeta), String> {
        let (files, total) = self.repository.find_all_paginated(pagination.clone()).await?;
        let responses = files.into_iter().map(Self::map_to_response).collect();
        let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
        Ok((responses, meta))
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<FileResponse>, String> {
        let file = self.repository.find_by_id(id).await?;
        Ok(file.map(Self::map_to_response))
    }

    pub async fn create(
        &self,
        file_name: String,
        file_bytes: Vec<u8>,
        uploader: String,
    ) -> Result<(StatusCode, FileResponse), (StatusCode, String)> {
        // ... (validation and upload logic same) ...
        let file_size = file_bytes.len() as u64;
        if let Err(_) = validation::validate_file_upload(&file_name, file_size) {
            return Err((StatusCode::BAD_REQUEST, "Invalid file".to_string()));
        }

        let s3_key = crate::s3::generate_s3_key(&file_name);
        let bucket = std::env::var("AWS_BUCKET").unwrap_or_else(|_| "atm-sehat".to_string());

        let s3_url = match crate::s3::upload_file_to_s3(&self.s3_client, &bucket, &s3_key, file_bytes).await {
            Ok(url) => url,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        };

        let file_record = File {
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

        match self.repository.insert(file_record).await {
            Ok(created) => Ok((StatusCode::CREATED, Self::map_to_response(created))),
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
