use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use futures_util::stream::TryStreamExt;
use crate::models::UserRole;
use crate::pagination::{PaginationParams, PaginatedResult};

pub struct UserRoleRepository {
    collection: Collection<UserRole>,
}

impl UserRoleRepository {
    pub fn new(db: Database) -> Self {
        UserRoleRepository {
            collection: db.collection("user_roles"),
        }
    }

    pub async fn find_all_paginated(&self, params: PaginationParams) -> Result<PaginatedResult<UserRole>, mongodb::error::Error> {
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

        while let Some(user_role) = cursor.try_next().await? {
            data.push(user_role);
        }

        Ok(PaginatedResult {
            data,
            total_items,
            total_pages,
            current_page: page,
        })
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<UserRole>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    pub async fn create(&self, user_role: UserRole) -> Result<UserRole, mongodb::error::Error> {
        let result = self.collection.insert_one(user_role.clone(), None).await?;
        let mut created_user_role = user_role;
        created_user_role.id = result.inserted_id.as_object_id();
        Ok(created_user_role)
    }

    pub async fn update(&self, id: ObjectId, user_role: UserRole) -> Result<Option<UserRole>, mongodb::error::Error> {
        let mut doc = mongodb::bson::to_document(&user_role)?;
        doc.remove("_id");

        let update_result = self.collection.find_one_and_update(
            doc! { "_id": id },
            doc! { "$set": doc },
            None
        ).await?;

        if let Some(_) = update_result {
            let mut updated_item = user_role;
            updated_item.id = Some(id);
            Ok(Some(updated_item))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, mongodb::error::Error> {
        let result = self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(result.deleted_count == 1)
    }
}
