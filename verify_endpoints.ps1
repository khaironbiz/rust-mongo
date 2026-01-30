$baseUrl = "http://localhost:8000"

Write-Host "--- Testing Jaga Sehat Indonesia API ---"

# 1. Create a Doctor
$doctorPayload = @{
    name = "Dr. Budi"
    nip = "123456"
    sip = "SIP-001"
    specialization = "Umum"
    status = "Aktif"
} | ConvertTo-Json

Write-Host "`n1. Creating Doctor..."
$doctor = Invoke-RestMethod -Uri "$baseUrl/doctors" -Method Post -Body $doctorPayload -ContentType "application/json"
Write-Host "Created Doctor ID: $($doctor.id)"

# 2. Get Doctors
Write-Host "`n2. Getting Doctors..."
$doctors = Invoke-RestMethod -Uri "$baseUrl/doctors" -Method Get
Write-Host "Found $($doctors.Count) doctors"

# 3. Create Medical Record
$mrPayload = @{
    nrme = "RM-001"
    nik = "3201234567890"
    name = "Pasien A"
    dob = "1990-01-01"
    gender = "Laki-laki"
    hp = "08123456789"
    email = "pasien@example.com"
    lastVisitDate = "2023-01-01"
} | ConvertTo-Json

Write-Host "`n3. Creating Medical Record..."
$mr = Invoke-RestMethod -Uri "$baseUrl/medical-records" -Method Post -Body $mrPayload -ContentType "application/json"
Write-Host "Created Medical Record ID: $($mr.id)"

# 4. Get Medical Records
Write-Host "`n4. Getting Medical Records..."
$mrs = Invoke-RestMethod -Uri "$baseUrl/medical-records" -Method Get
Write-Host "Found $($mrs.Count) medical records"

# 5. Create Appointment
if ($mr.id -and $doctor.id) {
    $apptPayload = @{
        patientId = $mr.id
        doctorId = $doctor.id
        date = "2023-10-27"
        time = "10:00"
        status = "Menunggu"
    } | ConvertTo-Json

    Write-Host "`n5. Creating Appointment..."
    $appt = Invoke-RestMethod -Uri "$baseUrl/appointments" -Method Post -Body $apptPayload -ContentType "application/json"
    Write-Host "Created Appointment ID: $($appt.id)"
} else {
    Write-Host "`nSkipping Appointment creation because dependencies failed."
}

Write-Host "`n--- Test Finished ---"
