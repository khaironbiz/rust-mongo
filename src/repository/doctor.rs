use mongodb::{bson::doc, Database, options::FindOptions};
use futures_util::stream::TryStreamExt;
use crate::models::Doctor;
use crate::pagination::PaginationParams;

pub struct DoctorRepository {
    db: Database,
}

impl DoctorRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Doctor>, String> {
        let collection = self.db.collection::<Doctor>("doctors");
        match collection.find(doc! {}, None).await {
            Ok(cursor) => {
                cursor
                    .try_collect::<Vec<Doctor>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Doctor>, u64), String> {
        let collection = self.db.collection::<Doctor>("doctors");
        
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
                    .try_collect::<Vec<Doctor>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))?;
                Ok((records, total))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_by_id(&self, id: mongodb::bson::oid::ObjectId) -> Result<Option<Doctor>, String> {
        let collection = self.db.collection::<Doctor>("doctors");
        collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn insert(&self, mut doctor: Doctor) -> Result<Doctor, String> {
        let collection = self.db.collection::<Doctor>("doctors");
        
        if doctor.id.is_none() {
            doctor.id = Some(mongodb::bson::oid::ObjectId::new());
        }

        collection
            .insert_one(doctor.clone(), None)
            .await
            .map_err(|e| format!("Insert failed: {}", e))?;

        Ok(doctor)
    }
}
