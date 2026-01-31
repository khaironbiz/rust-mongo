use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;
use crate::{
    models::{Role, RoleCategory},
    repository::RoleRepository,
    dto::role::{CreateRoleRequest, RoleResponse, UpdateRoleRequest, RoleCategoryDto},
    pagination::{PaginationParams, PaginatedResult, PaginationMeta},
};

pub struct RoleService {
    repo: RoleRepository,
}

impl RoleService {
    pub fn new(repo: RoleRepository) -> Self {
        RoleService { repo }
    }

    pub async fn get_all_paginated(&self, params: PaginationParams) -> Result<(Vec<RoleResponse>, PaginationMeta), String> {
        let limit = params.limit;
        let paginated_roles = self.repo.find_all_paginated(params).await
            .map_err(|e| e.to_string())?;

        let role_responses: Vec<RoleResponse> = paginated_roles.data.into_iter()
            .map(|role| self.map_to_response(role))
            .collect();

        let meta = PaginationMeta::new(
            paginated_roles.current_page,
            limit,
            paginated_roles.total_items
        );

        Ok((role_responses, meta))
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<RoleResponse>, String> {
        let role = self.repo.find_by_id(id).await.map_err(|e| e.to_string())?;
        Ok(role.map(|r| self.map_to_response(r)))
    }

    pub async fn create(&self, req: CreateRoleRequest) -> Result<(StatusCode, RoleResponse), (StatusCode, String)> {
        // Here you might want to check for duplicates (e.g. by code) if needed.
        
        let new_role = Role {
            id: None,
            code: req.code.clone(),
            system: req.system.clone(),
            display: req.display.clone(),
            category: RoleCategory {
                code: req.category.code,
                system: req.category.system,
                display: req.category.display,
                id: req.category.id,
            },
        };

        let result = self.repo.create(new_role).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok((StatusCode::CREATED, self.map_to_response(result)))
    }

    pub async fn update(&self, id: ObjectId, req: UpdateRoleRequest) -> Result<RoleResponse, (StatusCode, String)> {
        let existing = self.repo.find_by_id(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let match_role = existing.ok_or((StatusCode::NOT_FOUND, "Role not found".to_string()))?;

        let updated_role = Role {
            id: Some(id),
            code: req.code.unwrap_or(match_role.code),
            system: req.system.unwrap_or(match_role.system),
            display: req.display.unwrap_or(match_role.display),
            category: if let Some(cat) = req.category {
                RoleCategory {
                    code: cat.code,
                    system: cat.system,
                    display: cat.display,
                    id: cat.id,
                }
            } else {
                match_role.category
            },
        };

        let result = self.repo.update(id, updated_role).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        match result {
            Some(r) => Ok(self.map_to_response(r)),
            None => Err((StatusCode::NOT_FOUND, "Role not found".to_string())),
        }
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, (StatusCode, String)> {
        self.repo.delete(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }

    fn map_to_response(&self, role: Role) -> RoleResponse {
        RoleResponse {
            id: role.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            code: role.code,
            system: role.system,
            display: role.display,
            category: RoleCategoryDto {
                code: role.category.code,
                system: role.category.system,
                display: role.category.display,
                id: role.category.id,
            },
        }
    }
}
