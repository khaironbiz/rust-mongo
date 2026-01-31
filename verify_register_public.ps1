$baseUrl = "http://localhost:8000"

Write-Host "--- Verifying /auth/register Accessibility ---"

# Try to hit /auth/register with invalid data (but no token)
# If it returns 400 (Validation Error) or 409 (Conflict), it means it's PUBLIC.
# If it returns 401 (Unauthorized), it means it's PROTECTED.

$payload = @{
    email = "test_" + (Get-Date -Format "yyyyMMddHHmmss") + "@example.com"
    password = "short" # Should trigger validation error if public
    name = "Test User"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/auth/register" -Method Post -Body $payload -ContentType "application/json"
    Write-Host "Success! Response: $($response | ConvertTo-Json)"
} catch {
    $statusCode = $_.Exception.Response.StatusCode.value__
    Write-Host "Caught Exception with Status Code: $statusCode"
    
    if ($statusCode -eq 401) {
        Write-Host "FAILED: Route is still protected (401 Unauthorized)." -ForegroundColor Red
    } elseif ($statusCode -eq 400 -or $statusCode -eq 409 -or $statusCode -eq 201) {
        Write-Host "PASSED: Route is public (Received $statusCode)." -ForegroundColor Green
    } else {
        Write-Host "UNEXPECTED: Received status code $statusCode." -ForegroundColor Yellow
    }
}
