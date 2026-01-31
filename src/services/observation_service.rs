use mongodb::bson::oid::ObjectId;
use chrono::Utc;
use crate::models::{
    Observation, ObservationUnit, ObservationPasien, ObservationPasienNama,
    ObservationPasienLahir, ObservationPasienUsia, ObservationAtmSehat,
    ObservationAtmSehatOwner, ObservationCoding, ObservationCategory,
    ObservationBaseLine, ObservationInterpretation
};
use crate::repository::ObservationRepository;
use crate::dto::observation::{CreateObservationRequest, UpdateObservationRequest, ObservationResponse};
use crate::pagination::PaginationParams;

pub struct ObservationService {
    repository: ObservationRepository,
}

impl ObservationService {
    pub fn new(repository: ObservationRepository) -> Self {
        Self { repository }
    }

    pub async fn create_observation(&self, req: CreateObservationRequest) -> Result<ObservationResponse, String> {
        let now = Utc::now().to_rfc3339();
        
        let observation = Observation {
            id: None,
            value: req.value,
            unit: ObservationUnit {
                code: req.unit.code,
                display: req.unit.display,
                system: req.unit.system,
            },
            id_pasien: req.id_pasien,
            pasien: ObservationPasien {
                id: req.pasien.id,
                nama: ObservationPasienNama {
                    nama_depan: req.pasien.nama.nama_depan,
                    nama_belakang: req.pasien.nama.nama_belakang,
                },
                gender: req.pasien.gender,
                nik: req.pasien.nik,
                lahir: ObservationPasienLahir {
                    tempat: req.pasien.lahir.tempat,
                    tanggal: req.pasien.lahir.tanggal,
                },
                usia: ObservationPasienUsia {
                    tahun: req.pasien.usia.tahun,
                    bulan: req.pasien.usia.bulan,
                    hari: req.pasien.usia.hari,
                },
                parent: req.pasien.parent,
            },
            id_petugas: req.id_petugas,
            atm_sehat: ObservationAtmSehat {
                code: req.atm_sehat.code,
                name: req.atm_sehat.name,
                owner: ObservationAtmSehatOwner {
                    code: req.atm_sehat.owner.code,
                    name: req.atm_sehat.owner.name,
                },
            },
            time: req.time,
            coding: ObservationCoding {
                code: req.coding.code,
                display: req.coding.display,
                system: req.coding.system,
            },
            category: ObservationCategory {
                code: req.category.code,
                display: req.category.display,
                system: req.category.system,
            },
            base_line: ObservationBaseLine {
                min: req.base_line.min,
                max: req.base_line.max,
            },
            interpretation: ObservationInterpretation {
                code: req.interpretation.code,
                display: req.interpretation.display,
                system: req.interpretation.system,
                text: req.interpretation.text,
            },
            log_user_kit_id: req.log_user_kit_id,
            created_at: Some(now.clone()),
            updated_at: Some(now),
        };

        let created = self.repository.create(observation).await?;
        Ok(ObservationResponse::from(created))
    }

    pub async fn get_observations(&self, pagination: PaginationParams) -> Result<(Vec<ObservationResponse>, u64), String> {
        let (observations, total) = self.repository.find_all_paginated(pagination).await?;
        let responses = observations.into_iter().map(ObservationResponse::from).collect();
        Ok((responses, total))
    }

    pub async fn get_observation_by_id(&self, id: &str) -> Result<Option<ObservationResponse>, String> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
        let observation = self.repository.find_by_id(obj_id).await?;
        Ok(observation.map(ObservationResponse::from))
    }

    pub async fn update_observation(&self, id: &str, req: UpdateObservationRequest) -> Result<ObservationResponse, String> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
        
        let mut observation = self.repository.find_by_id(obj_id).await?
            .ok_or_else(|| "Observation not found".to_string())?;

        if let Some(v) = req.value { observation.value = v; }
        if let Some(u) = req.unit {
            observation.unit.code = u.code;
            observation.unit.display = u.display;
            observation.unit.system = u.system;
        }
        if let Some(ip) = req.id_pasien { observation.id_pasien = ip; }
        if let Some(p) = req.pasien {
            observation.pasien.id = p.id;
            observation.pasien.nama.nama_depan = p.nama.nama_depan;
            observation.pasien.nama.nama_belakang = p.nama.nama_belakang;
            observation.pasien.gender = p.gender;
            observation.pasien.nik = p.nik;
            observation.pasien.lahir.tempat = p.lahir.tempat;
            observation.pasien.lahir.tanggal = p.lahir.tanggal;
            observation.pasien.usia.tahun = p.usia.tahun;
            observation.pasien.usia.bulan = p.usia.bulan;
            observation.pasien.usia.hari = p.usia.hari;
            observation.pasien.parent = p.parent;
        }
        if let Some(ip) = req.id_petugas { observation.id_petugas = ip; }
        if let Some(a) = req.atm_sehat {
            observation.atm_sehat.code = a.code;
            observation.atm_sehat.name = a.name;
            observation.atm_sehat.owner.code = a.owner.code;
            observation.atm_sehat.owner.name = a.owner.name;
        }
        if let Some(t) = req.time { observation.time = t; }
        if let Some(c) = req.coding {
            observation.coding.code = c.code;
            observation.coding.display = c.display;
            observation.coding.system = c.system;
        }
        if let Some(c) = req.category {
            observation.category.code = c.code;
            observation.category.display = c.display;
            observation.category.system = c.system;
        }
        if let Some(b) = req.base_line {
            observation.base_line.min = b.min;
            observation.base_line.max = b.max;
        }
        if let Some(i) = req.interpretation {
            observation.interpretation.code = i.code;
            observation.interpretation.display = i.display;
            observation.interpretation.system = i.system;
            observation.interpretation.text = i.text;
        }
        if req.log_user_kit_id.is_some() {
            observation.log_user_kit_id = req.log_user_kit_id;
        }

        observation.updated_at = Some(Utc::now().to_rfc3339());

        let updated = self.repository.update(obj_id, observation).await?;
        Ok(ObservationResponse::from(updated))
    }

    pub async fn delete_observation(&self, id: &str) -> Result<bool, String> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| "Invalid ID format".to_string())?;
        self.repository.delete(obj_id).await
    }
}
