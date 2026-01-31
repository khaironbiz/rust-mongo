use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use crate::models::ChildCode;
use futures_util::stream::TryStreamExt;

pub struct ChildCodeRepository {
    collection: Collection<ChildCode>,
}

impl ChildCodeRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection::<ChildCode>("child_codes");
        Self { collection }
    }

    pub async fn create(&self, child_code: ChildCode) -> Result<ChildCode, String> {
        let result = self
            .collection
            .insert_one(child_code.clone(), None)
            .await
            .map_err(|e| e.to_string())?;

        let mut created_child_code = child_code;
        created_child_code.id = result.inserted_id.as_object_id();

        Ok(created_child_code)
    }

    pub async fn find_all(&self) -> Result<Vec<ChildCode>, String> {
        let cursor = self
            .collection
            .find(None, None)
            .await
            .map_err(|e| e.to_string())?;

        let child_codes: Vec<ChildCode> = cursor.try_collect().await.map_err(|e| e.to_string())?;

        Ok(child_codes)
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<ChildCode>, String> {
        self.collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update(&self, id: ObjectId, child_code: ChildCode) -> Result<ChildCode, String> {
        let filter = doc! { "_id": id };
        let update = doc! {
            "$set": {
                "parent": {
                    "code_id": child_code.parent.code_id.clone(),
                    "code": child_code.parent.code.clone(),
                    "system": child_code.parent.system.clone(),
                    "display": child_code.parent.display.clone(),
                },
                "code_id": child_code.code_id.clone(),
                "code": child_code.code.clone(),
                "system": child_code.system.clone(),
                "display": child_code.display.clone(),
                "norut": child_code.norut,
                "updated_at": child_code.updated_at.clone(),
            }
        };

        self.collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| e.to_string())?;

        Ok(child_code)
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
