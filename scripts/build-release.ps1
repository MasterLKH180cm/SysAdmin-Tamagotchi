# Build and Release Script for SysAdmin Tamagotchi
# PowerShell script for local development builds and releases

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet('dev', 'release', 'clean')]
    [string]$BuildType = 'dev',

    [Parameter(Mandatory=$false)]
    [switch]$SkipTests,

    [Parameter(Mandatory=$false)]
    [switch]$SkipLint,

    [Parameter(Mandatory=$false)]
    [string]$Version
)

$ErrorActionPreference = "Stop"

# Color output functions
function Write-Step {
    param([string]$Message)
    Write-Host "`n==> $Message" -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Message)
    Write-Host "    ✓ $Message" -ForegroundColor Green
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "    ✗ $Message" -ForegroundColor Red
}

function Write-Warning-Custom {
    param([string]$Message)
    Write-Host "    ⚠ $Message" -ForegroundColor Yellow
}

# Check prerequisites
function Test-Prerequisites {
    Write-Step "Checking prerequisites..."

    # Check Rust
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error-Custom "Rust/Cargo not found. Install from https://rustup.rs/"
        exit 1
    }
    $rustVersion = cargo --version
    Write-Success "Rust installed: $rustVersion"

    # Check Node.js
    if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
        Write-Error-Custom "Node.js not found. Install from https://nodejs.org/"
        exit 1
    }
    $nodeVersion = node --version
    Write-Success "Node.js installed: $nodeVersion"

    # Check npm
    if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
        Write-Error-Custom "npm not found"
        exit 1
    }
    $npmVersion = npm --version
    Write-Success "npm installed: v$npmVersion"
}

# Clean build artifacts
function Invoke-Clean {
    Write-Step "Cleaning build artifacts..."

    if (Test-Path "src-tauri\target") {
        Remove-Item -Recurse -Force "src-tauri\target"
        Write-Success "Cleaned src-tauri\target"
    }

    if (Test-Path "ui\dist") {
        Remove-Item -Recurse -Force "ui\dist"
        Write-Success "Cleaned ui\dist"
    }

    if (Test-Path "ui\node_modules") {
        Remove-Item -Recurse -Force "ui\node_modules"
        Write-Success "Cleaned ui\node_modules"
    }

    Write-Success "Clean complete"
}

# Install dependencies
function Install-Dependencies {
    Write-Step "Installing dependencies..."

    # Install frontend dependencies
    Push-Location ui
    try {
        npm ci
        Write-Success "Frontend dependencies installed"
    } catch {
        Write-Error-Custom "Failed to install frontend dependencies: $_"
        exit 1
    } finally {
        Pop-Location
    }

    # Update Cargo dependencies
    Push-Location src-tauri
    try {
        cargo fetch
        Write-Success "Rust dependencies fetched"
    } catch {
        Write-Error-Custom "Failed to fetch Rust dependencies: $_"
        exit 1
    } finally {
        Pop-Location
    }
}

# Run linting
function Invoke-Lint {
    if ($SkipLint) {
        Write-Warning-Custom "Skipping linting (--SkipLint specified)"
        return
    }

    Write-Step "Running linters..."

    Push-Location src-tauri
    try {
        # Check formatting
        Write-Host "  Checking Rust formatting..."
        cargo fmt --all --check
        Write-Success "Rust formatting check passed"

        # Run Clippy
        Write-Host "  Running Clippy..."
        cargo clippy --all-targets --all-features -- -D warnings
        Write-Success "Clippy passed"
    } catch {
        Write-Error-Custom "Linting failed: $_"
        exit 1
    } finally {
        Pop-Location
    }
}

# Run tests
function Invoke-Tests {
    if ($SkipTests) {
        Write-Warning-Custom "Skipping tests (--SkipTests specified)"
        return
    }

    Write-Step "Running tests..."

    Push-Location src-tauri
    try {
        cargo test --verbose
        Write-Success "All tests passed"

        # Run doc tests
        Write-Host "  Running doc tests..."
        cargo test --doc
        Write-Success "Doc tests passed"
    } catch {
        Write-Error-Custom "Tests failed: $_"
        exit 1
    } finally {
        Pop-Location
    }
}

