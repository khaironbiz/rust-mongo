use mongodb::{bson::doc, Database, options::FindOptions};
use futures_util::stream::TryStreamExt;
use crate::models::Insurance;
use crate::pagination::PaginationParams;

pub struct InsuranceRepository {
    db: Database,
}

impl InsuranceRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Insurance>, String> {
        let collection = self.db.collection::<Insurance>("insurances");
        match collection.find(doc! {}, None).await {
            Ok(cursor) => {
                cursor
                    .try_collect::<Vec<Insurance>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Insurance>, u64), String> {
        let collection = self.db.collection::<Insurance>("insurances");
        
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
                    .try_collect::<Vec<Insurance>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))?;
                Ok((records, total))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn insert(&self, insurance: Insurance) -> Result<Insurance, String> {
        let collection = self.db.collection::<Insurance>("insurances");
        match collection.insert_one(insurance.clone(), None).await {
            Ok(_) => Ok(insurance),
            Err(e) => Err(format!("Failed to insert insurance: {}", e)),
        }
    }

    pub async fn find_by_id(&self, id: mongodb::bson::oid::ObjectId) -> Result<Option<Insurance>, String> {
        let collection = self.db.collection::<Insurance>("insurances");
        collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn update(&self, id: mongodb::bson::oid::ObjectId, insurance: Insurance) -> Result<Insurance, String> {
        let collection = self.db.collection::<Insurance>("insurances");
        match collection.replace_one(doc! { "_id": id }, insurance.clone(), None).await {
            Ok(_) => Ok(insurance),
            Err(e) => Err(format!("Failed to update insurance: {}", e)),
        }
    }

    pub async fn delete(&self, id: mongodb::bson::oid::ObjectId) -> Result<bool, String> {
        let collection = self.db.collection::<Insurance>("insurances");
        match collection.delete_one(doc! { "_id": id }, None).await {
            Ok(result) => Ok(result.deleted_count > 0),
            Err(e) => Err(format!("Failed to delete insurance: {}", e)),
        }
    }
}
