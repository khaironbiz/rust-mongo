use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

// Helper to serialize Option<ObjectId> as Option<String> (hex)
fn serialize_oid_as_id<S>(oid: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match oid {
        Some(oid) => serializer.serialize_str(&oid.to_hex()),
        None => serializer.serialize_none(),
    }
}

// Helper to deserialize Option<String> as Option<ObjectId>
// Note: This is usually not needed because BSON reading handles ObjectId directly.
// But if we receive JSON with "id": "hex", we need to parse it.
// However, the input body usually doesn't have an ID (it's generated).
// For updates, the ID is in the path.
// But if it is in the body, it's a string.
// Let's implement a specific deserializer if needed, or just let BSON handle it naturally?
// BSON deserializer knows how to read ObjectId.
// If we send JSON {"id": "hexstring"}, default deserialization for ObjectId fails?
// Yes, it expects {"$oid": "..."} or extended JSON.
// So we need a custom deserializer for JSON string -> ObjectId.
#[allow(dead_code)]
fn deserialize_id_from_string<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            if s.is_empty() { return Ok(None); }
            ObjectId::parse_str(&s).map(Some).map_err(serde::de::Error::custom)
        },
        None => Ok(None),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalRecord {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub nrme: String,
    pub nik: String,
    pub name: String,
    pub dob: String,
    pub gender: String,
    pub hp: String,
    pub email: String,
    #[serde(rename = "lastVisitDate")]
    pub last_visit_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Doctor {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub nip: String,
    pub sip: String,
    pub specialization: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nurse {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub nip: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Medicine {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    #[serde(rename = "masterMedicineId")]
    pub master_medicine_id: String,
    #[serde(rename = "batchNumber")]
    pub batch_number: String,
    #[serde(rename = "tradeName")]
    pub trade_name: String,
    #[serde(rename = "productionDate")]
    pub production_date: String,
    #[serde(rename = "expiredDate")]
    pub expired_date: String,
    #[serde(rename = "purchasePrice")]
    pub purchase_price: f64,
    #[serde(rename = "sellingPrice")]
    pub selling_price: f64,
    pub qty: f64,
    pub manufacturer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Appointment {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    #[serde(rename = "patientId")]
    pub patient_id: String,
    #[serde(rename = "doctorId")]
    pub doctor_id: String,
    pub date: String,
    pub time: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub category: String,
    #[serde(rename = "subCategory")]
    pub sub_category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Insurance {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(rename = "type")]
    pub insurance_type: String,
    pub code: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub extension: String,
    pub size: u64,
    pub path: String,
    pub url: String,
    pub uploader: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub name: String,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "resetToken", skip_serializing_if = "Option::is_none")]
    pub reset_token: Option<String>,
    #[serde(rename = "resetTokenExpiry", skip_serializing_if = "Option::is_none")]
    pub reset_token_expiry: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleCategory {
    pub code: String,
    pub system: String,
    pub display: String,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub code: String,
    pub system: String,
    pub display: String,
    pub category: RoleCategory,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleEmbed {
    pub code: String,
    pub system: String,
    pub display: String,
    pub category: RoleCategory,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserName {
    #[serde(rename = "nama_depan")]
    pub nama_depan: String,
    #[serde(rename = "nama_belakang")]
    pub nama_belakang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContact {
    pub email: String,
    #[serde(rename = "nomor_telepon")]
    pub nomor_telepon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBirth {
    pub tempat: String,
    pub tanggal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEmbed {
    pub nama: UserName,
    pub nik: String,
    pub kontak: UserContact,
    pub lahir: UserBirth,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationEmbed {
    pub name: String,
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRole {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub role: RoleEmbed,
    pub user: UserEmbed,
    pub organisasi: OrganizationEmbed,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeCategoryEmbed {
    pub code: String,
    pub system: String,
    pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Code {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub code: String,
    pub display: String,
    pub system: String,
    pub category: CodeCategoryEmbed,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParentCodeEmbed {
    #[serde(rename = "code_id")] // Maps to code_id in JSON
    pub code_id: String,
    pub code: String,
    pub system: String,
    pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildCode {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub parent: ParentCodeEmbed,
    #[serde(rename = "code_id")]
    pub code_id: String, // Reference to the Child Code's original ID (from Code collection)
    pub code: String,
    pub system: String,
    pub display: String,
    pub norut: i32,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Region {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id_mongo: Option<ObjectId>,
    pub code: String,
    pub nama: String,
    pub wilayah: String,
    pub provinsi: String,
    pub kota: String,
    pub kecamatan: String,
    pub kelurahan: String,
    pub len: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterpretationCoding {
    pub code: String,
    pub system: String,
    pub display: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interpretation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub code: String,
    pub min: f64,
    pub max: f64,
    pub coding: InterpretationCoding,
    pub text: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KitOwner {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KitDistributor {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KitOperator {
    pub nik: String,
    pub id: String,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KitPasien {
    pub id_pasien: String,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kit {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub code: String,
    pub name: String,
    pub owner: KitOwner,
    pub distributor: KitDistributor,
    pub is_active: bool,
    pub operator: KitOperator,
    pub log_user_kit_id: String,
    pub order_id: String,
    pub pasien: KitPasien,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationUnit {
    pub code: String,
    pub display: String,
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationPasienNama {
    pub nama_depan: String,
    pub nama_belakang: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationPasienLahir {
    pub tempat: String,
    pub tanggal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationPasienUsia {
    pub tahun: i32,
    pub bulan: i32,
    pub hari: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationPasien {
    pub id: String,
    pub nama: ObservationPasienNama,
    pub gender: String,
    pub nik: String,
    pub lahir: ObservationPasienLahir,
    pub usia: ObservationPasienUsia,
    pub parent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationAtmSehatOwner {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationAtmSehat {
    pub code: String,
    pub name: String,
    pub owner: ObservationAtmSehatOwner,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationCoding {
    pub code: String,
    pub display: String,
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationCategory {
    pub code: String,
    pub display: String,
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationBaseLine {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationInterpretation {
    pub code: String,
    pub display: String,
    pub system: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Observation {
    #[serde(rename(serialize = "id", deserialize = "_id"), skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid_as_id")]
    pub id: Option<ObjectId>,
    pub value: f64,
    pub unit: ObservationUnit,
    pub id_pasien: String,
    pub pasien: ObservationPasien,
    pub id_petugas: String,
    pub atm_sehat: ObservationAtmSehat,
    pub time: i64,
    pub coding: ObservationCoding,
    pub category: ObservationCategory,
    pub base_line: ObservationBaseLine,
    pub interpretation: ObservationInterpretation,
    pub log_user_kit_id: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}
