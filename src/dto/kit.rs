use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct KitOwnerDto {
    #[validate(length(min = 1, message = "Owner code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "Owner name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct KitDistributorDto {
    #[validate(length(min = 1, message = "Distributor code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "Distributor name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct KitOperatorDto {
    #[validate(length(min = 1, message = "Operator NIK is required"))]
    pub nik: String,
    #[validate(length(min = 1, message = "Operator ID is required"))]
    pub id: String,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct KitPasienDto {
    #[validate(length(min = 1, message = "Pasien ID is required"))]
    pub id_pasien: String,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateKitRequest {
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate]
    pub owner: KitOwnerDto,
    #[validate]
    pub distributor: KitDistributorDto,
    pub is_active: bool,
    #[validate]
    pub operator: KitOperatorDto,
    pub log_user_kit_id: String,
    pub order_id: String,
    #[validate]
    pub pasien: KitPasienDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateKitRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub owner: Option<KitOwnerDto>,
    pub distributor: Option<KitDistributorDto>,
    pub is_active: Option<bool>,
    pub operator: Option<KitOperatorDto>,
    pub log_user_kit_id: Option<String>,
    pub order_id: Option<String>,
    pub pasien: Option<KitPasienDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KitResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub owner: KitOwnerDto,
    pub distributor: KitDistributorDto,
    pub is_active: bool,
    pub operator: KitOperatorDto,
    pub log_user_kit_id: String,
    pub order_id: String,
    pub pasien: KitPasienDto,
    pub created_at: String,
    pub updated_at: Option<String>,
}
