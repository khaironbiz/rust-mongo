use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMedicineRequest {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub purchase_price: Option<f64>,
    #[serde(default)]
    pub selling_price: Option<f64>,
    #[serde(default)]
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
