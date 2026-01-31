use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use chrono::Local;
use crate::repository::KitRepository;
use crate::models::{Kit, KitOwner, KitDistributor, KitOperator, KitPasien};
use crate::dto::kit::{CreateKitRequest, UpdateKitRequest, KitResponse, KitOwnerDto, KitDistributorDto, KitOperatorDto, KitPasienDto};

pub struct KitService {
    repo: Arc<KitRepository>,
}

impl KitService {
    pub fn new(repo: Arc<KitRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, dto: CreateKitRequest) -> Result<Kit, String> {
        // Unique Code Check
        if self.repo.find_by_code(&dto.code).await?.is_some() {
            return Err("Kit with this code already exists".to_string());
        }

        let kit = Kit {
            id: None,
            code: dto.code,
            name: dto.name,
            owner: KitOwner {
                code: dto.owner.code,
                name: dto.owner.name,
            },
            distributor: KitDistributor {
                code: dto.distributor.code,
                name: dto.distributor.name,
            },
            is_active: dto.is_active,
            operator: KitOperator {
                nik: dto.operator.nik,
                id: dto.operator.id,
                time: dto.operator.time,
            },
            log_user_kit_id: dto.log_user_kit_id,
            order_id: dto.order_id,
            pasien: KitPasien {
                id_pasien: dto.pasien.id_pasien,
                time: dto.pasien.time,
            },
            created_at: Local::now().to_rfc3339(),
            updated_at: Some(Local::now().to_rfc3339()),
        };

        self.repo.create(kit).await
    }

    pub async fn get_all(&self) -> Result<Vec<KitResponse>, String> {
        let kits = self.repo.find_all().await?;
        Ok(kits.into_iter().map(Self::map_to_response).collect())
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<KitResponse>, String> {
        let kit = self.repo.find_by_id(id).await?;
        Ok(kit.map(Self::map_to_response))
    }

    pub async fn update(&self, id: ObjectId, dto: UpdateKitRequest) -> Result<KitResponse, String> {
        let mut existing = self.repo.find_by_id(id).await?
            .ok_or_else(|| "Kit not found".to_string())?;

        if let Some(code) = dto.code {
            if code != existing.code {
                if self.repo.find_by_code(&code).await?.is_some() {
                    return Err("Kit with this code already exists".to_string());
                }
                existing.code = code;
            }
        }

        if let Some(name) = dto.name {
            existing.name = name;
        }

        if let Some(owner) = dto.owner {
            existing.owner = KitOwner {
                code: owner.code,
                name: owner.name,
            };
        }

        if let Some(distributor) = dto.distributor {
            existing.distributor = KitDistributor {
                code: distributor.code,
                name: distributor.name,
            };
        }

        if let Some(is_active) = dto.is_active {
            existing.is_active = is_active;
        }

        if let Some(operator) = dto.operator {
            existing.operator = KitOperator {
                nik: operator.nik,
                id: operator.id,
                time: operator.time,
            };
        }

        if let Some(log_user_kit_id) = dto.log_user_kit_id {
            existing.log_user_kit_id = log_user_kit_id;
        }

        if let Some(order_id) = dto.order_id {
            existing.order_id = order_id;
        }

        if let Some(pasien) = dto.pasien {
            existing.pasien = KitPasien {
                id_pasien: pasien.id_pasien,
                time: pasien.time,
            };
        }

        existing.updated_at = Some(Local::now().to_rfc3339());

        let updated = self.repo.update(id, existing).await?;
        Ok(Self::map_to_response(updated))
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, String> {
        self.repo.delete(id).await
    }

    fn map_to_response(kit: Kit) -> KitResponse {
        KitResponse {
            id: kit.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            code: kit.code,
            name: kit.name,
            owner: KitOwnerDto {
                code: kit.owner.code,
                name: kit.owner.name,
            },
            distributor: KitDistributorDto {
                code: kit.distributor.code,
                name: kit.distributor.name,
            },
            is_active: kit.is_active,
            operator: KitOperatorDto {
                nik: kit.operator.nik,
                id: kit.operator.id,
                time: kit.operator.time,
            },
            log_user_kit_id: kit.log_user_kit_id,
            order_id: kit.order_id,
            pasien: KitPasienDto {
                id_pasien: kit.pasien.id_pasien,
                time: kit.pasien.time,
            },
            created_at: kit.created_at,
            updated_at: kit.updated_at,
        }
    }
}
