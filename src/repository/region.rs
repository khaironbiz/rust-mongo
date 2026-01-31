use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use crate::models::Region;
use futures_util::stream::TryStreamExt;

pub struct RegionRepository {
    collection: Collection<Region>,
}

impl RegionRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection::<Region>("regions");
        Self { collection }
    }

    pub async fn create(&self, region: Region) -> Result<Region, String> {
        let result = self
            .collection
            .insert_one(region.clone(), None)
            .await
            .map_err(|e| e.to_string())?;

        let mut created_region = region;
        created_region.id_mongo = result.inserted_id.as_object_id();

        Ok(created_region)
    }

    pub async fn find_all(&self) -> Result<Vec<Region>, String> {
        let cursor = self
            .collection
            .find(None, None)
            .await
            .map_err(|e| e.to_string())?;

        let regions: Vec<Region> = cursor.try_collect().await.map_err(|e| e.to_string())?;

        Ok(regions)
    }

    pub async fn find_all_paginated(&self, pagination: crate::pagination::PaginationParams) -> Result<(Vec<Region>, u64), String> {
        self.find_by_filter_paginated(doc! {}, pagination).await
    }

    pub async fn find_by_filter_paginated(&self, filter: mongodb::bson::Document, pagination: crate::pagination::PaginationParams) -> Result<(Vec<Region>, u64), String> {
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

        let regions: Vec<Region> = cursor.try_collect().await.map_err(|e| e.to_string())?;

        Ok((regions, total))
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<Region>, String> {
        self.collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Region>, String> {
        self.collection
            .find_one(doc! { "code": code }, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update(&self, id: ObjectId, region: Region) -> Result<Region, String> {
        let filter = doc! { "_id": id };
        let update = doc! {
            "$set": {
                "code": region.code.clone(),
                "nama": region.nama.clone(),
                "wilayah": region.wilayah.clone(),
                "provinsi": region.provinsi.clone(),
                "kota": region.kota.clone(),
                "kecamatan": region.kecamatan.clone(),
                "kelurahan": region.kelurahan.clone(),
                "len": region.len.clone(),
            }
        };

        self.collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| e.to_string())?;

        Ok(region)
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
