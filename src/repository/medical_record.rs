use mongodb::{bson::doc, Database, options::FindOptions};
use futures_util::stream::TryStreamExt;
use crate::models::MedicalRecord;
use crate::pagination::PaginationParams;

pub struct MedicalRecordRepository {
    db: Database,
}

impl MedicalRecordRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<MedicalRecord>, String> {
        let collection = self.db.collection::<MedicalRecord>("medical_records");
        match collection.find(doc! {}, None).await {
            Ok(cursor) => {
                cursor
                    .try_collect::<Vec<MedicalRecord>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<MedicalRecord>, u64), String> {
        let collection = self.db.collection::<MedicalRecord>("medical_records");
        
        // Get total count
        let total = collection
            .count_documents(doc! {}, None)
            .await
            .map_err(|e| format!("Failed to count documents: {}", e))?;

        // Get paginated results
        let options = FindOptions::builder()
            .skip(pagination.skip())
            .limit(pagination.limit() as i64)
            .build();

        match collection.find(doc! {}, options).await {
            Ok(cursor) => {
                let records = cursor
                    .try_collect::<Vec<MedicalRecord>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))?;
                Ok((records, total))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_by_id(&self, id: mongodb::bson::oid::ObjectId) -> Result<Option<MedicalRecord>, String> {
        let collection = self.db.collection::<MedicalRecord>("medical_records");
        collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn find_by_nik(&self, nik: &str) -> Result<Option<MedicalRecord>, String> {
        let collection = self.db.collection::<MedicalRecord>("medical_records");
        collection
            .find_one(doc! { "nik": nik }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn insert(&self, mut record: MedicalRecord) -> Result<MedicalRecord, String> {
        let collection = self.db.collection::<MedicalRecord>("medical_records");
        
        if record.id.is_none() {
            record.id = Some(mongodb::bson::oid::ObjectId::new());
        }

        collection
            .insert_one(record.clone(), None)
            .await
            .map_err(|e| format!("Insert failed: {}", e))?;

        Ok(record)
    }

    pub async fn update(&self, id: mongodb::bson::oid::ObjectId, record: MedicalRecord) -> Result<MedicalRecord, String> {
        let collection = self.db.collection::<MedicalRecord>("medical_records");
        
        collection
            .replace_one(doc! { "_id": id }, record.clone(), None)
            .await
            .map_err(|e| format!("Update failed: {}", e))?;

        Ok(record)
    }

    pub async fn delete(&self, id: mongodb::bson::oid::ObjectId) -> Result<bool, String> {
        let collection = self.db.collection::<MedicalRecord>("medical_records");
        
        let result = collection
            .delete_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Delete failed: {}", e))?;

        Ok(result.deleted_count > 0)
    }
}
