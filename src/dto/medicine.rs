use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateMedicineRequest {
    #[validate(length(min = 1, message = "Master Medicine ID is required"))]
    pub master_medicine_id: String,
    #[validate(length(min = 1, message = "Batch number is required"))]
    pub batch_number: String,
    #[validate(length(min = 1, message = "Trade name is required"))]
    pub trade_name: String,
    #[validate(length(min = 1, message = "Production date is required"))]
    pub production_date: String,
    #[validate(length(min = 1, message = "Expired date is required"))]
    pub expired_date: String,
    #[validate(range(min = 0.0, message = "Purchase price cannot be negative"))]
    pub purchase_price: f64,
    #[validate(range(min = 0.0, message = "Selling price cannot be negative"))]
    pub selling_price: f64,
    #[validate(range(min = 0.0, message = "Quantity cannot be negative"))]
    pub qty: f64,
    #[validate(length(min = 1, message = "Manufacturer is required"))]
    pub manufacturer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateMedicineRequest {
    #[serde(default)]
    pub master_medicine_id: Option<String>,
    #[serde(default)]
    pub production_date: Option<String>,
    #[serde(default)]
    pub batch_number: Option<String>,
    #[serde(default)]
    pub trade_name: Option<String>,
    #[serde(default)]
    pub expired_date: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0.0, message = "Purchase price cannot be negative"))]
    pub purchase_price: Option<f64>,
    #[serde(default)]
    #[validate(range(min = 0.0, message = "Selling price cannot be negative"))]
    pub selling_price: Option<f64>,
    #[serde(default)]
    #[validate(range(min = 0.0, message = "Quantity cannot be negative"))]
    pub qty: Option<f64>,
    #[serde(default)]
    pub manufacturer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicineResponse {
    pub id: String,
    pub master_medicine_id: String,
    pub batch_number: String,
    pub trade_name: String,
    pub production_date: String,
    pub expired_date: String,
    pub purchase_price: f64,
    pub selling_price: f64,
    pub qty: f64,
    pub manufacturer: String,
}
