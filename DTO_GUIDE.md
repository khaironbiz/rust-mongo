# DTO (Data Transfer Object) Standardization Guide

## Overview

DTOs (Data Transfer Objects) have been implemented to standardize data transfer between API clients and the server. DTOs provide:

✅ **Clear separation** between database models and API contracts  
✅ **Input validation** at the DTO level  
✅ **Consistent response format** across all endpoints  
✅ **Optional fields** for partial updates  
✅ **Type safety** and automatic serialization/deserialization  

## Architecture

### DTO Layers

```
Client HTTP Request
        ↓
    [Request DTO] ← Deserialization & Validation
        ↓
   Handler Layer
        ↓
   Service Layer
        ↓
 Database Model
        ↓
   Service Layer
        ↓
    [Response DTO] ← Serialization
        ↓
Client HTTP Response
```

## DTO Structure

### Request DTOs (Input)

**Pattern**: `Create{Entity}Request`, `Update{Entity}Request`

**Characteristics**:
- Validates input data format
- Contains only required fields for creation
- Optional fields for partial updates
- Automatically deserializes from JSON

**Example**:
```rust
pub struct CreateMedicalRecordRequest {
    pub nik: String,              // Required
    pub name: String,             // Required
    pub age: u32,                 // Required
    pub gender: String,           // Required
    pub phone: String,            // Required
    pub address: String,          // Required
    #[serde(default)]
    pub medical_history: Option<String>, // Optional
}
```

### Response DTOs (Output)

**Pattern**: `{Entity}Response`

**Characteristics**:
- Clean, simplified data structure
- Removes internal fields (IDs, timestamps for simplicity)
- Hides sensitive information
- Properly formatted timestamps
- Optional fields marked with `#[serde(skip_serializing_if = "Option::is_none")]`

**Example**:
```rust
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
```

## Available DTOs

### Medical Records

**Create**:
```json
{
  "nik": "1234567890123456",
  "name": "John Doe",
  "age": 30,
  "gender": "M",
  "phone": "08123456789",
  "address": "123 Main Street",
  "medical_history": "No allergies"
}
```

**Update** (all fields optional):
```json
{
  "name": "Jane Doe",
  "age": 31
}
```

**Response**:
```json
{
  "id": "507f1f77bcf86cd799439011",
  "nik": "1234567890123456",
  "name": "John Doe",
  "age": 30,
  "gender": "M",
  "phone": "08123456789",
  "address": "123 Main Street",
  "medical_history": "No allergies",
  "created_at": "2026-01-31 12:00:00",
  "updated_at": "2026-01-31 12:30:00"
}
```

### Doctors

**Create**:
```json
{
  "name": "Dr. Smith",
  "nip": "1234567890123456",
  "specialty": "Cardiology",
  "phone": "08123456789",
  "email": "smith@hospital.com",
  "bio": "Expert in cardiovascular diseases"
}
```

**Response**:
```json
{
  "id": "507f1f77bcf86cd799439012",
  "name": "Dr. Smith",
  "nip": "1234567890123456",
  "specialty": "Cardiology",
  "phone": "08123456789",
  "email": "smith@hospital.com",
  "bio": "Expert in cardiovascular diseases",
  "created_at": "2026-01-31 12:00:00"
}
```

### Nurses

**Create**:
```json
{
  "name": "Jane Nurse",
  "license_number": "NI1234567890",
  "phone": "08123456789",
  "email": "jane@hospital.com",
  "shift": "Morning",
  "specialization": "ICU"
}
```

### Medicines

**Create**:
```json
{
  "name": "Aspirin",
  "generic_name": "Acetylsalicylic acid",
  "dosage": "500mg",
  "unit": "tablet",
  "price": 5000,
  "manufacturer": "Bayer",
  "description": "Pain reliever"
}
```

### Appointments

**Create**:
```json
{
  "patient_id": "507f1f77bcf86cd799439011",
  "doctor_id": "507f1f77bcf86cd799439012",
  "appointment_date": "2026-02-15",
  "appointment_time": "10:30",
  "reason": "Regular checkup",
  "notes": "Please bring previous medical records"
}
```

### Services (Healthcare)

**Create**:
```json
{
  "name": "General Consultation",
  "code": "GC001",
  "category": "Consultation",
  "price": 150000,
  "description": "30-minute general consultation with doctor"
}
```

### Insurances

**Create**:
```json
{
  "name": "Health Plus",
  "code": "HP001",
  "coverage_percentage": 80.5,
  "max_claim_amount": 100000000,
  "phone": "1234567890",
  "email": "contact@healthplus.com"
}
```

## HTTP Endpoints & DTOs

### Medical Records

| Method | Endpoint | Request DTO | Response DTO |
|--------|----------|-------------|--------------|
| POST | `/medical-records` | `CreateMedicalRecordRequest` | `MedicalRecordResponse` |
| PUT | `/medical-records/{id}` | `UpdateMedicalRecordRequest` | `MedicalRecordResponse` |
| GET | `/medical-records/{id}` | - | `MedicalRecordResponse` |
| GET | `/medical-records` | - | `List<MedicalRecordResponse>` (paginated) |
| DELETE | `/medical-records/{id}` | - | `DeleteResponse` |

### Doctors

| Method | Endpoint | Request DTO | Response DTO |
|--------|----------|-------------|--------------|
| POST | `/doctors` | `CreateDoctorRequest` | `DoctorResponse` |
| PUT | `/doctors/{id}` | `UpdateDoctorRequest` | `DoctorResponse` |
| GET | `/doctors/{id}` | - | `DoctorResponse` |
| GET | `/doctors` | - | `List<DoctorResponse>` (paginated) |

### Files

