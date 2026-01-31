use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateRegionRequest {
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "Nama is required"))]
    pub nama: String,
    pub wilayah: String,
    pub provinsi: String,
    pub kota: String,
    pub kecamatan: String,
    pub kelurahan: String,
    pub len: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateRegionRequest {
    pub code: Option<String>,
    pub nama: Option<String>,
    pub wilayah: Option<String>,
    pub provinsi: Option<String>,
    pub kota: Option<String>,
    pub kecamatan: Option<String>,
    pub kelurahan: Option<String>,
    pub len: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegionResponse {
    pub id_mongo: String,
    pub code: String,
    pub nama: String,
    pub wilayah: String,
    pub provinsi: String,
    pub kota: String,
    pub kecamatan: String,
    pub kelurahan: String,
    pub len: String,
}
