# Repository Service Pattern - Complete Flow Examples

## Example 1: Create Medical Record with Validation

### Request
```bash
curl -X POST http://localhost:8000/medical-records \
  -H "Content-Type: application/json" \
  -d '{
    "nik": "1234567890123456",
    "name": "John Doe",
    "age": 30,
    "gender": "Male",
    "address": "Jl. Test",
    "phone": "08123456789",
    "diagnosis": "Healthy"
  }'
```

### Data Flow

```
1. HANDLER (create_medical_record)
   ├─ Parse JSON → MedicalRecord {
   │    id: None,
   │    nik: "1234567890123456",
   │    ...
   │  }
   ├─ Generate ID if None
   │    id: Some(ObjectId::new())
   └─ Call Service.create(record)

2. SERVICE (MedicalRecordService.create)
   ├─ Validate NIK format
   │  └─ validation::validate_nik("1234567890123456")
   │     └─ Check if 16 digits
   │        └─ ✓ Valid
   │
   ├─ Check NIK uniqueness
   │  └─ Repository.find_by_nik("1234567890123456")
   │     └─ MongoDB: db.medical_records.findOne({nik: "..."})
   │        └─ Result: None (not found, OK)
   │
   └─ Insert record
      └─ Repository.insert(record)

3. REPOSITORY (MedicalRecordRepository.insert)
   ├─ Ensure ID exists
   │  └─ id = Some(ObjectId::new()) ✓
   │
   └─ Execute MongoDB insert_one
      └─ db.medical_records.insertOne({
           _id: ObjectId(...),
           nik: "1234567890123456",
           name: "John Doe",
           ...
         })
         └─ Result: Inserted successfully

4. RETURN VALUE PROPAGATION (upward)
   ├─ Repository returns: Ok(MedicalRecord { ... })
   │
   ├─ Service receives: Ok(created_record)
   │  └─ Returns: Ok((StatusCode::CREATED, created_record))
   │
   └─ Handler receives: Ok((StatusCode::CREATED, record))
      └─ Convert to HTTP Response:
         {
           "status": 201,
           "body": {
             "_id": "507f1f77bcf86cd799439011",
             "nik": "1234567890123456",
             "name": "John Doe",
             ...
           }
         }
```

### Response
```json
HTTP/1.1 201 Created
Content-Type: application/json

{
  "_id": "507f1f77bcf86cd799439011",
  "nik": "1234567890123456",
  "name": "John Doe",
  "age": 30,
  "gender": "Male",
  "address": "Jl. Test",
  "phone": "08123456789",
  "diagnosis": "Healthy"
}
```

## Example 2: Create Medical Record with Duplicate NIK (Error Case)

### Request
```bash
curl -X POST http://localhost:8000/medical-records \
  -H "Content-Type: application/json" \
  -d '{
    "nik": "1234567890123456",
    "name": "Jane Doe",
    ...
  }'
```

(Assuming medical record with NIK "1234567890123456" already exists)

### Data Flow

```
1. HANDLER (create_medical_record)
   ├─ Parse JSON → MedicalRecord
   ├─ Generate ID
   └─ Call Service.create(record)

2. SERVICE (MedicalRecordService.create)
   ├─ Validate NIK format
   │  └─ validation::validate_nik("1234567890123456")
   │     └─ ✓ Valid format
   │
   ├─ Check NIK uniqueness
   │  └─ Repository.find_by_nik("1234567890123456")
   │     └─ MongoDB: db.medical_records.findOne({nik: "..."})
   │        └─ Result: Some(MedicalRecord { ... }) ✗ FOUND!
   │
   └─ Return Error
      └─ Err((StatusCode::CONFLICT, "NIK already exists"))

3. REPOSITORY (NOT CALLED for database)
   (Skipped because error in service layer)

4. RETURN VALUE PROPAGATION (upward with error)
   ├─ Service returns: Err((StatusCode::CONFLICT, "NIK already exists"))
   │
   └─ Handler receives: Err((StatusCode::CONFLICT, msg))
      └─ Convert to HTTP Response:
         {
           "status": 409,
           "error": "NIK already exists"
         }
```

### Response
```json
HTTP/1.1 409 Conflict
Content-Type: application/json

{
  "error": "NIK already exists",
  "status": 409
}
```

## Example 3: Upload File