| Method | Endpoint | Request DTO | Response DTO |
|--------|----------|-------------|--------------|
| POST | `/files` | Multipart Form Data | `FileResponse` |
| GET | `/files/{id}` | - | `FileResponse` |
| GET | `/files` | - | `List<FileResponse>` (paginated) |
| DELETE | `/files/{id}` | - | `DeleteResponse` |

## Validation Rules

### Medical Record Request

```
nik:             • 16 digits only
name:            • Required, max 100 chars
age:             • Required, 1-150 range
gender:          • Required, one of: M, F
phone:           • Required, Indonesian format
address:         • Required, max 255 chars
medical_history: • Optional, max 1000 chars
```

### Doctor Request

```
name:            • Required, max 100 chars
nip:             • Required, 16 digits
specialty:       • Required, max 50 chars
phone:           • Required, Indonesian format
email:           • Required, valid email
bio:             • Optional, max 500 chars
```

### Update Requests

All fields are optional - only provided fields are updated.

```json
{
  "name": "Updated Name"
  // Other fields remain unchanged
}
```

## Request/Response Examples

### Create Medical Record

**Request**:
```bash
curl -X POST http://localhost:3000/medical-records \
  -H "Content-Type: application/json" \
  -d '{
    "nik": "1234567890123456",
    "name": "John Doe",
    "age": 30,
    "gender": "M",
    "phone": "08123456789",
    "address": "123 Main Street"
  }'
```

**Response** (201 Created):
```json
{
  "success": true,
  "status": 201,
  "message": "Medical record created successfully",
  "data": {
    "id": "507f1f77bcf86cd799439011",
    "nik": "1234567890123456",
    "name": "John Doe",
    "age": 30,
    "gender": "M",
    "phone": "08123456789",
    "address": "123 Main Street",
    "created_at": "2026-01-31 12:00:00"
  },
  "timestamp": "2026-01-31 12:00:00"
}
```

### Update Medical Record

**Request**:
```bash
curl -X PUT http://localhost:3000/medical-records/507f1f77bcf86cd799439011 \
  -H "Content-Type: application/json" \
  -d '{
    "age": 31,
    "phone": "08987654321"
  }'
```

**Response** (200 OK):
```json
{
  "success": true,
  "status": 200,
  "message": "Medical record updated successfully",
  "data": {
    "id": "507f1f77bcf86cd799439011",
    "nik": "1234567890123456",
    "name": "John Doe",
    "age": 31,
    "gender": "M",
    "phone": "08987654321",
    "address": "123 Main Street",
    "created_at": "2026-01-31 12:00:00",
    "updated_at": "2026-01-31 12:15:00"
  },
  "timestamp": "2026-01-31 12:15:00"
}
```

## Type Safety

DTOs provide compile-time type safety:

```rust
// ✅ Correct - Type-safe
let req: CreateMedicalRecordRequest = serde_json::from_str(json)?;

// ✅ Correct - Automatic validation during deserialization
if req.age > 150 {
    return Err("Invalid age");
}

// ❌ Compile error - Type mismatch
let req: DoctorResponse = serde_json::from_str(medical_record_json)?;
```

## Serialization/Deserialization

### Automatic Conversion

DTOs automatically convert to/from JSON:

```rust
// Input
let json = r#"{"nik": "1234567890123456", "name": "John", ...}"#;
let req: CreateMedicalRecordRequest = serde_json::from_str(json)?;

// Output
let response = MedicalRecordResponse { ... };
let json = serde_json::to_string(&response)?;
```

### Optional Fields

Fields marked with `#[serde(default)]` are optional in input:

```json
{
  "name": "John Doe",
  "age": 30
  // "medical_history" is optional
}
```

Fields marked with `#[serde(skip_serializing_if = "Option::is_none")]` are omitted if None:

```json
{
  "id": "...",
  "name": "John Doe",
  // "medical_history": null is omitted
}
```

## Future Enhancements

1. **Custom Validators** - Add attribute-based validation
2. **Versioning** - Support multiple API versions
3. **Transformation** - Auto-convert between DTOs and models
4. **Serialization Rules** - Custom field naming, formatting
5. **Documentation** - Auto-generate OpenAPI from DTOs

## Best Practices

### For Clients

1. **Always validate DTOs**
   ```javascript
   if (!response.data.id) {
       console.error("Invalid response");
   }
   ```

2. **Handle optional fields**
   ```javascript
   const bio = response.data.bio || "No bio available";
   ```

3. **Use type definitions**
   ```typescript
   interface MedicalRecordResponse {
       id: string;
       nik: string;
       name: string;
       // ... other fields
   }
   ```

### For Servers

1. **Keep DTOs simple** - Mirror your API contract
2. **Separate concerns** - Keep DTOs separate from models
3. **Document fields** - Add comments for clarity
4. **Test serialization** - Unit test DTO conversions
5. **Version DTOs** - Track changes over time

## File Location

All DTOs are defined in: `src/dto.rs`

Structure:
- Medical Record DTOs (lines 1-50)
- Doctor DTOs (lines 51-100)
- Nurse DTOs (lines 101-150)
- Medicine DTOs (lines 151-200)
- Appointment DTOs (lines 201-250)
- Service DTOs (lines 251-300)
- Insurance DTOs (lines 301-350)
- File DTOs (lines 351-380)
- Common DTOs (lines 381-420)
- Tests (lines 421-450)

## Summary

DTOs provide:

✅ **Clear API contracts** - Explicit input/output structures  
✅ **Validation** - Type-safe data handling  
✅ **Separation** - DTOs separate from database models  
✅ **Consistency** - Standardized across all endpoints  
✅ **Documentation** - Self-documenting API structure  
✅ **Type Safety** - Compile-time error detection  

All endpoints now use standardized DTOs for maximum clarity and safety!
