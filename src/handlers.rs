use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use mongodb::{bson::{doc, oid::ObjectId}, options::FindOptions};
use futures::TryStreamExt;
use std::sync::Arc;
use crate::{db::AppState, models::*};

// Generic error conversion could be improved, but for now specific handlers are fine.
// Helper to get collection
macro_rules! get_col {
    ($state:expr, $name:expr, $type:ty) => {
        $state.db.collection::<$type>($name)
    };
}

// --- Medical Records ---

pub async fn get_medical_records(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let collection = get_col!(state, "medical_records", MedicalRecord);
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let docs: Vec<MedicalRecord> = cursor.try_collect().await.unwrap_or_default();
            (StatusCode::OK, Json(docs)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn create_medical_record(State(state): State<Arc<AppState>>, Json(mut payload): Json<MedicalRecord>) -> impl IntoResponse {
    let collection = get_col!(state, "medical_records", MedicalRecord);
    // Ensure ID is None so Mongo generates it? Or allow user to set it?
    // Usually we let Mongo generate.
    if payload.id.is_none() {
        payload.id = Some(ObjectId::new());
    }
    
    match collection.insert_one(payload.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(payload)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert").into_response(),
    }
}

pub async fn get_medical_record(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    let collection = get_col!(state, "medical_records", MedicalRecord);
    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid ID").into_response(),
    };

    match collection.find_one(doc! { "_id": obj_id }).await {
        Ok(Some(doc)) => (StatusCode::OK, Json(doc)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn update_medical_record(State(state): State<Arc<AppState>>, Path(id): Path<String>, Json(payload): Json<MedicalRecord>) -> impl IntoResponse {
    let collection = get_col!(state, "medical_records", MedicalRecord);
    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid ID").into_response(),
    };

    // We replace the document entirely or update fields?
    // PUT usually means replace.
    // Spec says "Update a medical record". 
    // We will use replace_one or update_one.
    // If we use replace_one, we need to make sure the ID is preserved.
    let mut doc_to_update = payload.clone();
    doc_to_update.id = Some(obj_id); // Ensure ID matches path

    match collection.replace_one(doc! { "_id": obj_id }, doc_to_update.clone()).await {
        Ok(res) => {
            if res.matched_count == 0 {
                (StatusCode::NOT_FOUND, "Not found").into_response()
            } else {
                (StatusCode::OK, Json(doc_to_update)).into_response()
            }
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn delete_medical_record(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    let collection = get_col!(state, "medical_records", MedicalRecord);
    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid ID").into_response(),
    };

    match collection.delete_one(doc! { "_id": obj_id }).await {
        Ok(res) => {
            if res.deleted_count == 0 {
                (StatusCode::NOT_FOUND, "Not found").into_response()
            } else {
                (StatusCode::NO_CONTENT, ()).into_response()
            }
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

// --- Doctors ---

pub async fn get_doctors(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let collection = get_col!(state, "doctors", Doctor);
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let docs: Vec<Doctor> = cursor.try_collect().await.unwrap_or_default();
            (StatusCode::OK, Json(docs)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn create_doctor(State(state): State<Arc<AppState>>, Json(mut payload): Json<Doctor>) -> impl IntoResponse {
    let collection = get_col!(state, "doctors", Doctor);
    if payload.id.is_none() { payload.id = Some(ObjectId::new()); }
    match collection.insert_one(payload.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(payload)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert").into_response(),
    }
}

// --- Nurses ---

pub async fn get_nurses(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let collection = get_col!(state, "nurses", Nurse);
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let docs: Vec<Nurse> = cursor.try_collect().await.unwrap_or_default();
            (StatusCode::OK, Json(docs)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

// --- Medicines ---

pub async fn get_medicines(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let collection = get_col!(state, "medicines", Medicine);
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let docs: Vec<Medicine> = cursor.try_collect().await.unwrap_or_default();
            (StatusCode::OK, Json(docs)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

// --- Appointments ---

pub async fn get_appointments(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let collection = get_col!(state, "appointments", Appointment);
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let docs: Vec<Appointment> = cursor.try_collect().await.unwrap_or_default();
            (StatusCode::OK, Json(docs)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn create_appointment(State(state): State<Arc<AppState>>, Json(mut payload): Json<Appointment>) -> impl IntoResponse {
    let collection = get_col!(state, "appointments", Appointment);
    if payload.id.is_none() { payload.id = Some(ObjectId::new()); }
    match collection.insert_one(payload.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(payload)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert").into_response(),
    }
}

// --- Services ---

pub async fn get_services(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let collection = get_col!(state, "services", Service);
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let docs: Vec<Service> = cursor.try_collect().await.unwrap_or_default();
            (StatusCode::OK, Json(docs)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

// --- Insurances ---

pub async fn get_insurances(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let collection = get_col!(state, "insurances", Insurance);
    match collection.find(doc! {}).await {
        Ok(cursor) => {
            let docs: Vec<Insurance> = cursor.try_collect().await.unwrap_or_default();
            (StatusCode::OK, Json(docs)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}
