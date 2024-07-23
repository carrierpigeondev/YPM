param(
    [string]$targetName = "test",
    [string]$target = "Windows",
    [string]$serverIP = "localhost",
    [string]$serverPort = "41824"
)

# specify the URI\L to the server
$uri = "http://${serverIP}:$serverPort/package?name=$targetName&target=$target"

# get the response from the server
$response = Invoke-RestMethod -Uri $uri -Method Post

# use the current working directory to form the full path
# this will also be the initial location of failure if the response fails
# as file_name will not be present if the response fails
$path = Join-Path -Path (Get-Location) -ChildPath $response.file_name

# decode the Base64 content to bytes
$bytes = [System.Convert]::FromBase64String($response.binary_content)

# write the decoded data to the file
[System.IO.File]::WriteAllBytes($path, $bytes)

# checking is essential as PowerShell will just continue
if (Test-Path $path) {
    Write-Output "File saved to $path"

    # check the sha256 of the contents of the file
    # see if it matches with the one received in the request
    $hasher = [System.Security.Cryptography.SHA256]::Create()
    $fileContent = [System.IO.File]::ReadAllBytes($path)
    $computedHash = [BitConverter]::ToString($hasher.ComputeHash($fileContent)).Replace("-", "").ToLower()

    # notify the user if it is valid or not
    if ($computedHash -eq $response.sha256) {
        Write-Output "Checksum valid. It is safe to run $path"
    } else {
        Write-Output "Checksum invalid. Be cautious when opening $path"
    }

} else {
    Write-Output "Failed to save the file at $path"
}
