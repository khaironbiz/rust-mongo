use mongodb::{bson::doc, Database, options::FindOptions};
use futures_util::stream::TryStreamExt;
use crate::models::File;
use crate::pagination::PaginationParams;

pub struct FileRepository {
    db: Database,
}

impl FileRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<File>, String> {
        let collection = self.db.collection::<File>("files");
        match collection.find(doc! {}, None).await {
            Ok(cursor) => {
                cursor
                    .try_collect::<Vec<File>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<File>, u64), String> {
        let collection = self.db.collection::<File>("files");
        
        let total = collection
            .count_documents(doc! {}, None)
            .await
            .map_err(|e| format!("Failed to count documents: {}", e))?;

        let options = FindOptions::builder()
            .skip(pagination.skip())
            .limit(pagination.limit() as i64)
            .build();

        match collection.find(doc! {}, options).await {
            Ok(cursor) => {
                let records = cursor
                    .try_collect::<Vec<File>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))?;
                Ok((records, total))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_by_id(&self, id: mongodb::bson::oid::ObjectId) -> Result<Option<File>, String> {
        let collection = self.db.collection::<File>("files");
        collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn insert(&self, mut file: File) -> Result<File, String> {
        let collection = self.db.collection::<File>("files");
        
        if file.id.is_none() {
            file.id = Some(mongodb::bson::oid::ObjectId::new());
        }

        collection
            .insert_one(file.clone(), None)
            .await
            .map_err(|e| format!("Insert failed: {}", e))?;

        Ok(file)
    }

    pub async fn delete(&self, id: mongodb::bson::oid::ObjectId) -> Result<bool, String> {
        let collection = self.db.collection::<File>("files");
        
        let result = collection
            .delete_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Delete failed: {}", e))?;

        Ok(result.deleted_count > 0)
    }
}
