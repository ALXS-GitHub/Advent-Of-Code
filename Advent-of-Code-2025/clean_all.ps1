Get-ChildItem -Directory -Filter "day*" | ForEach-Object {
    Write-Host "Cleaning $_"
    Set-Location $_.FullName
    cargo clean
    Set-Location ..
}