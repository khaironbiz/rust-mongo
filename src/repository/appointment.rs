use mongodb::{bson::doc, Database, options::FindOptions};
use futures_util::stream::TryStreamExt;
use crate::models::Appointment;
use crate::pagination::PaginationParams;

pub struct AppointmentRepository {
    db: Database,
}

impl AppointmentRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Appointment>, String> {
        let collection = self.db.collection::<Appointment>("appointments");
        match collection.find(doc! {}, None).await {
            Ok(cursor) => {
                cursor
                    .try_collect::<Vec<Appointment>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Appointment>, u64), String> {
        let collection = self.db.collection::<Appointment>("appointments");
        
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
                    .try_collect::<Vec<Appointment>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))?;
                Ok((records, total))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn insert(&self, mut appointment: Appointment) -> Result<Appointment, String> {
        let collection = self.db.collection::<Appointment>("appointments");
        
        if appointment.id.is_none() {
            appointment.id = Some(mongodb::bson::oid::ObjectId::new());
        }

        collection
            .insert_one(appointment.clone(), None)
            .await
            .map_err(|e| format!("Insert failed: {}", e))?;

        Ok(appointment)
    }
}