### Request
```bash
curl -X POST http://localhost:8000/files \
  -F "file=@/path/to/document.pdf" \
  -F "uploader=john_doe"
```

### Data Flow

```
1. HANDLER (create_file)
   ├─ Parse Multipart Form Data
   │  ├─ Extract field "file"
   │  │  ├─ Read file bytes
   │  │  └─ file_bytes: Vec<u8>
   │  │
   │  └─ Extract field "uploader"
   │     └─ uploader: "john_doe"
   │
   ├─ Validate file exists
   │  └─ Check file_name and file_bytes are not empty
   │     └─ ✓ Valid
   │
   └─ Call Service.create(file_name, file_bytes, uploader)

2. SERVICE (FileService.create)
   ├─ Validate file
   │  └─ validation::validate_file_upload(file_name, size)
   │     ├─ Check extension: .pdf ✓
   │     ├─ Check size: < 256KB ✓
   │     └─ Valid
   │
   ├─ Upload to S3 (NEO Object Storage)
   │  └─ S3::upload_file_to_s3(
   │       client, bucket, s3_key, file_bytes
   │     )
   │     ├─ Generate s3_key: "files/20260131_102030_document.pdf"
   │     └─ S3 PUT request to https://nos.wjv-1.neo.id/atm-sehat/files/20260131_102030_document.pdf
   │        └─ Response: 200 OK, file uploaded
   │
   ├─ Create File record
   │  └─ File {
   │       id: Some(ObjectId),
   │       name: "document.pdf",
   │       file_type: "application/pdf",
   │       extension: "pdf",
   │       size: 102400,
   │       path: "files/20260131_102030_document.pdf",
   │       url: "https://nos.wjv-1.neo.id/atm-sehat/files/20260131_102030_document.pdf",
   │       uploader: "john_doe",
   │       created_at: "2026-01-31 10:20:30"
   │     }
   │
   └─ Insert record into database
      └─ Repository.insert(file_record)

3. REPOSITORY (FileRepository.insert)
   ├─ Ensure ID exists ✓
   └─ MongoDB insert
      └─ db.files.insertOne({
           _id: ObjectId(...),
           name: "document.pdf",
           file_type: "application/pdf",
           extension: "pdf",
           size: 102400,
           path: "files/20260131_102030_document.pdf",
           url: "https://nos.wjv-1.neo.id/...",
           uploader: "john_doe",
           created_at: "2026-01-31 10:20:30"
         })
         └─ Success

4. RETURN VALUE PROPAGATION (upward)
   ├─ Repository: Ok(File { ... })
   ├─ Service: Ok((StatusCode::CREATED, File { ... }))
   └─ Handler: Transform to HTTP response (201 Created)
```

### Response
```json
HTTP/1.1 201 Created
Content-Type: application/json

{
  "_id": "507f1f77bcf86cd799439012",
  "name": "document.pdf",
  "file_type": "application/pdf",
  "extension": "pdf",
  "size": 102400,
  "path": "files/20260131_102030_document.pdf",
  "url": "https://nos.wjv-1.neo.id/atm-sehat/files/20260131_102030_document.pdf",
  "uploader": "john_doe",
  "created_at": "2026-01-31 10:20:30"
}
```

## Example 4: Get Medical Record by ID

### Request
```bash
curl -X GET http://localhost:8000/medical-records/507f1f77bcf86cd799439011
```

### Data Flow (Success Case)

```
1. HANDLER (get_medical_record)
   ├─ Extract path parameter: "507f1f77bcf86cd799439011"
   ├─ Parse to ObjectId
   │  └─ ObjectId::parse_str("507f1f77bcf86cd799439011")
   │     └─ ✓ Valid ObjectId
   │
   └─ Call Service.get_by_id(oid)

2. SERVICE (MedicalRecordService.get_by_id)
   └─ Call Repository.find_by_id(oid)

3. REPOSITORY (MedicalRecordRepository.find_by_id)
   └─ MongoDB findOne
      └─ db.medical_records.findOne({_id: ObjectId(...)})
         └─ Result: Some(MedicalRecord { ... })

4. RETURN VALUE PROPAGATION
   ├─ Repository: Ok(Some(MedicalRecord { ... }))
   ├─ Service: Ok(Some(MedicalRecord { ... }))
   └─ Handler: 
      ├─ Ok(Some(record)) match arm
      └─ Return (StatusCode::OK, Json(record))
```

