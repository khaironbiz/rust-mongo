use mongodb::{bson::doc, Database, options::FindOptions};
use futures_util::stream::TryStreamExt;
use crate::models::Nurse;
use crate::pagination::PaginationParams;

pub struct NurseRepository {
    db: Database,
}

impl NurseRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Nurse>, String> {
        let collection = self.db.collection::<Nurse>("nurses");
        match collection.find(doc! {}, None).await {
            Ok(cursor) => {
                cursor
                    .try_collect::<Vec<Nurse>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Nurse>, u64), String> {
        let collection = self.db.collection::<Nurse>("nurses");
        
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
                    .try_collect::<Vec<Nurse>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))?;
                Ok((records, total))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn insert(&self, nurse: Nurse) -> Result<Nurse, String> {
        let collection = self.db.collection::<Nurse>("nurses");
        match collection.insert_one(nurse.clone(), None).await {
            Ok(_) => Ok(nurse),
            Err(e) => Err(format!("Failed to insert nurse: {}", e)),
        }
    }

    pub async fn find_by_id(&self, id: mongodb::bson::oid::ObjectId) -> Result<Option<Nurse>, String> {
        let collection = self.db.collection::<Nurse>("nurses");
        collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn update(&self, id: mongodb::bson::oid::ObjectId, nurse: Nurse) -> Result<Nurse, String> {
        let collection = self.db.collection::<Nurse>("nurses");
        match collection.replace_one(doc! { "_id": id }, nurse.clone(), None).await {
            Ok(_) => Ok(nurse),
            Err(e) => Err(format!("Failed to update nurse: {}", e)),
        }
    }

    pub async fn delete(&self, id: mongodb::bson::oid::ObjectId) -> Result<bool, String> {
        let collection = self.db.collection::<Nurse>("nurses");
        match collection.delete_one(doc! { "_id": id }, None).await {
            Ok(result) => Ok(result.deleted_count > 0),
            Err(e) => Err(format!("Failed to delete nurse: {}", e)),
        }
    }
}
