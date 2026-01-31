use std::sync::Arc;
use mongodb::bson::{oid::ObjectId, doc};
use crate::repository::RegionRepository;
use crate::models::Region;
use crate::dto::region::{CreateRegionRequest, UpdateRegionRequest};

pub struct RegionService {
    repo: Arc<RegionRepository>,
}

impl RegionService {
    pub fn new(repo: Arc<RegionRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, dto: CreateRegionRequest) -> Result<Region, String> {
        // Unique Code Check
        if self.repo.find_by_code(&dto.code).await?.is_some() {
            return Err("Region with this code already exists".to_string());
        }

        let region = Region {
            id_mongo: None,
            code: dto.code,
            nama: dto.nama,
            wilayah: dto.wilayah,
            provinsi: dto.provinsi,
            kota: dto.kota,
            kecamatan: dto.kecamatan,
            kelurahan: dto.kelurahan,
            len: dto.len,
        };

        self.repo.create(region).await
    }

    pub async fn get_all(&self) -> Result<Vec<Region>, String> {
        self.repo.find_all().await
    }

    pub async fn get_all_paginated(&self, params: crate::pagination::PaginationParams) -> Result<(Vec<Region>, crate::pagination::PaginationMeta), String> {
        let (regions, total) = self.repo.find_all_paginated(params.clone()).await?;
        let meta = crate::pagination::PaginationMeta::new(params.page, params.limit, total);
        Ok((regions, meta))
    }

    pub async fn get_by_provinsi_paginated(&self, provinsi: &str, params: crate::pagination::PaginationParams) -> Result<(Vec<Region>, crate::pagination::PaginationMeta), String> {
        let filter = doc! { "provinsi": provinsi };
        let (regions, total) = self.repo.find_by_filter_paginated(filter, params.clone()).await?;
        let meta = crate::pagination::PaginationMeta::new(params.page, params.limit, total);
        Ok((regions, meta))
    }

    pub async fn get_by_kota_paginated(&self, kota: &str, params: crate::pagination::PaginationParams) -> Result<(Vec<Region>, crate::pagination::PaginationMeta), String> {
        let filter = doc! { "kota": kota };
        let (regions, total) = self.repo.find_by_filter_paginated(filter, params.clone()).await?;
        let meta = crate::pagination::PaginationMeta::new(params.page, params.limit, total);
        Ok((regions, meta))
    }

    pub async fn get_by_kecamatan_paginated(&self, kecamatan: &str, params: crate::pagination::PaginationParams) -> Result<(Vec<Region>, crate::pagination::PaginationMeta), String> {
        let filter = doc! { "kecamatan": kecamatan };
        let (regions, total) = self.repo.find_by_filter_paginated(filter, params.clone()).await?;
        let meta = crate::pagination::PaginationMeta::new(params.page, params.limit, total);
        Ok((regions, meta))
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<Region>, String> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_by_code(&self, code: &str) -> Result<Option<Region>, String> {
        self.repo.find_by_code(code).await
    }

    pub async fn update(&self, id: ObjectId, dto: UpdateRegionRequest) -> Result<Region, String> {
        let mut existing = self.repo.find_by_id(id).await?
            .ok_or_else(|| "Region not found".to_string())?;

        if let Some(code) = dto.code {
            if code != existing.code {
                // Code is changing, check uniqueness
                if self.repo.find_by_code(&code).await?.is_some() {
                    return Err("Region with this code already exists".to_string());
                }
                existing.code = code;
            }
        }

        if let Some(nama) = dto.nama {
            existing.nama = nama;
        }

        if let Some(wilayah) = dto.wilayah {
            existing.wilayah = wilayah;
        }

        if let Some(provinsi) = dto.provinsi {
            existing.provinsi = provinsi;
        }

        if let Some(kota) = dto.kota {
            existing.kota = kota;
        }

        if let Some(kecamatan) = dto.kecamatan {
            existing.kecamatan = kecamatan;
        }

        if let Some(kelurahan) = dto.kelurahan {
            existing.kelurahan = kelurahan;
        }

        if let Some(len) = dto.len {
            existing.len = len;
        }

        self.repo.update(id, existing).await
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, String> {
        self.repo.delete(id).await
    }
}
