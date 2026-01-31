use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use crate::models::Interpretation;
use futures_util::stream::TryStreamExt;

pub struct InterpretationRepository {
    collection: Collection<Interpretation>,
}

impl InterpretationRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection::<Interpretation>("interpretations");
        Self { collection }
    }

    pub async fn create(&self, interpretation: Interpretation) -> Result<Interpretation, String> {
        let result = self
            .collection
            .insert_one(interpretation.clone(), None)
            .await
            .map_err(|e| e.to_string())?;

        let mut created_interpretation = interpretation;
        created_interpretation.id = result.inserted_id.as_object_id();

        Ok(created_interpretation)
    }

    pub async fn find_all(&self) -> Result<Vec<Interpretation>, String> {
        let cursor = self
            .collection
            .find(None, None)
            .await
            .map_err(|e| e.to_string())?;

        let interpretations: Vec<Interpretation> = cursor.try_collect().await.map_err(|e| e.to_string())?;

        Ok(interpretations)
    }

    pub async fn find_all_paginated(&self, pagination: crate::pagination::PaginationParams) -> Result<(Vec<Interpretation>, u64), String> {
        self.find_by_filter_paginated(doc! {}, pagination).await
    }

    pub async fn find_by_filter_paginated(&self, filter: mongodb::bson::Document, pagination: crate::pagination::PaginationParams) -> Result<(Vec<Interpretation>, u64), String> {
        // Get total count
        let total = self.collection
            .count_documents(filter.clone(), None)
            .await
            .map_err(|e| e.to_string())?;

        // Get paginated results
        let options = mongodb::options::FindOptions::builder()
            .skip(pagination.skip())
            .limit(pagination.limit() as i64)
            .build();

        let cursor = self.collection
            .find(filter, options)
            .await
            .map_err(|e| e.to_string())?;

        let interpretations: Vec<Interpretation> = cursor.try_collect().await.map_err(|e| e.to_string())?;

        Ok((interpretations, total))
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<Interpretation>, String> {
        self.collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Interpretation>, String> {
        self.collection
            .find_one(doc! { "code": code }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn find_by_code_and_coding_code(&self, code: &str, coding_code: &str) -> Result<Option<Interpretation>, String> {
        self.collection
            .find_one(doc! { "code": code, "coding.code": coding_code }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update(&self, id: ObjectId, interpretation: Interpretation) -> Result<Interpretation, String> {
        let filter = doc! { "_id": id };
        let update = doc! {
            "$set": {
                "code": interpretation.code.clone(),
                "min": interpretation.min,
                "max": interpretation.max,
                "coding": {
                    "code": interpretation.coding.code.clone(),
                    "system": interpretation.coding.system.clone(),
                    "display": interpretation.coding.display.clone(),
                },
                "text": interpretation.text.clone(),
                "created_at": interpretation.created_at.clone(),
                "updated_at": interpretation.updated_at.clone(),
            }
        };

        self.collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| e.to_string())?;

        Ok(interpretation)
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
