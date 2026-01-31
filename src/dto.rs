use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ============================================================================
// MEDICAL RECORD DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMedicalRecordRequest {
    pub nik: String,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub phone: String,
    pub address: String,
    #[serde(default)]
    pub medical_history: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateMedicalRecordRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub age: Option<u32>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub medical_history: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalRecordResponse {
    pub id: String,
    pub nik: String,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub phone: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_history: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

// ============================================================================
// DOCTOR DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDoctorRequest {
    pub name: String,
    pub nip: String, // Indonesian doctor license number
    pub specialty: String,
    pub phone: String,
    pub email: String,
    #[serde(default)]
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateDoctorRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub specialty: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoctorResponse {
    pub id: String,
    pub name: String,
    pub nip: String,
    pub specialty: String,
    pub phone: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    pub created_at: String,
}

// ============================================================================
// NURSE DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateNurseRequest {
    pub name: String,
    pub license_number: String,
    pub phone: String,
    pub email: String,
    pub shift: String, // Morning, Afternoon, Night
    #[serde(default)]
    pub specialization: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateNurseRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub shift: Option<String>,
    #[serde(default)]
    pub specialization: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NurseResponse {
    pub id: String,
    pub name: String,
    pub license_number: String,
    pub phone: String,
    pub email: String,
    pub shift: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specialization: Option<String>,
    pub created_at: String,
}

// ============================================================================
// MEDICINE DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMedicineRequest {
    pub name: String,
    pub generic_name: String,
    pub dosage: String,
    pub unit: String,
    pub price: f64,
    pub manufacturer: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateMedicineRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub dosage: Option<String>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub manufacturer: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicineResponse {
    pub id: String,
    pub name: String,
    pub generic_name: String,
    pub dosage: String,
    pub unit: String,
    pub price: f64,
    pub manufacturer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
}

// ============================================================================
// APPOINTMENT DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAppointmentRequest {
    pub patient_id: String,
    pub doctor_id: String,
    pub appointment_date: String, // ISO 8601 format
    pub appointment_time: String,  // HH:MM format
    pub reason: String,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAppointmentRequest {
    #[serde(default)]
    pub appointment_date: Option<String>,
    #[serde(default)]
    pub appointment_time: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub status: Option<String>, // scheduled, completed, cancelled
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppointmentResponse {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub appointment_date: String,
    pub appointment_time: String,
    pub reason: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_at: String,
}

// ============================================================================
// SERVICE DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateServiceRequest {
    pub name: String,
    pub code: String,
    pub category: String,
    pub price: f64,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateServiceRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceResponse {
    pub id: String,
    pub name: String,
    pub code: String,
    pub category: String,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
}

// ============================================================================
// INSURANCE DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateInsuranceRequest {
    pub name: String,
    pub code: String,
    pub coverage_percentage: f32,
    pub max_claim_amount: f64,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateInsuranceRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub coverage_percentage: Option<f32>,
    #[serde(default)]
    pub max_claim_amount: Option<f64>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceResponse {
    pub id: String,
    pub name: String,
    pub code: String,
    pub coverage_percentage: f32,
    pub max_claim_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub created_at: String,
}

// ============================================================================
// FILE DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileResponse {
    pub id: String,
    pub name: String,
    pub file_type: String,
    pub extension: String,
    pub size: u64,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub uploader: String,
    pub created_at: String,
}

// ============================================================================
// COMMON DTOs
// ============================================================================

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct BulkDeleteRequest {
    pub ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BulkDeleteResponse {
    pub deleted_count: usize,
    pub failed_count: usize,
    pub deleted_ids: Vec<String>,
    pub failed_ids: Vec<String>,
}

// ============================================================================
// AUTH DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct ForgotPasswordResponse {
    pub success: bool,
    pub message: String,
    pub reset_token: String,
}

#[derive(Debug, Serialize)]
pub struct ResetPasswordResponse {
    pub success: bool,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_medical_record_request_serialization() {
        let json = r#"{
            "nik": "1234567890123456",
            "name": "John Doe",
            "age": 30,
            "gender": "M",
            "phone": "08123456789",
            "address": "123 Main St"
        }"#;

        let req: CreateMedicalRecordRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.nik, "1234567890123456");
        assert_eq!(req.name, "John Doe");
    }

    #[test]
    fn test_update_request_optional_fields() {
        let json = r#"{
            "name": "Jane Doe"
        }"#;

        let req: UpdateMedicalRecordRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, Some("Jane Doe".to_string()));
        assert_eq!(req.age, None);
    }
}
