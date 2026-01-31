# Repository Service Pattern - Visual Diagrams

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                          CLIENT                                   │
│                     (HTTP Client / cURL)                          │
└────────────────────────────┬─────────────────────────────────────┘
                             │
                    GET /doctors
                    POST /medical-records
                    POST /files
                             │
┌────────────────────────────▼─────────────────────────────────────┐
│                    HTTP SERVER (Axum)                             │
└────────────────────────────┬─────────────────────────────────────┘
                             │
                    ┌────────▼────────┐
                    │    Router       │
                    └────────┬────────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
   ┌────────▼────────┐ ┌────▼─────────┐ ┌───▼──────────┐
   │ /doctors routes │ │ /files routes│ │ /medical-    │
   │                 │ │              │ │ records      │
   └────────┬────────┘ └────┬─────────┘ └───┬──────────┘
            │                │                │
   ┌────────▼────────────────▼────────────────▼────────────┐
   │                                                        │
   │              HANDLER LAYER                           │
   │  (HTTP Parsing & Coordination)                       │
   │                                                        │
   │  get_doctors()          get_files()                  │
   │  create_doctor()        create_file()                │
   │  get_medical_records()  delete_file()                │
   │  create_medical_record()                             │
   │                                                        │
   └────────┬──────────────────────────────────────────────┘
            │
            │ Creates instances of
            │
   ┌────────▼──────────────────────────────────────────────┐
   │                                                        │
   │              SERVICE LAYER                           │
   │  (Business Logic & Validation)                       │
   │                                                        │
   │  ┌────────────────────────────────────────────────┐  │
   │  │ MedicalRecordService                           │  │
   │  │ - Validate NIK format                          │  │
   │  │ - Check NIK uniqueness                         │  │
   │  │ - Coordinate with Repository                  │  │
   │  │ - Return StatusCode + Result                  │  │
   │  └────────────────────────────────────────────────┘  │
   │                                                        │
   │  ┌────────────────────────────────────────────────┐  │
   │  │ FileService                                    │  │
   │  │ - Validate file size & type                    │  │
   │  │ - Upload to S3 (NEO)                          │  │
   │  │ - Call FileRepository                         │  │
   │  │ - Handle S3 cleanup on delete                 │  │
   │  └────────────────────────────────────────────────┘  │
   │                                                        │
   │  ┌────────────────────────────────────────────────┐  │
   │  │ DoctorService, AppointmentService, etc...     │  │
   │  │ - Business logic specific to entity           │  │
   │  │ - Delegate to Repository                      │  │
   │  └────────────────────────────────────────────────┘  │
   │                                                        │
   └────────┬──────────────────────────────────────────────┘
            │
            │ Uses
            │
   ┌────────▼──────────────────────────────────────────────┐
   │                                                        │
   │              REPOSITORY LAYER                         │
   │  (Database Access Only)                             │
   │                                                        │
   │  ┌────────────────────────────────────────────────┐  │
   │  │ MedicalRecordRepository                        │  │
   │  │ - find_all() → Query all records             │  │
   │  │ - find_by_id(id) → Get by ID                │  │
   │  │ - find_by_nik(nik) → Find unique record     │  │
   │  │ - insert(record) → Create in DB             │  │
   │  │ - update(id, record) → Update in DB         │  │
   │  │ - delete(id) → Remove from DB               │  │
   │  └────────────────────────────────────────────────┘  │
   │                                                        │
   │  ┌────────────────────────────────────────────────┐  │
   │  │ FileRepository                                 │  │
   │  │ - find_all() → List all files               │  │
   │  │ - find_by_id(id) → Get file by ID           │  │
   │  │ - insert(file) → Save file metadata         │  │
   │  │ - delete(id) → Remove file metadata         │  │
   │  └────────────────────────────────────────────────┘  │
   │                                                        │
   │  ┌────────────────────────────────────────────────┐  │
   │  │ DoctorRepository, AppointmentRepository, etc. │  │
   │  │ - Pure database operations per entity        │  │
   │  └────────────────────────────────────────────────┘  │
   │                                                        │
   └────────┬──────────────────────────────────────────────┘
            │
            │ Queries
            │
   ┌────────▼──────────────────────────────────────────────┐
   │                  MongoDB Database                      │
   │                   (Atlas Cloud)                        │
   │                                                        │
   │  Collections:                                         │
   │  ├─ medical_records                                   │
   │  ├─ doctors                                           │
   │  ├─ files (metadata only)                           │
   │  ├─ nurses                                           │
   │  ├─ medicines                                        │
   │  ├─ appointments                                     │
   │  ├─ services                                         │
   │  └─ insurances                                       │
   │                                                        │
   └────────────────────────────────────────────────────────┘

                    ┌──────────────────┐
                    │  S3 Integration  │
                    │  (NEO Storage)   │
                    │                  │
                    │ FileService calls│
                    │ S3 upload/delete │
                    └──────────────────┘
