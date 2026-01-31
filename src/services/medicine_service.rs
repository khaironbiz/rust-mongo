use crate::models::Medicine;
use crate::repository::MedicineRepository;
use crate::pagination::{PaginationParams, PaginationMeta};
use crate::dto::medicine::{CreateMedicineRequest, UpdateMedicineRequest, MedicineResponse};
use mongodb::bson::oid::ObjectId;
use axum::http::StatusCode;

pub struct MedicineService {
    repository: MedicineRepository,
}

impl MedicineService {
    pub fn new(repository: MedicineRepository) -> Self {
        Self { repository }
    }

    /// Map Medicine model to MedicineResponse DTO
    fn map_to_response(medicine: Medicine) -> MedicineResponse {
        MedicineResponse {
            id: medicine.id.map(|id| id.to_hex()).unwrap_or_default(),
            master_medicine_id: medicine.master_medicine_id,
            batch_number: medicine.batch_number,
            trade_name: medicine.trade_name,
            production_date: medicine.production_date,
            expired_date: medicine.expired_date,
            purchase_price: medicine.purchase_price,
            selling_price: medicine.selling_price,
            qty: medicine.qty,
            manufacturer: medicine.manufacturer,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<MedicineResponse>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(medicines) => Ok(medicines.into_iter().map(Self::map_to_response).collect()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_all_paginated(&self, pagination: PaginationParams) -> Result<(Vec<MedicineResponse>, PaginationMeta), (StatusCode, String)> {
        match self.repository.find_all_paginated(pagination.clone()).await {
            Ok((medicines, total)) => {
                let responses = medicines.into_iter().map(Self::map_to_response).collect();
                let meta = PaginationMeta::new(pagination.page, pagination.limit, total);
                Ok((responses, meta))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn create(&self, request: CreateMedicineRequest) -> Result<(StatusCode, MedicineResponse), (StatusCode, String)> {
        let medicine = Medicine {
            id: Some(ObjectId::new()),
            master_medicine_id: request.master_medicine_id,
            batch_number: request.batch_number,
            trade_name: request.trade_name,
            production_date: request.production_date,
            expired_date: request.expired_date,
            purchase_price: request.purchase_price,
            selling_price: request.selling_price,
            qty: request.qty,
            manufacturer: request.manufacturer,
        };

        match self.repository.insert(medicine).await {
            Ok(created) => Ok((StatusCode::CREATED, Self::map_to_response(created))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<MedicineResponse>, (StatusCode, String)> {
        match self.repository.find_by_id(id).await {
            Ok(Some(medicine)) => Ok(Some(Self::map_to_response(medicine))),
            Ok(None) => Ok(None),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn update(&self, id: ObjectId, request: UpdateMedicineRequest) -> Result<MedicineResponse, (StatusCode, String)> {
        let mut medicine = self.repository.find_by_id(id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
            .ok_or((StatusCode::NOT_FOUND, "Medicine not found".to_string()))?;

        if let Some(val) = request.master_medicine_id { medicine.master_medicine_id = val; }
        if let Some(val) = request.batch_number { medicine.batch_number = val; }
        if let Some(val) = request.trade_name { medicine.trade_name = val; }
        if let Some(val) = request.production_date { medicine.production_date = val; }
        if let Some(val) = request.expired_date { medicine.expired_date = val; }
        if let Some(val) = request.purchase_price { medicine.purchase_price = val; }
        if let Some(val) = request.selling_price { medicine.selling_price = val; }
        if let Some(val) = request.qty { medicine.qty = val; }
        if let Some(val) = request.manufacturer { medicine.manufacturer = val; }

        match self.repository.update(id, medicine).await {
            Ok(updated) => Ok(Self::map_to_response(updated)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    pub async fn delete(&self, id: ObjectId) -> Result<bool, (StatusCode, String)> {
        match self.repository.delete(id).await {
            Ok(deleted) => Ok(deleted),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
