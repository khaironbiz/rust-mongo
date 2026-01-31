use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use crate::models::Observation;
use futures_util::stream::TryStreamExt;
use crate::pagination::PaginationParams;

pub struct ObservationRepository {
    collection: Collection<Observation>,
}

impl ObservationRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection::<Observation>("observations");
        Self { collection }
    }

    pub async fn create(&self, observation: Observation) -> Result<Observation, String> {
        let result = self
            .collection
            .insert_one(observation.clone(), None)
            .await
            .map_err(|e| e.to_string())?;

        let mut created_observation = observation;
        created_observation.id = result.inserted_id.as_object_id();

        Ok(created_observation)
    }

    pub async fn find_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<Observation>, u64), String> {
        let total = self.collection
            .count_documents(None, None)
            .await
            .map_err(|e| e.to_string())?;

        let options = mongodb::options::FindOptions::builder()
            .skip(pagination.skip())
            .limit(pagination.limit() as i64)
            .sort(doc! { "created_at": -1 })
            .build();

        let cursor = self.collection
            .find(None, options)
            .await
            .map_err(|e| e.to_string())?;

        let observations: Vec<Observation> = cursor.try_collect().await.map_err(|e| e.to_string())?;

        Ok((observations, total))
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<Observation>, String> {
        self.collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update(&self, id: ObjectId, observation: Observation) -> Result<Observation, String> {
        let filter = doc! { "_id": id };
        
        // Convert observation to BSON document for update
        let mut obs_doc = mongodb::bson::to_document(&observation).map_err(|e| e.to_string())?;
        obs_doc.remove("_id"); // Don't update the ID

        let update = doc! {
            "$set": obs_doc
        };

        self.collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| e.to_string())?;

        Ok(observation)
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, String> {
        let result = self
            .collection
            .delete_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.deleted_count > 0)
    }
}
