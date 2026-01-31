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
