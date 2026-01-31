use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;
use chrono::Utc;
use crate::{
    models::{UserRole, RoleEmbed, UserEmbed, OrganizationEmbed, UserName, UserContact, UserBirth},
    repository::UserRoleRepository,
    dto::user_role::{CreateUserRoleRequest, UserRoleResponse, UpdateUserRoleRequest, RoleEmbedDto, UserEmbedDto, OrganizationEmbedDto},
    pagination::{PaginationParams, PaginationMeta},
    dto::role::RoleCategoryDto,
    dto::user_role::{UserNameDto, UserContactDto, UserBirthDto}
};

pub struct UserRoleService {
    repo: UserRoleRepository,
}

impl UserRoleService {
    pub fn new(repo: UserRoleRepository) -> Self {
        UserRoleService { repo }
    }

    pub async fn get_all_paginated(&self, params: PaginationParams) -> Result<(Vec<UserRoleResponse>, PaginationMeta), String> {
        let limit = params.limit;
        let paginated = self.repo.find_all_paginated(params).await
            .map_err(|e| e.to_string())?;

        let responses: Vec<UserRoleResponse> = paginated.data.into_iter()
            .map(|item| self.map_to_response(item))
            .collect();

        let meta = PaginationMeta::new(
            paginated.current_page,
            limit,
            paginated.total_items
        );

        Ok((responses, meta))
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<UserRoleResponse>, String> {
        let item = self.repo.find_by_id(id).await.map_err(|e| e.to_string())?;
        Ok(item.map(|i| self.map_to_response(i)))
    }

    pub async fn create(&self, req: CreateUserRoleRequest) -> Result<(StatusCode, UserRoleResponse), (StatusCode, String)> {
        let now = Utc::now().to_rfc3339();

        let new_user_role = UserRole {
            id: None,
            role: RoleEmbed {
                code: req.role.code,
                system: req.role.system,
                display: req.role.display,
                category: crate::models::RoleCategory {
                    code: req.role.category.code,
                    system: req.role.category.system,
                    display: req.role.category.display,
                    id: req.role.category.id,
                },
            },
            user: UserEmbed {
                nama: UserName {
                    nama_depan: req.user.nama.nama_depan,
                    nama_belakang: req.user.nama.nama_belakang,
                },
                nik: req.user.nik,
                kontak: UserContact {
                    email: req.user.kontak.email,
                    nomor_telepon: req.user.kontak.nomor_telepon,
                },
                lahir: UserBirth {
                    tempat: req.user.lahir.tempat,
                    tanggal: req.user.lahir.tanggal,
                },
                id: req.user.id,
            },
            organisasi: OrganizationEmbed {
                name: req.organisasi.name,
                id: req.organisasi.id,
            },
            is_active: req.is_active,
            updated_at: now.clone(),
            created_at: now,
        };

        let result = self.repo.create(new_user_role).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok((StatusCode::CREATED, self.map_to_response(result)))
    }

    pub async fn update(&self, id: ObjectId, req: UpdateUserRoleRequest) -> Result<UserRoleResponse, (StatusCode, String)> {
        let existing = self.repo.find_by_id(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let match_item = existing.ok_or((StatusCode::NOT_FOUND, "User Role not found".to_string()))?;

        let now = Utc::now().to_rfc3339();

        let updated_role = if let Some(r) = req.role {
            RoleEmbed {
                code: r.code,
                system: r.system,
                display: r.display,
                category: crate::models::RoleCategory {
                    code: r.category.code,
                    system: r.category.system,
                    display: r.category.display,
                    id: r.category.id,
                }
            }
        } else {
            match_item.role
        };

        let updated_user = if let Some(u) = req.user {
             UserEmbed {
                nama: UserName {
                    nama_depan: u.nama.nama_depan,
                    nama_belakang: u.nama.nama_belakang,
                },
                nik: u.nik,
                kontak: UserContact {
                    email: u.kontak.email,
                    nomor_telepon: u.kontak.nomor_telepon,
                },
                lahir: UserBirth {
                    tempat: u.lahir.tempat,
                    tanggal: u.lahir.tanggal,
                },
                id: u.id,
            }
        } else {
            match_item.user
        };

         let updated_org = if let Some(o) = req.organisasi {
            OrganizationEmbed {
                name: o.name,
                id: o.id,
            }
        } else {
            match_item.organisasi
        };

        let updated_item = UserRole {
            id: Some(id),
            role: updated_role,
            user: updated_user,
            organisasi: updated_org,
            is_active: req.is_active.unwrap_or(match_item.is_active),
            created_at: match_item.created_at,
            updated_at: now,
        };

        let result = self.repo.update(id, updated_item).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        match result {
            Some(r) => Ok(self.map_to_response(r)),
            None => Err((StatusCode::NOT_FOUND, "User Role not found".to_string())),
        }
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, (StatusCode, String)> {
        self.repo.delete(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }

    fn map_to_response(&self, item: UserRole) -> UserRoleResponse {
        UserRoleResponse {
            id: item.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            role: RoleEmbedDto {
                code: item.role.code,
                system: item.role.system,
                display: item.role.display,
                category: RoleCategoryDto {
                    code: item.role.category.code,
                    system: item.role.category.system,
                    display: item.role.category.display,
                    id: item.role.category.id,
                },
            },
            user: UserEmbedDto {
                nama: UserNameDto {
                     nama_depan: item.user.nama.nama_depan,
                     nama_belakang: item.user.nama.nama_belakang,
                },
                nik: item.user.nik,
                kontak: UserContactDto {
                     email: item.user.kontak.email,
                     nomor_telepon: item.user.kontak.nomor_telepon,
                },
                lahir: UserBirthDto {
                     tempat: item.user.lahir.tempat,
                     tanggal: item.user.lahir.tanggal,
                },
                id: item.user.id,
            },
            organisasi: OrganizationEmbedDto {
                name: item.organisasi.name,
                id: item.organisasi.id,
            },
            is_active: item.is_active,
            updated_at: item.updated_at,
            created_at: item.created_at,
        }
    }
}