```

## Data Flow: Create Medical Record with Validation

```
┌─ REQUEST ────────────────────────────────────────────────────────┐
│                                                                    │
│  POST /medical-records                                            │
│  {                                                                │
│    "nik": "1234567890123456",                                    │
│    "name": "John Doe",                                           │
│    "age": 30,                                                    │
│    ...                                                            │
│  }                                                                │
└────────────────────┬─────────────────────────────────────────────┘
                     │
        ┌────────────▼──────────────┐
        │  HANDLER LAYER            │
        │                           │
        │ 1. Parse JSON             │
        │ 2. Create Repository      │
        │ 3. Create Service         │
        │ 4. Call service.create()  │
        │                           │
        └────────────┬──────────────┘
                     │
        ┌────────────▼──────────────────────────────────┐
        │  SERVICE LAYER                                │
        │                                               │
        │  1. Validate NIK format                       │
        │     └─ ✓ "1234567890123456" (16 digits)     │
        │                                               │
        │  2. Call repo.find_by_nik("1234567890123456")│
        │     └─ Check for duplicates                  │
        │                                               │
        │     ┌─────────────┬───────────────────────┐ │
        │     │ IF FOUND    │ RETURN ERROR          │ │
        │     │             │ (StatusCode::CONFLICT)│ │
        │     └─────────────┴───────────────────────┘ │
        │                                               │
        │     ┌─────────────┬───────────────────────┐ │
        │     │ NOT FOUND   │ Continue...           │ │
        │     │             │ Call repo.insert()    │ │
        │     └─────────────┴───────────────────────┘ │
        │                                               │
        │  3. Return (StatusCode::CREATED, record)    │
        │                                               │
        └────────────┬──────────────────────────────────┘
                     │
        ┌────────────▼──────────────┐
        │  REPOSITORY LAYER         │
        │                           │
        │  find_by_nik()            │
        │  ├─ Query: {nik: "..."}   │
        │  └─ Result: None/Some     │
        │                           │
        │  insert()                 │
        │  ├─ Ensure ID exists      │
        │  ├─ Execute insertOne()   │
        │  └─ Return record         │
        │                           │
        └────────────┬──────────────┘
                     │
        ┌────────────▼──────────────┐
        │  MONGODB                  │
        │                           │
        │  db.medical_records       │
        │    .findOne({nik: "..."})│
        │    .insertOne({...})      │
        │                           │
        └─────────────┬──────────────┘
                      │
        ┌─────────────▼──────────────┐
        │  RETURN PROPAGATION        │
        │                            │
        │  Repo → Service → Handler  │
        │   ↓      ↓        ↓        │
        │  Ok()   Ok()    Response   │
        │                            │
        └─────────────┬──────────────┘
                      │
┌─────────────────────▼───────────────────────────────────┐
│  RESPONSE                                                │
│                                                          │
│  HTTP/1.1 201 Created                                  │
│  Content-Type: application/json                        │
│                                                          │
│  {                                                       │
│    "_id": "507f1f77bcf86cd799439011",                  │
│    "nik": "1234567890123456",                          │
│    "name": "John Doe",                                 │
│    "age": 30,                                          │
│    ...                                                  │
│  }                                                       │
└─────────────────────────────────────────────────────────┘
```

## Error Flow: Duplicate NIK

```
POST /medical-records (NIK already exists)
        │
        ▼
   HANDLER
        │
        ├─ Parse JSON
        └─ Call Service.create()
                │
                ▼
           SERVICE
                │
                ├─ Validate format ✓
                │
                ├─ Call Repository.find_by_nik()
                │        │
                │        ▼
                │   REPOSITORY
                │        │
                │        ├─ Query MongoDB
                │        │
                │        └─ Return: Some(existing_record)
                │
                ├─ Check result
                │
                └─ ❌ Return Error:
                   (StatusCode::CONFLICT, "NIK already exists")
                        │
                        ▼
                   HANDLER
                        │
                        ├─ Match on Error
                        │
                        └─ Build Response:
                           (StatusCode::CONFLICT, error JSON)
                                │
                                ▼
           ┌─────────────────────────────────────┐
           │  HTTP/1.1 409 Conflict             │
           │                                     │
           │  {                                  │
           │    "error": "NIK already exists",   │
           │    "status": 409                    │
           │  }                                  │
           └─────────────────────────────────────┘

NOTE: Repository was NOT called for database operation!
      Error detected at Service level before DB query.
