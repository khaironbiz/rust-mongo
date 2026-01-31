# Repository Service Pattern Implementation Guide

## Quick Start

### Creating a New Entity with Repository Service Pattern

Let's use `Doctor` as an example.

#### 1. Model (Already exists in `models.rs`)
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Doctor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub specialization: String,
    pub nip: String, // Nomor Induk Profesional
    pub contact: String,
}
```

#### 2. Repository (`src/repository/doctor.rs`)
```rust
use mongodb::{bson::doc, Database};
use futures_util::stream::TryStreamExt;
use crate::models::Doctor;

pub struct DoctorRepository {
    db: Database,
}

impl DoctorRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // List all doctors
    pub async fn find_all(&self) -> Result<Vec<Doctor>, String> {
        let collection = self.db.collection::<Doctor>("doctors");
        match collection.find(doc! {}, None).await {
            Ok(cursor) => {
                cursor
                    .try_collect::<Vec<Doctor>>()
                    .await
                    .map_err(|e| format!("Failed to collect results: {}", e))
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    // Get doctor by ID
    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<Doctor>, String> {
        let collection = self.db.collection::<Doctor>("doctors");
        collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    // Insert new doctor
    pub async fn insert(&self, mut doctor: Doctor) -> Result<Doctor, String> {
        let collection = self.db.collection::<Doctor>("doctors");
        
        if doctor.id.is_none() {
            doctor.id = Some(ObjectId::new());
        }

        collection
            .insert_one(doctor.clone(), None)
            .await
            .map_err(|e| format!("Insert failed: {}", e))?;

        Ok(doctor)
    }
    
    // Add more methods as needed (update, delete, etc.)
}
```

#### 3. Service (`src/services/doctor_service.rs`)
```rust
use crate::models::Doctor;
use crate::repository::DoctorRepository;
use axum::http::StatusCode;

pub struct DoctorService {
    repository: DoctorRepository,
}

impl DoctorService {
    pub fn new(repository: DoctorRepository) -> Self {
        Self { repository }
    }

    // Get all doctors
    pub async fn get_all(&self) -> Result<Vec<Doctor>, (StatusCode, String)> {
        match self.repository.find_all().await {
            Ok(doctors) => Ok(doctors),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    // Get doctor by ID
    pub async fn get_by_id(&self, id: ObjectId) -> Result<Option<Doctor>, (StatusCode, String)> {
        match self.repository.find_by_id(id).await {
            Ok(doctor) => Ok(doctor),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }

    // Create new doctor with validation
    pub async fn create(&self, doctor: Doctor) -> Result<(StatusCode, Doctor), (StatusCode, String)> {
        // Add validation business logic here
        // For example: validate NIP format, check uniqueness, etc.
        
        // If validation needed:
        // if let Err(_) = validation::validate_nip(&doctor.nip) {
        //     return Err((StatusCode::BAD_REQUEST, "Invalid NIP format".to_string()));
        // }
        
        match self.repository.insert(doctor).await {
            Ok(created) => Ok((StatusCode::CREATED, created)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
        }
    }
}
```

#### 4. Handlers (`src/handlers.rs`)
```rust
pub async fn get_doctors(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Create repository and service
    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    // Call service
    match service.get_all().await {
        Ok(doctors) => (StatusCode::OK, Json(doctors)).into_response(),
        Err((status, msg)) => (status, Json(json!({"error": msg}))).into_response(),
    }
}

pub async fn get_doctor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return (StatusCode::BAD_REQUEST, "Invalid ID format").into_response();
    };

    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    match service.get_by_id(oid).await {
        Ok(Some(doctor)) => (StatusCode::OK, Json(doctor)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Doctor not found").into_response(),
        Err((status, msg)) => (status, Json(json!({"error": msg}))).into_response(),
    }
}

pub async fn create_doctor(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<Doctor>,
) -> impl IntoResponse {
    if payload.id.is_none() {
        payload.id = Some(ObjectId::new());
    }

    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    match service.create(payload).await {
        Ok((status, doctor)) => (status, Json(doctor)).into_response(),
        Err((status, msg)) => {
            let status_code: StatusCode = status;
            (status_code, Json(json!({"error": msg, "status": status_code.as_u16()}))).into_response()
        }
    }
}
```

#### 5. Routes (`src/routes.rs`)
```rust
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/doctors", get(get_doctors).post(create_doctor))
        .route("/doctors/:id", get(get_doctor))
        // ... other routes
        .with_state(state)
}
```

## Pattern Architecture

```
┌──────────────────────────────────────────────────────────┐
│                    HTTP Request                          │
└────────────────────────┬─────────────────────────────────┘
                         │
┌────────────────────────▼─────────────────────────────────┐
│               Handler Layer (handlers.rs)                 │
│  - Parse HTTP requests & path parameters                  │
│  - Create Repository instances                            │
│  - Create Service instances                               │
│  - Call Service methods                                   │
│  - Transform responses to HTTP responses                  │
└────────────────────────┬─────────────────────────────────┘
                         │
┌────────────────────────▼─────────────────────────────────┐
│             Service Layer (services/*.rs)                 │
│  - Business logic validation                              │
│  - Entity constraints checking                            │
│  - Cross-entity operations                                │
│  - Error handling with HTTP status codes                  │
│  - Database transaction coordination                      │
└────────────────────────┬─────────────────────────────────┘
                         │
┌────────────────────────▼─────────────────────────────────┐
│          Repository Layer (repository/*.rs)               │
│  - CRUD operations                                        │
│  - Direct database queries                                │
│  - Error handling to strings                              │
│  - Data transformation                                    │
└────────────────────────┬─────────────────────────────────┘
                         │
┌────────────────────────▼─────────────────────────────────┐
│              Database (MongoDB)                           │
└─────────────────────────────────────────────────────────┘
```

## Dependency Flow

```
Handlers → Services → Repositories → Database
  ↑          ↓          ↓
  └──────────┴──────────┘
        (owns instances)

No circular dependencies!
Each layer only depends on lower layers.
```

## Error Handling Strategy

### Repository Layer
- Returns `Result<T, String>` 
- Focus: Database operation errors
- Example: "Failed to insert document"

### Service Layer
- Returns `Result<T, (StatusCode, String)>`
- Focus: Business logic validation
- Example: (StatusCode::CONFLICT, "NIK already exists")

### Handler Layer
- Receives service result
- Transforms to HTTP response
- Returns (StatusCode, Json)

## Key Patterns

### 1. Validation at Service Layer
✅ CORRECT:
```rust
pub async fn create(&self, record: MedicalRecord) 
    -> Result<(StatusCode, MedicalRecord), (StatusCode, String)> {
    // Validate here
    if let Err(_) = validation::validate_nik(&record.nik) {
        return Err((StatusCode::BAD_REQUEST, "Invalid NIK"));
    }
    // Then call repository
    self.repository.insert(record).await...
}
```

❌ WRONG:
```rust
// Don't validate in handler
pub async fn create_medical_record(State(state): State<Arc<AppState>>, Json(record): Json<MedicalRecord>) {
    validation::validate_nik(&record.nik); // ← WRONG PLACE
    ...
}
```

### 2. Business Logic in Service
✅ CORRECT:
```rust
// Service: Check NIK uniqueness
pub async fn create(&self, record: MedicalRecord) -> Result<...> {
    if let Ok(Some(_)) = self.repository.find_by_nik(&record.nik).await {
        return Err((StatusCode::CONFLICT, "NIK already exists"));
    }
    ...
}
```

❌ WRONG:
```rust
// Handler: Don't put business logic here
pub async fn create_medical_record(...) {
    // Find duplicate check
    let repo = MedicalRecordRepository::new(...);
    if let Ok(Some(_)) = repo.find_by_nik(...).await { // ← WRONG PLACE
        ...
    }
}
```

### 3. Database Queries Only in Repository
✅ CORRECT:
```rust
// Repository: Pure database operations
pub async fn find_by_nik(&self, nik: &str) -> Result<Option<Doctor>, String> {
    let collection = self.db.collection::<Doctor>("doctors");
    collection.find_one(doc! { "nip": nik }, None).await...
}
```

❌ WRONG:
```rust
// Handler: Don't query database directly
pub async fn create_doctor(...) {
    // Get database directly
    let collection = state.db.collection::<Doctor>("doctors");
    collection.find_one(...).await // ← WRONG PLACE
}
```

## Testing with Repository Service Pattern

### Unit Test Service (Without Database)
```rust
#[tokio::test]
async fn test_create_duplicate_nik() {
    // Mock Repository
    struct MockMedicalRecordRepository {
        existing_niks: Vec<String>,
    }
    
    impl MockMedicalRecordRepository {
        pub async fn find_by_nik(&self, nik: &str) -> Result<Option<MedicalRecord>, String> {
            if self.existing_niks.contains(&nik.to_string()) {
                Ok(Some(MedicalRecord { /* ... */ }))
            } else {
                Ok(None)
            }
        }
        
        pub async fn insert(&self, record: MedicalRecord) 
            -> Result<MedicalRecord, String> {
            Ok(record)
        }
    }
    
    // Test Service with Mock
    let mock_repo = MockMedicalRecordRepository {
        existing_niks: vec!["1234567890123456".to_string()],
    };
    
    let service = MedicalRecordService::new(mock_repo);
    
    let record = MedicalRecord {
        id: None,
        nik: "1234567890123456".to_string(),
        // ...
    };
    
    let result = service.create(record).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, StatusCode::CONFLICT);
}
```

### Integration Test (With Database)
```rust
#[tokio::test]
async fn test_create_doctor_integration() {
    let state = db::init_db().await.unwrap();
    
    let repo = DoctorRepository::new(state.db.clone());
    let service = DoctorService::new(repo);
    
    let doctor = Doctor {
        id: None,
        name: "Dr. John".to_string(),
        specialization: "Cardiology".to_string(),
        nip: "1234567890".to_string(),
        contact: "08123456".to_string(),
    };
    
    let result = service.create(doctor).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, StatusCode::CREATED);
}
```

## Best Practices

1. **Repository**: Pure data access, no business logic
2. **Service**: Business logic, validation, coordination
3. **Handler**: HTTP parsing & response building only
4. **Errors**: Type-safe, meaningful messages
5. **Database**: Always through repository
6. **Validation**: In service, not in handler
7. **Testing**: Test service with mock repository
8. **Dependencies**: Always flow downward

## Advantages of This Pattern

| Aspect | Benefit |
|--------|---------|
| **Testing** | Easy to mock repositories for service unit tests |
| **Maintenance** | Changes isolated to specific layers |
| **Reusability** | Services can be used from multiple interfaces (HTTP, CLI, gRPC) |
| **Scalability** | Easy to add new entities following same pattern |
| **Readability** | Clear separation makes code easy to understand |
| **Flexibility** | Can swap implementations (MongoDB ↔ PostgreSQL) |

## Common Patterns to Follow

### Pattern 1: Create with Validation
```rust
// In Service
pub async fn create(&self, entity: Entity) -> Result<(StatusCode, Entity), (StatusCode, String)> {
    // Validate
    // Check uniqueness
    // Call repository.insert()
    // Return (StatusCode::CREATED, created_entity)
}
```

### Pattern 2: Update with Version Check
```rust
// In Service  
pub async fn update(&self, id: ObjectId, entity: Entity) 
    -> Result<Entity, (StatusCode, String)> {
    // Get existing entity
    // Validate changes
    // Call repository.update()
    // Return updated entity
}
```

### Pattern 3: Delete with Cascade
```rust
// In Service
pub async fn delete(&self, id: ObjectId) -> Result<bool, (StatusCode, String)> {
    // Check if deletable
    // Delete related entities if needed
    // Call repository.delete()
    // Return success
}
```

## File Organization Checklist

When adding a new entity:

- [ ] Add model in `src/models.rs`
- [ ] Create `src/repository/entity_name.rs` with Repository struct
- [ ] Create `src/services/entity_name_service.rs` with Service struct
- [ ] Update `src/repository/mod.rs` to export repository
- [ ] Update `src/services/mod.rs` to export service
- [ ] Add handlers in `src/handlers.rs`
- [ ] Add routes in `src/routes.rs`
- [ ] Add tests in `tests/api_tests.rs`
- [ ] Update `ARCHITECTURE.md` documentation

This structure ensures consistency across the entire codebase!
