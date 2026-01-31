use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::Observation;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationUnitDto {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(length(min = 1))]
    pub display: String,
    #[validate(length(min = 1))]
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationPasienNamaDto {
    #[validate(length(min = 1))]
    pub nama_depan: String,
    #[validate(length(min = 1))]
    pub nama_belakang: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationPasienLahirDto {
    #[validate(length(min = 1))]
    pub tempat: String,
    #[validate(length(min = 1))]
    pub tanggal: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationPasienUsiaDto {
    pub tahun: i32,
    pub bulan: i32,
    pub hari: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationPasienDto {
    #[validate(length(min = 1))]
    pub id: String,
    pub nama: ObservationPasienNamaDto,
    #[validate(length(min = 1))]
    pub gender: String,
    #[validate(length(min = 1))]
    pub nik: String,
    pub lahir: ObservationPasienLahirDto,
    pub usia: ObservationPasienUsiaDto,
    pub parent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationAtmSehatOwnerDto {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationAtmSehatDto {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub owner: ObservationAtmSehatOwnerDto,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationCodingDto {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(length(min = 1))]
    pub display: String,
    #[validate(length(min = 1))]
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationCategoryDto {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(length(min = 1))]
    pub display: String,
    #[validate(length(min = 1))]
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationBaseLineDto {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ObservationInterpretationDto {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(length(min = 1))]
    pub display: String,
    #[validate(length(min = 1))]
    pub system: String,
    #[validate(length(min = 1))]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateObservationRequest {
    pub value: f64,
    pub unit: ObservationUnitDto,
    #[validate(length(min = 1))]
    pub id_pasien: String,
    pub pasien: ObservationPasienDto,
    #[validate(length(min = 1))]
    pub id_petugas: String,
    pub atm_sehat: ObservationAtmSehatDto,
    pub time: i64,
    pub coding: ObservationCodingDto,
    pub category: ObservationCategoryDto,
    pub base_line: ObservationBaseLineDto,
    pub interpretation: ObservationInterpretationDto,
    pub log_user_kit_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateObservationRequest {
    pub value: Option<f64>,
    pub unit: Option<ObservationUnitDto>,
    pub id_pasien: Option<String>,
    pub pasien: Option<ObservationPasienDto>,
    pub id_petugas: Option<String>,
    pub atm_sehat: Option<ObservationAtmSehatDto>,
    pub time: Option<i64>,
    pub coding: Option<ObservationCodingDto>,
    pub category: Option<ObservationCategoryDto>,
    pub base_line: Option<ObservationBaseLineDto>,
    pub interpretation: Option<ObservationInterpretationDto>,
    pub log_user_kit_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationResponse {
    pub id: String,
    pub value: f64,
    pub unit: ObservationUnitDto,
    pub id_pasien: String,
    pub pasien: ObservationPasienDto,
    pub id_petugas: String,
    pub atm_sehat: ObservationAtmSehatDto,
    pub time: i64,
    pub coding: ObservationCodingDto,
    pub category: ObservationCategoryDto,
    pub base_line: ObservationBaseLineDto,
    pub interpretation: ObservationInterpretationDto,
    pub log_user_kit_id: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
}

impl From<Observation> for ObservationResponse {
    fn from(obs: Observation) -> Self {
        Self {
            id: obs.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            value: obs.value,
            unit: ObservationUnitDto {
                code: obs.unit.code,
                display: obs.unit.display,
                system: obs.unit.system,
            },
            id_pasien: obs.id_pasien,
            pasien: ObservationPasienDto {
                id: obs.pasien.id,
                nama: ObservationPasienNamaDto {
                    nama_depan: obs.pasien.nama.nama_depan,
                    nama_belakang: obs.pasien.nama.nama_belakang,
                },
                gender: obs.pasien.gender,
                nik: obs.pasien.nik,
                lahir: ObservationPasienLahirDto {
                    tempat: obs.pasien.lahir.tempat,
                    tanggal: obs.pasien.lahir.tanggal,
                },
                usia: ObservationPasienUsiaDto {
                    tahun: obs.pasien.usia.tahun,
                    bulan: obs.pasien.usia.bulan,
                    hari: obs.pasien.usia.hari,
                },
                parent: obs.pasien.parent,
            },
            id_petugas: obs.id_petugas,
            atm_sehat: ObservationAtmSehatDto {
                code: obs.atm_sehat.code,
                name: obs.atm_sehat.name,
                owner: ObservationAtmSehatOwnerDto {
                    code: obs.atm_sehat.owner.code,
                    name: obs.atm_sehat.owner.name,
                },
            },
            time: obs.time,
            coding: ObservationCodingDto {
                code: obs.coding.code,
                display: obs.coding.display,
                system: obs.coding.system,
            },
            category: ObservationCategoryDto {
                code: obs.category.code,
                display: obs.category.display,
                system: obs.category.system,
            },
            base_line: ObservationBaseLineDto {
                min: obs.base_line.min,
                max: obs.base_line.max,
            },
            interpretation: ObservationInterpretationDto {
                code: obs.interpretation.code,
                display: obs.interpretation.display,
                system: obs.interpretation.system,
                text: obs.interpretation.text,
            },
            log_user_kit_id: obs.log_user_kit_id,
            updated_at: obs.updated_at,
            created_at: obs.created_at,
        }
    }
}
