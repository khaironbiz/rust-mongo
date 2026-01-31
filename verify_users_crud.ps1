$baseUrl = "http://localhost:8000"

Write-Host "--- Verifying Users CRUD Feature ---"

# 1. Register a temporary user for verification
$testEmail = "admin_test_" + (Get-Date -Format "yyyyMMddHHmmss") + "@example.com"
$registerPayload = @{
    email    = $testEmail
    password = "password123"
    name     = "Admin Test User"
} | ConvertTo-Json

try {
    Write-Host "`n1. Registering test admin user ($testEmail)..."
    $registerResponse = Invoke-RestMethod -Uri "$baseUrl/auth/register" -Method Post -Body $registerPayload -ContentType "application/json"
    Write-Host "Registered successfully."

    # 2. Login to get token
    $loginPayload = @{
        email    = $testEmail
        password = "password123"
    } | ConvertTo-Json

    Write-Host "`n2. Logging in..."
    $loginResponse = Invoke-RestMethod -Uri "$baseUrl/auth/login" -Method Post -Body $loginPayload -ContentType "application/json"
    $token = $loginResponse.data.access_token
    Write-Host "Logged in successfully."

    $headers = @{
        "Authorization" = "Bearer $token"
        "Content-Type"  = "application/json"
    }

    # 3. List Users
    Write-Host "`n3. Listing Users..."
    $usersResponse = Invoke-RestMethod -Uri "$baseUrl/users" -Method Get -Headers $headers
    Write-Host "Found $($usersResponse.data.items.Count) users."

    # 4. Create a New User (via CRUD endpoint)
    $newEmail = "new_user_" + (Get-Date -Format "yyyyMMddHHmmss") + "@example.com"
    $createPayload = @{
        email    = $newEmail
        password = "newpassword123"
        name     = "New User Test"
    } | ConvertTo-Json

    Write-Host "`n4. Creating New User ($newEmail)..."
    $createResponse = Invoke-RestMethod -Uri "$baseUrl/users" -Method Post -Body $createPayload -Headers $headers
    $newUserId = $createResponse.data.id
    Write-Host "Created User ID: $newUserId"

    # 5. Get User by ID
    Write-Host "`n5. Getting User by ID..."
    $getResponse = Invoke-RestMethod -Uri "$baseUrl/users/$newUserId" -Method Get -Headers $headers
    Write-Host "Retrieved User: $($getResponse.data.name) ($($getResponse.data.email))"

    # 6. Update User
    $updatePayload = @{
        name = "Updated User Name"
    } | ConvertTo-Json

    Write-Host "`n6. Updating User..."
    $updateResponse = Invoke-RestMethod -Uri "$baseUrl/users/$newUserId" -Method Put -Body $updatePayload -Headers $headers
    Write-Host "Updated Name: $($updateResponse.data.name)"

    # 7. Delete User
    Write-Host "`n7. Deleting User..."
    $deleteResponse = Invoke-RestMethod -Uri "$baseUrl/users/$newUserId" -Method Delete -Headers $headers
    Write-Host "User deleted successfully."

    Write-Host "`n--- PASSED: Users CRUD Verification Finished ---" -ForegroundColor Green

}
catch {
    Write-Host "`nFAILED: An error occurred during verification." -ForegroundColor Red
    Write-Host "Exception: $($_.Exception.Message)"
    if ($_.Exception.Response) {
        $errorBody = $_.Exception.Response.GetResponseStream()
        $reader = New-Object System.IO.StreamReader($errorBody)
        Write-Host "Error Body: $($reader.ReadToEnd())"
    }
}
