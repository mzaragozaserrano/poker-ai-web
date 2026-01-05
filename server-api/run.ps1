# Development server startup script

param(
    [int]$Port = 8000,
    [string]$Host = "127.0.0.1"
)

$ErrorActionPreference = "Stop"

# SECURITY: Validate that host is localhost only
$allowedHosts = @("127.0.0.1", "localhost", "::1")
if ($Host -notin $allowedHosts) {
    Write-Host "SECURITY ERROR: Invalid host '$Host'" -ForegroundColor Red
    Write-Host "This API must only bind to localhost (127.0.0.1)" -ForegroundColor Red
    Write-Host "Binding to 0.0.0.0 or other IPs would expose data to the network." -ForegroundColor Red
    exit 1
}

Write-Host "Starting Poker AI API server..." -ForegroundColor Cyan
Write-Host "Listening on http://$Host`:$Port" -ForegroundColor Green
Write-Host "API documentation: http://$Host`:$Port/docs" -ForegroundColor Green
Write-Host "SECURITY: Localhost-only mode enabled" -ForegroundColor Yellow

# Set environment to development
$env:DEBUG = "true"

# Run uvicorn with hot reload
poetry run uvicorn app.main:app --host $Host --port $Port --reload

Write-Host "Server stopped." -ForegroundColor Yellow


