# Deployment Guide - SysAdmin Tamagotchi

## Table of Contents
- [Overview](#overview)
- [Build Requirements](#build-requirements)
- [Local Development Build](#local-development-build)
- [Production Release Build](#production-release-build)
- [CI/CD Pipeline](#cicd-pipeline)
- [Distribution](#distribution)
- [Code Signing](#code-signing)
- [Versioning Strategy](#versioning-strategy)
- [Troubleshooting](#troubleshooting)

## Overview

SysAdmin Tamagotchi is a Tauri 2.x desktop application with:
- **Backend**: Rust (src-tauri/)
- **Frontend**: Svelte + Vite (ui/)
- **Primary Platform**: Windows 10/11 (64-bit)
- **Optional Platforms**: Linux, macOS

## Build Requirements

### Windows

| Requirement | Version | Purpose |
|------------|---------|---------|
| **Rust** | 1.70+ | Backend compilation |
| **Node.js** | 20.x LTS | Frontend build |
| **npm** | 10.x | Package management |
| **WebView2** | Latest | Runtime for Tauri |
| **Visual Studio Build Tools** | 2019+ | Rust compilation on Windows |

#### Installation Steps (Windows)

1. **Install Rust**
   ```powershell
   # Using rustup installer
   winget install Rustlang.Rustup
   # OR download from https://rustup.rs/

   # Verify installation
   cargo --version
   rustc --version
   ```

2. **Install Node.js**
   ```powershell
   winget install OpenJS.NodeJS.LTS
   # OR download from https://nodejs.org/

   # Verify installation
   node --version
   npm --version
   ```

3. **Install Visual Studio Build Tools**
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   # During installation, select "Desktop development with C++"
   ```

4. **Install WebView2 Runtime** (usually pre-installed on Windows 11)
   ```powershell
   winget install Microsoft.EdgeWebView2Runtime
   ```

### Linux (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Tauri dependencies
sudo apt-get update
sudo apt-get install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf
```

### macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (using Homebrew)
brew install node@20

# Install Xcode Command Line Tools
xcode-select --install
```

## Local Development Build

### Quick Start

**Windows:**
```powershell
# Clone repository
git clone https://github.com/yourusername/SysAdmin-Tamagotchi.git
cd SysAdmin-Tamagotchi

# Install dependencies
cd ui
npm install
cd ..

# Run development build
.\scripts\build-release.ps1 -BuildType dev
```

**Linux/macOS:**
```bash
# Clone repository
git clone https://github.com/yourusername/SysAdmin-Tamagotchi.git
cd SysAdmin-Tamagotchi

# Install dependencies
cd ui
npm install
cd ..

# Make script executable
chmod +x scripts/build-release.sh

# Run development build
./scripts/build-release.sh dev
```

### Development Workflow

1. **Start development server**
   ```bash
   cd ui
   npm run dev
   ```
   This starts Vite dev server at http://localhost:5173

2. **In another terminal, start Tauri dev mode**
   ```bash
   cd ui
   npm run tauri dev
   ```
   This launches the app with hot-reload enabled.

3. **Run tests**
   ```bash
   cd src-tauri
   cargo test
   ```

4. **Run linting**
   ```bash
   cd src-tauri
   cargo fmt --all --check
   cargo clippy --all-targets --all-features
   ```

## Production Release Build

### Using Build Scripts

**Windows:**
```powershell
# Full release build with all checks
.\scripts\build-release.ps1 -BuildType release

# Skip tests (faster, not recommended)
.\scripts\build-release.ps1 -BuildType release -SkipTests

# Skip linting
.\scripts\build-release.ps1 -BuildType release -SkipLint
```

**Linux/macOS:**
```bash
# Full release build
./scripts/build-release.sh release

# Skip tests
SKIP_TESTS=true ./scripts/build-release.sh release

# Skip linting
SKIP_LINT=true ./scripts/build-release.sh release
```

### Manual Build Process

1. **Install frontend dependencies**
   ```bash
   cd ui
   npm ci
   ```

2. **Build frontend**
   ```bash
   npm run build
   ```

3. **Build Tauri release**
   ```bash
   npm install -g @tauri-apps/cli@next
   npm run tauri build
   ```

4. **Locate installers**
   - **Windows NSIS**: `src-tauri/target/release/bundle/nsis/*.exe`
   - **Windows MSI**: `src-tauri/target/release/bundle/msi/*.msi`
   - **Linux AppImage**: `src-tauri/target/release/bundle/appimage/*.AppImage`
   - **macOS DMG**: `src-tauri/target/release/bundle/dmg/*.dmg`

### Build Optimization

The release profile in `src-tauri/Cargo.toml` is configured for optimal performance:

```toml
[profile.release]
opt-level = "z"       # Optimize for size (typical binary: 5-8 MB)
lto = true            # Link Time Optimization (20-30% size reduction)
codegen-units = 1     # Single codegen unit (better optimization)
strip = true          # Remove debug symbols (50% size reduction)
```

**Expected Build Times** (Windows, Ryzen 5/i5 equivalent):
- Clean build: 3-5 minutes
- Incremental build: 30-60 seconds
- With LTO: +1-2 minutes

**Expected Binary Sizes**:
- Windows installer (NSIS): 6-10 MB
- Windows MSI: 7-12 MB
- Linux AppImage: 8-15 MB
- macOS DMG: 10-18 MB

## CI/CD Pipeline

### GitHub Actions Workflows

The project includes two automated workflows:

#### 1. CI Workflow (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `master` or `develop` branches
- Pull requests to `master`
- Manual workflow dispatch

**Jobs:**
- **security-audit**: Runs `cargo audit` for vulnerability scanning
- **lint**: Checks Rust formatting and Clippy
- **test-windows**: Runs all tests on Windows
- **build-windows**: Creates release artifacts on push to master
- **build-linux**: Optional Linux builds (manual trigger)
- **build-macos**: Optional macOS builds (manual trigger)

**Artifacts Retention**: 30 days

#### 2. Release Workflow (`.github/workflows/release.yml`)

**Triggers:**
- Git tags matching `v*.*.*` (e.g., `v0.1.0`)
- Manual workflow dispatch with version input

**Process:**
1. Creates GitHub Release (draft)
2. Builds Windows installer
3. Uploads installer to release
4. Generates SHA256 checksums
5. Optionally builds Linux/macOS variants

### Running CI Locally

**Windows:**
```powershell
# Simulate CI pipeline
.\scripts\build-release.ps1 -BuildType release
```

**Linux/macOS:**
```bash
./scripts/build-release.sh release
```

### Setting Up CI Secrets

For code signing and secure releases, configure these GitHub Secrets:

| Secret Name | Purpose | Required |
|------------|---------|----------|
| `TAURI_SIGNING_PRIVATE_KEY` | Code signing key | Optional |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Key password | Optional |

**Generate signing keys:**
```bash
# Install Tauri CLI
npm install -g @tauri-apps/cli@next

# Generate key pair
tauri signer generate -w ~/.tauri/myapp.key

# Add public key to tauri.conf.json under `bundle.publisher`
# Add private key to GitHub Secrets
```

## Distribution

### Windows Distribution

#### NSIS Installer (Recommended)

**Features:**
- Single-file installer
- User/system-wide installation
- Automatic WebView2 installation
- Add to Start Menu
- Desktop shortcut (optional)
- Uninstaller

**Distribution:**
```powershell
# Installer location
src-tauri\target\release\bundle\nsis\SysAdmin-Tamagotchi_0.1.0_x64-setup.exe

# Distribute via:
# - GitHub Releases
# - Company intranet
# - USB drives
# - Software deployment tools (SCCM, Intune)
```

**Silent Installation:**
```powershell
# Silent install for system-wide deployment
.\SysAdmin-Tamagotchi_0.1.0_x64-setup.exe /S /ALLUSERS

# Silent install for current user
.\SysAdmin-Tamagotchi_0.1.0_x64-setup.exe /S /CURRENTUSER
```

#### MSI Installer (Enterprise)

**Features:**
- Group Policy deployment
- Active Directory integration
- Corporate IT management

**Distribution:**
```powershell
# MSI location
src-tauri\target\release\bundle\msi\SysAdmin-Tamagotchi_0.1.0_x64_en-US.msi

# Silent install
msiexec /i SysAdmin-Tamagotchi_0.1.0_x64_en-US.msi /quiet /norestart
```

### Linux Distribution

#### AppImage (Universal)

```bash
# AppImage location
src-tauri/target/release/bundle/appimage/sysadmin-tamagotchi_0.1.0_amd64.AppImage

# Make executable
chmod +x sysadmin-tamagotchi_0.1.0_amd64.AppImage

# Run
./sysadmin-tamagotchi_0.1.0_amd64.AppImage
```

### macOS Distribution

#### DMG (Disk Image)

```bash
# DMG location
src-tauri/target/release/bundle/dmg/SysAdmin-Tamagotchi_0.1.0_x64.dmg

# Mount and install
open SysAdmin-Tamagotchi_0.1.0_x64.dmg
# Drag app to Applications folder
```

### Checksum Verification

Users should verify downloads using SHA256 checksums:

```powershell
# Windows (PowerShell)
Get-FileHash SysAdmin-Tamagotchi_0.1.0_x64-setup.exe -Algorithm SHA256

# Linux/macOS
sha256sum sysadmin-tamagotchi_0.1.0_amd64.AppImage
```

Compare output with published checksums in `checksums-v0.1.0.txt`.

## Code Signing

### Windows Code Signing

**Why Sign?**
- Removes SmartScreen warnings
- Establishes publisher identity
- Required for some enterprise environments

**Options:**

#### 1. Self-Signed Certificate (Development/Testing)

```powershell
# Create self-signed cert
$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=YourCompany"

# Export certificate
Export-Certificate -Cert $cert -FilePath "code-signing-cert.cer"

# Sign installer (requires signtool from Windows SDK)
signtool sign /f code-signing-cert.pfx /p password /t http://timestamp.digicert.com SysAdmin-Tamagotchi_0.1.0_x64-setup.exe
```

**Note**: Self-signed certificates require users to trust the certificate manually.

#### 2. Commercial Certificate (Production)

Purchase from:
- DigiCert
- Sectigo
- GlobalSign

**Cost**: $100-$500/year

**Signing Process:**
```powershell
# Sign with commercial cert
signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com /td sha256 /fd sha256 SysAdmin-Tamagotchi_0.1.0_x64-setup.exe

# Verify signature
signtool verify /pa SysAdmin-Tamagotchi_0.1.0_x64-setup.exe
```

#### 3. Tauri Built-in Signing

Configure in `tauri.conf.json`:

```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  }
}
```

Set environment variables during build:
```powershell
$env:TAURI_SIGNING_PRIVATE_KEY = Get-Content ~/.tauri/myapp.key -Raw
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = "your_password"
npm run tauri build
```

### macOS Code Signing

```bash
# Sign app bundle
codesign --force --deep --sign "Developer ID Application: Your Name" SysAdmin-Tamagotchi.app

# Verify signature
codesign --verify --deep --strict --verbose=2 SysAdmin-Tamagotchi.app

# Notarize with Apple (required for macOS 10.15+)
xcrun notarytool submit SysAdmin-Tamagotchi_0.1.0_x64.dmg --apple-id your-email --password app-specific-password --team-id TEAM_ID
```

## Versioning Strategy

### Semantic Versioning (SemVer)

Format: `MAJOR.MINOR.PATCH` (e.g., `0.1.0`, `1.0.0`, `1.2.3`)

- **MAJOR**: Breaking changes, incompatible API updates
- **MINOR**: New features, backward-compatible
- **PATCH**: Bug fixes, backward-compatible

### Pre-release Versions

- **Alpha**: `v0.1.0-alpha.1` (internal testing)
- **Beta**: `v0.1.0-beta.1` (public testing)
- **Release Candidate**: `v0.1.0-rc.1` (final testing)

### Version Update Process

1. **Update version in files:**
   ```toml
   # src-tauri/Cargo.toml
   [package]
   version = "0.2.0"
   ```

   ```json
   // src-tauri/tauri.conf.json
   {
     "version": "0.2.0"
   }
   ```

   ```json
   // ui/package.json
   {
     "version": "0.2.0"
   }
   ```

2. **Commit version bump:**
   ```bash
   git add .
   git commit -m "chore: bump version to v0.2.0"
   ```

3. **Create and push tag:**
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin master --tags
   ```

4. **GitHub Actions automatically creates release**

### Changelog Management

Maintain `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/):

```markdown
# Changelog

## [0.2.0] - 2026-01-20

### Added
- New pet animation states
- Disk space monitoring

### Changed
- Improved CPU polling accuracy

### Fixed
- Memory leak in system monitor

### Security
- Updated sysinfo dependency to patch CVE-2026-XXXX
```

## Troubleshooting

### Build Errors

#### Error: "WebView2 not found"
**Solution:**
```powershell
winget install Microsoft.EdgeWebView2Runtime
```

#### Error: "link.exe not found"
**Solution:**
```powershell
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
# Select "Desktop development with C++" workload
```

#### Error: "VCRUNTIME140.dll missing"
**Solution:**
```powershell
winget install Microsoft.VCRedist.2015+.x64
```

#### Error: "npm ERR! code ELIFECYCLE"
**Solution:**
```bash
# Clean node_modules and reinstall
cd ui
rm -rf node_modules package-lock.json
npm install
```

#### Error: "cargo: 'tauri' is not a Cargo command"
**Solution:**
```bash
# Install Tauri CLI globally
npm install -g @tauri-apps/cli@next
```

### Runtime Issues

#### Issue: App won't start on Windows
**Diagnostics:**
```powershell
# Check WebView2 installation
reg query "HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" /v pv

# Run from command line to see error messages
.\SysAdmin-Tamagotchi.exe
```

#### Issue: High memory usage
**Solution:**
- Check for memory leaks in Rust code
- Use `cargo flamegraph` to profile
- Review system monitor polling intervals

#### Issue: Installer blocked by SmartScreen
**Solution:**
- Sign the installer with valid code signing certificate
- Request Microsoft SmartScreen reputation building
- Distribute via trusted channels

### CI/CD Issues

#### Issue: GitHub Actions build fails
**Diagnostics:**
1. Check workflow logs in GitHub Actions tab
2. Verify Rust version compatibility
3. Check dependency versions
4. Review error messages carefully

**Common fixes:**
```yaml
# Update Rust toolchain in workflow
- uses: dtolnay/rust-toolchain@stable
  with:
    toolchain: 1.75.0  # Pin specific version
```

#### Issue: Artifact upload fails
**Solution:**
```yaml
# Verify path exists before upload
- name: Check bundle exists
  run: |
    if (!(Test-Path "src-tauri/target/release/bundle/nsis/*.exe")) {
      Write-Error "No installer found"
      exit 1
    }
```

### Performance Optimization

#### Build Time Reduction

1. **Use incremental compilation (dev builds)**
   ```toml
   [profile.dev]
   incremental = true
   ```

2. **Enable shared build cache**
   ```yaml
   # In GitHub Actions
   - uses: Swatinem/rust-cache@v2
     with:
       workspaces: src-tauri
       shared-key: "build-cache"
   ```

3. **Parallel frontend builds**
   ```bash
   # Use multiple CPU cores
   npm run build -- --parallel
   ```

#### Binary Size Reduction

Current optimizations already applied:
- `opt-level = "z"` (size optimization)
- `lto = true` (link-time optimization)
- `strip = true` (remove debug symbols)

**Further reductions:**
```toml
[profile.release]
panic = "abort"  # Remove panic unwinding code
```

**Trade-offs**: Increased compile time, no panic backtraces

## Support and Resources

### Documentation
- [Tauri Documentation](https://tauri.app/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Svelte Documentation](https://svelte.dev/)

### Community
- GitHub Issues: Report bugs and request features
- GitHub Discussions: Ask questions and share ideas

### Monitoring and Telemetry

**Recommended tools** (privacy-respecting):
- Crash reporting: Sentry (self-hosted)
- Analytics: Plausible (self-hosted)
- Logging: structured logs with `tracing` crate

**Example logging setup:**
```rust
// src-tauri/src/main.rs
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // ... rest of app
}
```

---

**Last Updated**: 2026-01-13
**Maintainer**: SysAdmin Tamagotchi Contributors
**License**: MIT
