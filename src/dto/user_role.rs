use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RoleEmbedDto {
    #[validate(length(min = 1, message = "Role Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "Role System is required"))]
    pub system: String,
    #[validate(length(min = 1, message = "Role Display is required"))]
    pub display: String,
    #[validate]
    pub category: crate::dto::role::RoleCategoryDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UserNameDto {
    #[serde(rename = "nama_depan")]
    #[validate(length(min = 1, message = "First name is required"))]
    pub nama_depan: String,
    #[serde(rename = "nama_belakang")]
    pub nama_belakang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UserContactDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[serde(rename = "nomor_telepon")]
    #[validate(length(min = 10, message = "Phone number too short"))]
    pub nomor_telepon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UserBirthDto {
    #[validate(length(min = 1, message = "Birth place is required"))]
    pub tempat: String,
    #[validate(length(min = 1, message = "Birth date is required"))]
    pub tanggal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UserEmbedDto {
    #[validate]
    pub nama: UserNameDto,
    #[validate(length(min = 16, max = 16, message = "NIK must be 16 chars"))]
    pub nik: String,
    #[validate]
    pub kontak: UserContactDto,
    #[validate]
    pub lahir: UserBirthDto,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct OrganizationEmbedDto {
    #[validate(length(min = 1, message = "Organization Name is required"))]
    pub name: String,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateUserRoleRequest {
    #[validate]
    pub role: RoleEmbedDto,
    #[validate]
    pub user: UserEmbedDto,
    #[validate]
    pub organisasi: OrganizationEmbedDto,
    #[serde(rename = "is_active")]
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateUserRoleRequest {
    #[validate]
    pub role: Option<RoleEmbedDto>,
    #[validate]
    pub user: Option<UserEmbedDto>,
    #[validate]
    pub organisasi: Option<OrganizationEmbedDto>,
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRoleResponse {
    pub id: String,
    pub role: RoleEmbedDto,
    pub user: UserEmbedDto,
    pub organisasi: OrganizationEmbedDto,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
}
