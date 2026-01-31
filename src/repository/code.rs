use mongodb::{bson::doc, Database};
use futures_util::stream::TryStreamExt;
use crate::models::Code;

pub struct CodeRepository {
    db: Database,
}

impl CodeRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Code>, String> {
        let collection = self.db.collection::<Code>("codes");
        let cursor = collection.find(doc! {}, None).await.map_err(|e| e.to_string())?;
        let codes = cursor.try_collect().await.map_err(|e| e.to_string())?;
        Ok(codes)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Code>, String> {
        let collection = self.db.collection::<Code>("codes");
        collection.find_one(doc! { "code": code }, None).await.map_err(|e| e.to_string())
    }

    pub async fn find_by_id(&self, id: mongodb::bson::oid::ObjectId) -> Result<Option<Code>, String> {
        let collection = self.db.collection::<Code>("codes");
        collection.find_one(doc! { "_id": id }, None).await.map_err(|e| e.to_string())
    }

    pub async fn create(&self, mut code: Code) -> Result<Code, String> {
        let collection = self.db.collection::<Code>("codes");
        let result = collection.insert_one(code.clone(), None).await.map_err(|e| e.to_string())?;
        
        if code.id.is_none() {
             code.id = result.inserted_id.as_object_id();
        }
        Ok(code)
    }

    pub async fn update(&self, id: mongodb::bson::oid::ObjectId, code: Code) -> Result<Code, String> {
        let collection = self.db.collection::<Code>("codes");
        collection.replace_one(doc! { "_id": id }, code.clone(), None).await.map_err(|e| e.to_string())?;
        Ok(code)
    }

    pub async fn delete(&self, id: mongodb::bson::oid::ObjectId) -> Result<bool, String> {
        let collection = self.db.collection::<Code>("codes");
        let result = collection.delete_one(doc! { "_id": id }, None).await.map_err(|e| e.to_string())?;
        Ok(result.deleted_count > 0)
    }
}
