#!/bin/bash
# Build and Release Script for SysAdmin Tamagotchi (Linux/macOS)
# Usage: ./build-release.sh [dev|release|clean]

set -e

BUILD_TYPE="${1:-dev}"
SKIP_TESTS="${SKIP_TESTS:-false}"
SKIP_LINT="${SKIP_LINT:-false}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
step() {
    echo -e "\n${CYAN}==> $1${NC}"
}

success() {
    echo -e "    ${GREEN}✓ $1${NC}"
}

error() {
    echo -e "    ${RED}✗ $1${NC}"
}

warning() {
    echo -e "    ${YELLOW}⚠ $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    step "Checking prerequisites..."

    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo not found. Install from https://rustup.rs/"
        exit 1
    fi
    success "Rust installed: $(cargo --version)"

    if ! command -v node &> /dev/null; then
        error "Node.js not found. Install from https://nodejs.org/"
        exit 1
    fi
    success "Node.js installed: $(node --version)"

    if ! command -v npm &> /dev/null; then
        error "npm not found"
        exit 1
    fi
    success "npm installed: $(npm --version)"
}

# Clean build artifacts
clean_build() {
    step "Cleaning build artifacts..."

    rm -rf src-tauri/target
    rm -rf ui/dist
    rm -rf ui/node_modules

    success "Clean complete"
}

# Install dependencies
install_dependencies() {
    step "Installing dependencies..."

    cd ui
    npm ci
    success "Frontend dependencies installed"
    cd ..

    cd src-tauri
    cargo fetch
    success "Rust dependencies fetched"
    cd ..
}

# Run linting
run_lint() {
    if [ "$SKIP_LINT" = "true" ]; then
        warning "Skipping linting (SKIP_LINT=true)"
        return
    fi

    step "Running linters..."

    cd src-tauri

    echo "  Checking Rust formatting..."
    cargo fmt --all --check
    success "Rust formatting check passed"

    echo "  Running Clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    success "Clippy passed"

    cd ..
}

# Run tests
run_tests() {
    if [ "$SKIP_TESTS" = "true" ]; then
        warning "Skipping tests (SKIP_TESTS=true)"
        return
    fi

    step "Running tests..."

    cd src-tauri

    cargo test --verbose
    success "All tests passed"

    echo "  Running doc tests..."
    cargo test --doc
    success "Doc tests passed"

    cd ..
}

# Build frontend
build_frontend() {
    step "Building frontend..."

    cd ui
    npm run build
    success "Frontend build complete"
    cd ..
}

# Build Tauri app
build_tauri() {
    local mode=$1
    step "Building Tauri app ($mode mode)..."

    # Install Tauri CLI if not present
    if ! command -v tauri &> /dev/null; then
        echo "  Installing Tauri CLI..."
        npm install -g @tauri-apps/cli@next
    fi

    cd ui

    if [ "$mode" = "release" ]; then
        npm run tauri build
        success "Release build complete"

        echo -e "\n${CYAN}==> Build artifacts:${NC}"

        # Linux AppImage
        if [ -d "../src-tauri/target/release/bundle/appimage" ]; then
            appimage=$(find ../src-tauri/target/release/bundle/appimage -name "*.AppImage" | head -n 1)
            if [ -n "$appimage" ]; then
                echo -e "  AppImage: ${YELLOW}$appimage${NC}"
                size=$(du -h "$appimage" | cut -f1)
                echo "  Size: $size"
            fi
        fi

        # macOS DMG
        if [ -d "../src-tauri/target/release/bundle/dmg" ]; then
            dmg=$(find ../src-tauri/target/release/bundle/dmg -name "*.dmg" | head -n 1)
            if [ -n "$dmg" ]; then
                echo -e "  DMG: ${YELLOW}$dmg${NC}"
                size=$(du -h "$dmg" | cut -f1)
                echo "  Size: $size"
            fi
        fi
    else
        npm run tauri dev
    fi

    cd ..
}

# Run security audit
run_security_audit() {
    step "Running security audit..."

    if ! command -v cargo-audit &> /dev/null; then
        echo "  Installing cargo-audit..."
        cargo install cargo-audit --locked
    fi

    cd src-tauri

    if cargo audit; then
        success "Security audit passed"
    else
        warning "Security audit found issues (review above)"
    fi

    cd ..
}

# Generate checksums
generate_checksums() {
    step "Generating checksums..."

    if [ -d "src-tauri/target/release/bundle/appimage" ]; then
        cd src-tauri/target/release/bundle/appimage
        for file in *.AppImage; do
            if [ -f "$file" ]; then
                sha256sum "$file" > checksums.txt
                success "Checksums generated: $(pwd)/checksums.txt"
                echo -e "  SHA256: ${YELLOW}$(cat checksums.txt)${NC}"
            fi
        done
        cd ../../../../..
    elif [ -d "src-tauri/target/release/bundle/dmg" ]; then
        cd src-tauri/target/release/bundle/dmg
        for file in *.dmg; do
            if [ -f "$file" ]; then
                shasum -a 256 "$file" > checksums.txt
                success "Checksums generated: $(pwd)/checksums.txt"
                echo -e "  SHA256: ${YELLOW}$(cat checksums.txt)${NC}"
            fi
        done
        cd ../../../../..
    else
        warning "No release bundle found"
    fi
}

# Main execution
main() {
    start_time=$(date +%s)

    echo -e "\n${CYAN}╔════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║  SysAdmin Tamagotchi Build Script            ║${NC}"
    echo -e "${CYAN}╚════════════════════════════════════════════════╝${NC}\n"

    check_prerequisites

    case "$BUILD_TYPE" in
        clean)
            clean_build
            ;;
        dev)
            install_dependencies
            run_lint
            run_tests
            build_frontend
            build_tauri "dev"
            ;;
        release)
            install_dependencies
            run_lint
            run_tests
            run_security_audit
            build_frontend
            build_tauri "release"
            generate_checksums
            ;;
        *)
            error "Unknown build type: $BUILD_TYPE"
            echo "Usage: $0 [dev|release|clean]"
            exit 1
            ;;
    esac

    end_time=$(date +%s)
    duration=$((end_time - start_time))
    minutes=$((duration / 60))
    seconds=$((duration % 60))

    echo -e "\n${GREEN}╔════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  Build Complete!                              ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════╝${NC}"
    echo -e "  Duration: ${minutes}m ${seconds}s\n"
}

# Run main function
main
