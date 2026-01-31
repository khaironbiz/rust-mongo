use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use crate::models::Kit;
use futures_util::stream::TryStreamExt;

pub struct KitRepository {
    collection: Collection<Kit>,
}

impl KitRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection::<Kit>("kits");
        Self { collection }
    }

    pub async fn create(&self, kit: Kit) -> Result<Kit, String> {
        let result = self
            .collection
            .insert_one(kit.clone(), None)
            .await
            .map_err(|e| e.to_string())?;

        let mut created_kit = kit;
        created_kit.id = result.inserted_id.as_object_id();

        Ok(created_kit)
    }

    pub async fn find_all(&self) -> Result<Vec<Kit>, String> {
        let cursor = self
            .collection
            .find(None, None)
            .await
            .map_err(|e| e.to_string())?;

        let kits: Vec<Kit> = cursor.try_collect().await.map_err(|e| e.to_string())?;

        Ok(kits)
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<Kit>, String> {
        self.collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Kit>, String> {
        self.collection
            .find_one(doc! { "code": code }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update(&self, id: ObjectId, kit: Kit) -> Result<Kit, String> {
        let filter = doc! { "_id": id };
        let update = doc! {
            "$set": {
                "code": kit.code.clone(),
                "name": kit.name.clone(),
                "owner": {
                    "code": kit.owner.code.clone(),
                    "name": kit.owner.name.clone(),
                },
                "distributor": {
                    "code": kit.distributor.code.clone(),
                    "name": kit.distributor.name.clone(),
                },
                "is_active": kit.is_active,
                "operator": {
                    "nik": kit.operator.nik.clone(),
                    "id": kit.operator.id.clone(),
                    "time": kit.operator.time,
                },
                "log_user_kit_id": kit.log_user_kit_id.clone(),
                "order_id": kit.order_id.clone(),
                "pasien": {
                    "id_pasien": kit.pasien.id_pasien.clone(),
                    "time": kit.pasien.time,
                },
                "updated_at": kit.updated_at.clone(),
            }
        };

        self.collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| e.to_string())?;

        Ok(kit)
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