# Build frontend
function Build-Frontend {
    Write-Step "Building frontend..."

    Push-Location ui
    try {
        npm run build
        Write-Success "Frontend build complete"
    } catch {
        Write-Error-Custom "Frontend build failed: $_"
        exit 1
    } finally {
        Pop-Location
    }
}

# Build Tauri app
function Build-TauriApp {
    param([string]$Mode)

    Write-Step "Building Tauri app ($Mode mode)..."

    # Install Tauri CLI if not present
    if (-not (Get-Command tauri -ErrorAction SilentlyContinue)) {
        Write-Host "  Installing Tauri CLI..."
        npm install -g @tauri-apps/cli@next
    }

    Push-Location ui
    try {
        if ($Mode -eq 'release') {
            npm run tauri build
            Write-Success "Release build complete"

            # Show output locations
            Write-Host "`n==> Build artifacts:" -ForegroundColor Cyan

            if (Test-Path "..\src-tauri\target\release\bundle\nsis") {
                $installer = Get-ChildItem -Path "..\src-tauri\target\release\bundle\nsis" -Filter "*.exe" | Select-Object -First 1
                if ($installer) {
                    Write-Host "  NSIS Installer: " -NoNewline
                    Write-Host $installer.FullName -ForegroundColor Yellow

                    # Calculate file size
                    $sizeInMB = [math]::Round($installer.Length / 1MB, 2)
                    Write-Host "  Size: $sizeInMB MB"
                }
            }

            if (Test-Path "..\src-tauri\target\release\bundle\msi") {
                $msi = Get-ChildItem -Path "..\src-tauri\target\release\bundle\msi" -Filter "*.msi" | Select-Object -First 1
                if ($msi) {
                    Write-Host "  MSI Installer: " -NoNewline
                    Write-Host $msi.FullName -ForegroundColor Yellow
                }
            }
        } else {
            npm run tauri dev
        }
    } catch {
        Write-Error-Custom "Tauri build failed: $_"
        exit 1
    } finally {
        Pop-Location
    }
}

# Run security audit
function Invoke-SecurityAudit {
    Write-Step "Running security audit..."

    # Check if cargo-audit is installed
    if (-not (Get-Command cargo-audit -ErrorAction SilentlyContinue)) {
        Write-Host "  Installing cargo-audit..."
        cargo install cargo-audit --locked
    }

    Push-Location src-tauri
    try {
        cargo audit
        Write-Success "Security audit passed"
    } catch {
        Write-Warning-Custom "Security audit found issues (review above)"
    } finally {
        Pop-Location
    }
}

# Generate checksums
function New-Checksums {
    Write-Step "Generating checksums..."

    $bundlePath = "src-tauri\target\release\bundle\nsis"

    if (-not (Test-Path $bundlePath)) {
        Write-Warning-Custom "No release bundle found at $bundlePath"
        return
    }

    $installer = Get-ChildItem -Path $bundlePath -Filter "*.exe" | Select-Object -First 1

    if ($installer) {
        $hash = Get-FileHash -Path $installer.FullName -Algorithm SHA256
        $checksumFile = Join-Path $bundlePath "checksums.txt"

        "$($hash.Hash)  $($installer.Name)" | Out-File -FilePath $checksumFile -Encoding ASCII

        Write-Success "Checksums generated: $checksumFile"
        Write-Host "  SHA256: $($hash.Hash)" -ForegroundColor Yellow
    }
}

# Main execution
function Main {
    $startTime = Get-Date

    Write-Host "`n╔════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║  SysAdmin Tamagotchi Build Script            ║" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

    Test-Prerequisites

    switch ($BuildType) {
        'clean' {
            Invoke-Clean
        }
        'dev' {
            Install-Dependencies
            Invoke-Lint
            Invoke-Tests
            Build-Frontend
            Build-TauriApp -Mode 'dev'
        }
        'release' {
            Install-Dependencies
            Invoke-Lint
            Invoke-Tests
            Invoke-SecurityAudit
            Build-Frontend
            Build-TauriApp -Mode 'release'
            New-Checksums
        }
    }

    $duration = (Get-Date) - $startTime
    $durationFormatted = "{0:mm}m {0:ss}s" -f $duration

    Write-Host "`n╔════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║  Build Complete!                              ║" -ForegroundColor Green
    Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host "  Duration: $durationFormatted`n" -ForegroundColor Gray
}

# Run main function
Main