### Response (Success)
```json
HTTP/1.1 200 OK
Content-Type: application/json

{
  "_id": "507f1f77bcf86cd799439011",
  "nik": "1234567890123456",
  "name": "John Doe",
  ...
}
```

### Response (Not Found)
```json
HTTP/1.1 404 Not Found
Content-Type: text/plain

Record not found
```

## Example 5: Delete File

### Request
```bash
curl -X DELETE http://localhost:8000/files/507f1f77bcf86cd799439012
```

### Data Flow

```
1. HANDLER (delete_file)
   ├─ Extract path parameter: "507f1f77bcf86cd799439012"
   ├─ Parse to ObjectId ✓
   └─ Call Service.delete(oid)

2. SERVICE (FileService.delete)
   ├─ Get file record
   │  └─ Repository.find_by_id(oid)
   │     └─ MongoDB: Find file metadata
   │        └─ Result: Some(File { 
   │             path: "files/20260131_102030_document.pdf",
   │             ...
   │           })
   │
   ├─ Delete from S3
   │  └─ S3::delete_file_from_s3(client, bucket, path)
   │     ├─ Generate S3 URL from bucket and path
   │     └─ S3 DELETE request to NEO Object Storage
   │        └─ Response: 200 OK, file deleted
   │
   └─ Delete from MongoDB
      └─ Repository.delete(oid)
         └─ db.files.deleteOne({_id: ObjectId(...)})
            └─ Result: deleted_count: 1

3. RETURN VALUE PROPAGATION
   ├─ Repository: Ok(true) [because deleted_count > 0]
   ├─ Service: Ok(true)
   └─ Handler:
      ├─ Ok(true) match arm
      └─ Return (StatusCode::NO_CONTENT, "")
```

### Response
```json
HTTP/1.1 204 No Content
```

## Error Handling Scenarios

### Scenario 1: Database Connection Error
```
Handler
  └─ Service.get_all()
     └─ Repository.find_all()
        └─ MongoDB connection fails
           └─ Err("Database error: connection refused")
              └─ Service receives error string
                 └─ Returns: Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error: ..."))
                    └─ Handler converts to 500 response
```

### Scenario 2: Validation Error
```
Handler
  └─ Service.create(record)
     ├─ Validate format
     │  └─ Returns Err
     │     └─ Service immediately returns:
     │        Err((StatusCode::BAD_REQUEST, "Invalid format"))
     └─ Repository never called
        └─ Handler converts to 400 response
```

### Scenario 3: Business Logic Error (Duplicate)
```
Handler
  └─ Service.create(record)
     ├─ Validate format ✓
     ├─ Check uniqueness
     │  └─ Repository.find_by_nik(nik)
     │     └─ Result: Some(...) // Found!
     │        └─ Service returns:
     │           Err((StatusCode::CONFLICT, "NIK already exists"))
     └─ Handler converts to 409 response
```

## Status Codes Used

| Code | Scenario |
|------|----------|
| 200 | GET successful |
| 201 | POST successful (created) |
| 204 | DELETE successful (no content) |
| 400 | Validation error |
| 404 | Resource not found |
| 409 | Conflict (duplicate, business logic) |
| 500 | Database/internal error |

## Performance Considerations

### Layer 1 (Handler)
- Minimal processing
- Quick object parsing
- ~1ms per request

### Layer 2 (Service)  
- Validation checks (fast, in-memory)
- Business logic (usually fast)
- May call Repository 1-2 times
- ~1-5ms per request

### Layer 3 (Repository)
- Database query execution
- Network to MongoDB
- Slowest part of pipeline
- ~10-100ms per query

### Layer 4 (External)
- S3 upload (for FileService)
- Network to NEO Object Storage
- Can be very slow for large files
- May need async processing

## Example Request Timeline

```
Request arrives: T=0ms
  ↓
Handler parsing: T=1ms
  ├─ Invoke Service
  ↓
Service validation: T=2ms
  ├─ Create Repository
  ├─ Invoke Repository
  ↓
Repository DB query: T=50ms
  ├─ MongoDB query
  ├─ Network roundtrip
  ↓
Return to Service: T=51ms
  ├─ Process result
  ├─ Return to Handler
  ↓
Handler builds response: T=52ms
  ├─ Convert to JSON
  ├─ Send HTTP response
  ↓
Request complete: T=53ms
```

This three-layer pattern is optimized for clarity and maintainability!
