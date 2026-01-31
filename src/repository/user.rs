use mongodb::{bson::doc, Database, options::FindOptions};
use futures_util::stream::TryStreamExt;
use crate::models::User;
use crate::pagination::PaginationParams;

pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<User>, u64), String> {
        let collection = self.db.collection::<User>("users");
        
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
                let users = cursor
                    .try_collect::<Vec<User>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))?;
                Ok((users, total))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let collection = self.db.collection::<User>("users");
        collection
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn find_by_id(&self, id: mongodb::bson::oid::ObjectId) -> Result<Option<User>, String> {
        let collection = self.db.collection::<User>("users");
        collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn find_by_reset_token(&self, token: &str) -> Result<Option<User>, String> {
        let collection = self.db.collection::<User>("users");
        collection
            .find_one(doc! { "resetToken": token }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn insert(&self, mut user: User) -> Result<User, String> {
        let collection = self.db.collection::<User>("users");
        
        if user.id.is_none() {
            user.id = Some(mongodb::bson::oid::ObjectId::new());
        }

        collection
            .insert_one(user.clone(), None)
            .await
            .map_err(|e| format!("Insert failed: {}", e))?;

        Ok(user)
    }

    pub async fn update(&self, id: mongodb::bson::oid::ObjectId, user: User) -> Result<User, String> {
        let collection = self.db.collection::<User>("users");
        
        collection
            .replace_one(doc! { "_id": id }, user.clone(), None)
            .await
            .map_err(|e| format!("Update failed: {}", e))?;

        Ok(user)
    }

    pub async fn update_reset_token(&self, id: mongodb::bson::oid::ObjectId, token: Option<String>, expiry: Option<String>) -> Result<bool, String> {
        let collection = self.db.collection::<User>("users");
        
        let update = doc! {
            "$set": {
                "resetToken": token,
                "resetTokenExpiry": expiry,
                "updatedAt": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            }
        };

        let result = collection
            .update_one(doc! { "_id": id }, update, None)
            .await
            .map_err(|e| format!("Update failed: {}", e))?;

        Ok(result.modified_count > 0)
    }

    pub async fn update_password(&self, id: mongodb::bson::oid::ObjectId, password_hash: &str) -> Result<bool, String> {
        let collection = self.db.collection::<User>("users");
        
        let update = doc! {
            "$set": {
                "password": password_hash,
                "resetToken": null,
                "resetTokenExpiry": null,
                "updatedAt": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            }
        };

        let result = collection
            .update_one(doc! { "_id": id }, update, None)
            .await
            .map_err(|e| format!("Update failed: {}", e))?;

        Ok(result.modified_count > 0)
    }

    pub async fn update_refresh_token(&self, id: mongodb::bson::oid::ObjectId, refresh_token: Option<String>) -> Result<bool, String> {
        let collection = self.db.collection::<User>("users");
        
        let update = doc! {
            "$set": {
                "refreshToken": refresh_token,
                "updatedAt": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            }
        };

        let result = collection
            .update_one(doc! { "_id": id }, update, None)
            .await
            .map_err(|e| format!("Update failed: {}", e))?;

        Ok(result.modified_count > 0)
    }

    pub async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Option<User>, String> {
        let collection = self.db.collection::<User>("users");
        collection
            .find_one(doc! { "refreshToken": refresh_token }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn delete(&self, id: mongodb::bson::oid::ObjectId) -> Result<bool, String> {
        let collection = self.db.collection::<User>("users");
        
        let result = collection
            .delete_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Delete failed: {}", e))?;

        Ok(result.deleted_count > 0)
    }
}
