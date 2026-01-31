use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use futures_util::stream::TryStreamExt;
use crate::models::Role;
use crate::pagination::{PaginationParams, PaginatedResult};

pub struct RoleRepository {
    collection: Collection<Role>,
}

impl RoleRepository {
    pub fn new(db: Database) -> Self {
        RoleRepository {
            collection: db.collection("roles"),
        }
    }

    pub async fn find_all_paginated(&self, params: PaginationParams) -> Result<PaginatedResult<Role>, mongodb::error::Error> {
        let page = params.page;
        let limit = params.limit;
        let skip = params.skip();

        let total_items = self.collection.count_documents(doc! {}, None).await?;
        let total_pages = (total_items as f64 / limit as f64).ceil() as u64;

        let find_options = mongodb::options::FindOptions::builder()
            .skip(skip)
            .limit(limit as i64)
            .build();

        let mut cursor = self.collection.find(doc! {}, find_options).await?;
        let mut data = Vec::new();

        while let Some(role) = cursor.try_next().await? {
            data.push(role);
        }

        Ok(PaginatedResult {
            data,
            total_items,
            total_pages,
            current_page: page,
        })
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<Role>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    pub async fn create(&self, role: Role) -> Result<Role, mongodb::error::Error> {
        let result = self.collection.insert_one(role.clone(), None).await?;
        let mut created_role = role;
        created_role.id = result.inserted_id.as_object_id();
        Ok(created_role)
    }

    pub async fn update(&self, id: ObjectId, role: Role) -> Result<Option<Role>, mongodb::error::Error> {
        // Convert Role to BSON document, excluding _id
        let mut role_doc = mongodb::bson::to_document(&role)?;
        role_doc.remove("_id");

        let update_result = self.collection.find_one_and_update(
            doc! { "_id": id },
            doc! { "$set": role_doc },
            None
        ).await?;

        if let Some(_) = update_result {
            // Return updated document (or just the local modified one)
            // find_one_and_update returns the OLD document by default unless ReturnDocument::After is used
            // Let's refetch or just modify the ID
            let mut updated_role = role;
            updated_role.id = Some(id);
            Ok(Some(updated_role))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, mongodb::error::Error> {
        let result = self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(result.deleted_count == 1)
    }
}
