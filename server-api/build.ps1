# Build script for Poker AI API
# Compiles Rust extensions using maturin

param(
    [switch]$Release = $false,
    [switch]$Dev = $false
)

$ErrorActionPreference = "Stop"

Write-Host "Building Poker AI API..." -ForegroundColor Cyan

# Check if maturin is installed
$maturin = poetry run which maturin 2>$null
if (-not $maturin) {
    Write-Host "Installing build dependencies..." -ForegroundColor Yellow
    poetry install --with build
}

# Build type
$buildType = if ($Release) { "release" } else { "debug" }
Write-Host "Building in $buildType mode..." -ForegroundColor Cyan

# Run maturin develop for local development
if ($Dev) {
    Write-Host "Running maturin develop (editable installation)..." -ForegroundColor Cyan
    poetry run maturin develop
}
else {
    # Run maturin build for release
    Write-Host "Running maturin build..." -ForegroundColor Cyan
    if ($Release) {
        poetry run maturin build --release
    }
    else {
        poetry run maturin build
    }
}

Write-Host "Build completed successfully!" -ForegroundColor Green