```

## File Upload Flow with S3

```
POST /files (multipart/form-data)
    │
    ├─ file: document.pdf (102KB)
    └─ uploader: "john_doe"
         │
         ▼
    HANDLER: create_file()
         │
         ├─ Parse multipart
         ├─ Extract file_bytes
         ├─ Extract file_name
         ├─ Call Service.create(filename, bytes, uploader)
         │
         ▼
    SERVICE: FileService.create()
         │
         ├─ Validate file size & type
         │  └─ Check: ✓ .pdf in allowed list
         │  └─ Check: ✓ 102KB < 256KB max
         │
         ├─ Upload to S3 (NEO)
         │  │
         │  ├─ Generate S3 key:
         │  │  "files/20260131_102030_document.pdf"
         │  │
         │  └─ S3 PUT request:
         │     https://nos.wjv-1.neo.id/atm-sehat/files/...
         │        │
         │        └─ Response: 200 OK
         │           File uploaded to NEO
         │
         ├─ Create File record object
         │  ├─ id: ObjectId
         │  ├─ name: "document.pdf"
         │  ├─ file_type: "application/pdf"
         │  ├─ extension: "pdf"
         │  ├─ size: 102400
         │  ├─ path: "files/20260131_102030_document.pdf"
         │  ├─ url: "https://nos.wjv-1.neo.id/atm-sehat/files/..."
         │  ├─ uploader: "john_doe"
         │  └─ created_at: "2026-01-31 10:20:30"
         │
         ├─ Call Repository.insert(file_record)
         │  │
         │  ▼
         │  REPOSITORY
         │  │
         │  ├─ Ensure ID ✓
         │  ├─ MongoDB insert
         │  └─ Return: Ok(file_record)
         │
         └─ Return: Ok((StatusCode::CREATED, file_record))
              │
              ▼
         HANDLER
              │
              └─ Return: (StatusCode::CREATED, Json(file_record))
                    │
                    ▼
         ┌─────────────────────────────────┐
         │  HTTP/1.1 201 Created          │
         │                                 │
         │  {                              │
         │    "_id": "507f...",           │
         │    "name": "document.pdf",     │
         │    "file_type": "application/pdf",
         │    "size": 102400,             │
         │    "url": "https://nos.wjv.../ │
         │    "uploader": "john_doe",     │
         │    "created_at": "2026-01-31..."
         │  }                              │
         └─────────────────────────────────┘
```

## Dependency Graph

```
                         ┌──────────────────┐
                         │  models.rs       │
                         │  (Data classes)  │
                         └────────┬─────────┘
                                  │
                    ┌─────────────┼─────────────┐
                    │             │             │
           ┌────────▼───────┐    │    ┌────────▼───────┐
           │ repository/*   │    │    │ services/*     │
           │ (Data access)  │    │    │ (Business log) │
           └────────┬───────┘    │    └────────┬───────┘
                    │            │             │
           ┌────────▼────────────▼─────────────▼────┐
           │           handlers.rs                   │
           │      (HTTP coordination)                │
           └────────────────┬─────────────────────────┘
                            │
                   ┌────────▼────────┐
                   │   routes.rs     │
                   │  (HTTP routes)  │
                   └─────────────────┘
                            │
                   ┌────────▼────────┐
                   │   main.rs       │
                   │  (Entry point)  │
                   └─────────────────┘

Key: No circular dependencies! Each layer only depends on lower layers.
```

## Validation Pyramid

```
                         HTTP Request
                            │
                    ┌───────▼────────┐
                    │   HANDLER      │
                    │   (HTTP parse) │
                    └───────┬────────┘
                            │
                    ┌───────▼────────────────┐
                    │   SERVICE              │
                    │   ✓ Format validation  │
                    │   ✓ Uniqueness check   │
                    │   ✓ Business rules     │
                    └───────┬────────────────┘
                            │
                    ┌───────▼────────────────┐
                    │   REPOSITORY           │
                    │   (Pure DB access)     │
                    └───────┬────────────────┘
                            │
                    ┌───────▼────────────────┐
                    │   MONGODB              │
                    │   (Indexes, constraints)
                    └────────────────────────┘

All validation happens at appropriate layers!
```

## Testing Strategy

```
┌────────────────────────────────────────────────┐
│         SERVICE UNIT TEST (Fast)               │
│                                                │
│  Mock Repository  →  Test Service Logic       │
│                                                │
│  ✓ Validation      ✓ Duplicate check          │
│  ✓ Error handling  ✓ Business rules           │
│  ✓ Status codes    ✓ Coordination             │
│                                                │
│  No database needed!                          │
└────────────────────────────────────────────────┘

┌────────────────────────────────────────────────┐
│     INTEGRATION TEST (Slow but complete)      │
│                                                │
│  Real Repository  →  Real Database            │
│                                                │
│  ✓ Full HTTP flow   ✓ Handler response        │
│  ✓ DB operations    ✓ Error handling          │
│                                                │
│  Requires MongoDB setup                       │
└────────────────────────────────────────────────┘
```

---

These diagrams show the complete architecture and data flows in the Repository Service Pattern implementation.
