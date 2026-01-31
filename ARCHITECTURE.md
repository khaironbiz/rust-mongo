# Repository Service Pattern - Architecture Documentation

## Overview
Aplikasi telah direfactor menggunakan **Repository Service Pattern** yang merupakan arsitektur bersih dengan pemisahan concerns yang jelas:

```
HTTP Request → Handlers → Services → Repositories → Database
```

## Directory Structure

```
src/
├── repository/              # Data Access Layer
│   ├── mod.rs              # Exports all repositories
│   ├── medical_record.rs   # MedicalRecordRepository
│   ├── file.rs            # FileRepository
│   ├── doctor.rs          # DoctorRepository
│   ├── nurse.rs           # NurseRepository
│   ├── medicine.rs        # MedicineRepository
│   ├── appointment.rs     # AppointmentRepository
│   ├── service.rs         # ServiceRepository
│   └── insurance.rs       # InsuranceRepository
│
├── services/               # Business Logic Layer
│   ├── mod.rs              # Exports all services
│   ├── medical_record_service.rs
│   ├── file_service.rs
│   ├── doctor_service.rs
│   ├── nurse_service.rs
│   ├── medicine_service.rs
│   ├── appointment_service.rs
│   ├── service_service.rs
│   └── insurance_service.rs
│
├── handlers.rs             # HTTP Request Handlers (Simplified)
├── routes.rs              # Route definitions
├── models.rs              # Data models
├── validation.rs          # Input validation rules
├── db.rs                  # Database initialization
├── s3.rs                  # S3/Object storage client
├── docs.rs                # API documentation
└── main.rs                # Application entry point
```

## Layer Responsibilities

### 1. Repository Layer (`src/repository/`)
**Tanggung Jawab**: Data Access Object (DAO)
- Menangani semua database operations
- Fokus pada CRUD operations
- Mengembalikan hasil atau error messages

**Contoh - MedicalRecordRepository**:
```rust
pub async fn find_all(&self) -> Result<Vec<MedicalRecord>, String>
pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<MedicalRecord>, String>
pub async fn find_by_nik(&self, nik: &str) -> Result<Option<MedicalRecord>, String>
pub async fn insert(&self, record: MedicalRecord) -> Result<MedicalRecord, String>
pub async fn update(&self, id: ObjectId, record: MedicalRecord) -> Result<MedicalRecord, String>
pub async fn delete(&self, id: ObjectId) -> Result<bool, String>
```

### 2. Service Layer (`src/services/`)
**Tanggung Jawab**: Business Logic
- Validasi input
- Business rule enforcement
- Koordinasi antara repositories
- Error handling
- S3 integration (untuk FileService)

**Contoh - MedicalRecordService**:
```rust
pub async fn create(&self, record: MedicalRecord) 
    -> Result<(StatusCode, MedicalRecord), (StatusCode, String)> {
    // 1. Validate NIK format
    if let Err(_) = validation::validate_nik(&record.nik) {
        return Err((StatusCode::BAD_REQUEST, "Invalid NIK format".to_string()));
    }
    
    // 2. Check NIK uniqueness
    match self.repository.find_by_nik(&record.nik).await {
        Ok(Some(_)) => Err((StatusCode::CONFLICT, "NIK already exists")),
        Ok(None) => {}
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
    
    // 3. Insert record
    self.repository.insert(record).await
        .map(|created| (StatusCode::CREATED, created))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
```

### 3. Handler Layer (`src/handlers.rs`)
**Tanggung Jawab**: HTTP Request Handling
- Extract dan parse HTTP request
- Memanggil service layer
- Transform service responses menjadi HTTP responses
- Minimal business logic

**Contoh**:
```rust
pub async fn create_medical_record(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<MedicalRecord>,
) -> impl IntoResponse {
    if payload.id.is_none() {
        payload.id = Some(ObjectId::new());
    }

    let repo = MedicalRecordRepository::new(state.db.clone());
    let service = MedicalRecordService::new(repo);
    
    match service.create(payload).await {
        Ok((status, record)) => (status, Json(record)).into_response(),
        Err((status, msg)) => {
            let status_code: StatusCode = status;
            (status_code, Json(json!({"error": msg, "status": status_code.as_u16()}))).into_response()
        }
    }
}
```

## Benefits

### 1. **Separation of Concerns**
- Repository: Hanya database queries
- Service: Business logic dan validation
- Handler: HTTP dan request/response handling

### 2. **Testability**
- Mudah mock repository untuk unit test service
- Service logic tidak depend pada HTTP framework
- Bisa test business logic terpisah dari HTTP

### 3. **Reusability**
- Service bisa digunakan dari CLI, gRPC, atau HTTP handler
- Tidak terikat ke framework tertentu

### 4. **Maintainability**
- Changes di database query hanya affect repository
- Business rule changes hanya affect service
- HTTP logic changes hanya affect handler

### 5. **Scalability**
- Mudah menambah service baru
- Mudah menambah repository baru
- Struktur konsisten dan predictable

## Data Flow Example

### Create Medical Record
```
1. HTTP POST /medical-records dengan JSON payload
   ↓
2. Handler: create_medical_record()
   - Parse JSON → MedicalRecord
   - Generate ID jika belum ada
   - Call Service.create()
   ↓
3. Service: MedicalRecordService.create()
   - Validate NIK format
   - Check NIK uniqueness via Repository.find_by_nik()
   - Call Repository.insert()
   - Return (StatusCode, MedicalRecord)
   ↓
4. Repository: MedicalRecordRepository.insert()
   - Create MongoDB collection reference
   - Ensure ID exists
   - Execute MongoDB insert_one()
   - Return created record
   ↓
5. Handler Transform & Response
   - Convert service result ke HTTP response
   - Return 201 Created dengan record JSON
```

## Service Instances

Setiap handler membuat instance service baru:
```rust
let repo = MedicalRecordRepository::new(state.db.clone());
let service = MedicalRecordService::new(repo);
```

Ini bisa dioptimasi ke dependency injection di masa depan jika diperlukan.

## Error Handling

### Repository Level
- Returns `Result<T, String>` dengan database error messages

### Service Level
- Returns `Result<T, (StatusCode, String)>` dengan HTTP status codes
- Menambah business logic context ke errors

### Handler Level
- Match pada service result
- Transform ke HTTP response dengan appropriate status codes

## Next Steps untuk Improvement

1. **Dependency Injection**: Gunakan container untuk manage repositories dan services
2. **Caching Layer**: Tambah cache service di antara service dan repository
3. **Transaction Management**: Perlu transactional service untuk multi-repository operations
4. **Error Types**: Custom error type daripada String untuk better type safety
5. **Async Repository**: Implementasi repository trait untuk better abstraction
6. **Unit Tests**: Tambah comprehensive tests untuk service layer

## Testing Example

```rust
#[tokio::test]
async fn test_create_medical_record_duplicate_nik() {
    // Setup
    let db = create_test_db().await;
    let repo = MedicalRecordRepository::new(db);
    let service = MedicalRecordService::new(repo);
    
    // Create first record
    let record = MedicalRecord {
        id: None,
        nik: "1234567890123456".to_string(),
        ...
    };
    
    service.create(record.clone()).await.unwrap();
    
    // Try create duplicate
    let result = service.create(record).await;
    
    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::CONFLICT);
}
```
