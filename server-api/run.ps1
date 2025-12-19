# Development server startup script

param(
    [int]$Port = 8000,
    [string]$Host = "127.0.0.1"
)

$ErrorActionPreference = "Stop"

Write-Host "Starting Poker AI API server..." -ForegroundColor Cyan
Write-Host "Listening on http://$Host`:$Port" -ForegroundColor Green
Write-Host "API documentation: http://$Host`:$Port/docs" -ForegroundColor Green

# Set environment to development
$env:DEBUG = "true"

# Run uvicorn with hot reload
poetry run uvicorn app.main:app --host $Host --port $Port --reload

Write-Host "Server stopped." -ForegroundColor Yellow
