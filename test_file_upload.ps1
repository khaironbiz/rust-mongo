$filePath = "C:\xampp\htdocs\tsi\rust-mongo\README.md"
$fileContent = [System.IO.File]::ReadAllBytes($filePath)
$fileName = [System.IO.Path]::GetFileName($filePath)

$boundary = [System.Guid]::NewGuid().ToString()
$body = @"
--$boundary
Content-Disposition: form-data; name="file"; filename="$fileName"
Content-Type: application/octet-stream

"@

$bodyBytes = [System.Text.Encoding]::UTF8.GetBytes($body)
$allBytes = New-Object System.Collections.ArrayList
$allBytes.AddRange($bodyBytes)
$allBytes.AddRange($fileContent)

$footer = @"

--$boundary
Content-Disposition: form-data; name="uploader"

testuser
--$boundary--
"@

$footerBytes = [System.Text.Encoding]::UTF8.GetBytes($footer)
$allBytes.AddRange($footerBytes)

try {
    $response = Invoke-WebRequest -Uri "http://127.0.0.1:8000/files" -Method POST -ContentType "multipart/form-data; boundary=$boundary" -Body $allBytes.ToArray()
    Write-Host "Status: $($response.StatusCode)"
    Write-Host "Response:"
    $response.Content | ConvertFrom-Json | ConvertTo-Json
} catch {
    Write-Host "Error: $($_.Exception.Message)"
    Write-Host "Response:"
    $_.Exception.Response
}
