#!/bin/bash

# Local CI Testing Script for OpenMMO
# This script runs the same checks as the GitHub Actions CI pipeline locally

set -e

echo "🚀 Running OpenMMO CI Pipeline locally..."
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        exit 1
    fi
}

print_info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

# Check if required tools are installed
check_dependencies() {
    print_info "Checking dependencies..."
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Rust/Cargo not found${NC}"
        exit 1
    fi

    # Note: protoc, godot, and database tools are handled by CI or optional locally
    
    print_status 0 "Dependencies checked"
}

# Rust server checks
run_rust_checks() {
    print_info "Running Rust server checks..."

    # Set environment variables to match CI
    export CARGO_TERM_COLOR=always
    export CARGO_INCREMENTAL=0
    export RUST_BACKTRACE=1
    export CARGO_INCREMENTAL=0
    export RUST_BACKTRACE=1

    echo "📝 Checking formatting..."
    cargo fmt --all -- --check
    print_status $? "Rust formatting check"

    echo "🔍 Running clippy..."
    SQLX_OFFLINE=true cargo clippy --workspace --all-targets --all-features -- -A dead_code
    print_status $? "Clippy linting"

    echo "🏗️  Building workspace..."
    SQLX_OFFLINE=true cargo build --workspace --verbose
    print_status $? "Build workspace"

    echo "🔍 Checking SQLx offline mode..."
    DATABASE_URL="" cargo check -p server
    print_status $? "SQLx offline check"

    echo "🧪 Running tests..."
    # Set up database for tests (simplified local version)
    if command -v docker &> /dev/null; then
        echo "🐳 Starting test database..."
        docker-compose up -d db 2>/dev/null || true
        sleep 5
        export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/openmmo_test"

        echo "🗄️ Running migrations..."
        if command -v sqlx &> /dev/null; then
            sqlx migrate run --source migrations >/dev/null 2>&1 || true
            cargo sqlx prepare --workspace >/dev/null 2>&1 || true
        fi
    fi

    cargo test --workspace --verbose
    print_status $? "Unit tests"

    echo "🔒 Running security audit..."
    if command -v cargo-audit &> /dev/null; then
        cargo audit --ignore RUSTSEC-2023-0071
    else
        echo "Installing cargo-audit..."
        cargo install cargo-audit
        cargo audit --ignore RUSTSEC-2023-0071
    fi
    print_status $? "Security audit"

    echo "📚 Checking documentation build..."
    cargo doc --workspace --no-deps --document-private-items
    print_status $? "Documentation build"
}

# Godot client checks
run_godot_checks() {
    print_info "Running Godot client checks..."

    cd client

    echo "🎬 Checking scene structure..."
    [ -f "scenes/Main.tscn" ] || { echo "Main.tscn not found"; exit 1; }
    [ -f "scenes/GameWorld.tscn" ] || { echo "GameWorld.tscn not found"; exit 1; }
    echo "Godot client validation passed"

    cd ..
}

# Database checks
run_database_checks() {
    print_info "Database checks handled in test execution"
    print_status 0 "Database checks"
}

# Code quality checks
run_quality_checks() {
    print_info "Running code quality checks..."
    
    echo "🔍 Checking for secrets..."
    PATTERNS=(
        "password\s*=\s*['\"][^'\"]*['\"]"
        "secret\s*=\s*['\"][^'\"]*['\"]"
        "token\s*=\s*['\"][^'\"]*['\"]"
        "api_key\s*=\s*['\"][^'\"]*['\"]"
        "AKIA[0-9A-Z]{16}"
    )
        for pattern in "${PATTERNS[@]}"; do
          if git grep -E "$pattern" -- . \
            ':(exclude)*.md' \
            ':(exclude)*.example' \
            ':(exclude)rustfmt.toml' \
            ':(exclude)clippy.toml' \
            ':(exclude).github/workflows/' \
            ':(exclude)client/scenes/' \
            ':(exclude)client/scripts/' \
            ':(exclude)scripts/' \
            ':(exclude)*.proto' \
            ':(exclude)*.rs' \
            ':(exclude).sqlx/' \
            2>/dev/null; then
            print_status 1 "Secret detection"
          fi
        done
    print_status 0 "Secret detection"
    
    echo "📁 Validating project structure..."
    [ -d "server/" ] || { echo "server directory missing"; exit 1; }
    [ -d "client/" ] || { echo "client directory missing"; exit 1; }
    [ -d "migrations/" ] || { echo "migrations directory missing"; exit 1; }
    [ -f "AGENTS.md" ] || { echo "AGENTS.md missing"; exit 1; }
    echo "Project structure validation passed"

    echo "📝 Checking for documentation updates..."
    # Check if code changes require documentation updates
    if git diff --name-only origin/master...HEAD | grep -E "\.(rs|gd)$" 2>/dev/null; then
        if ! git diff --name-only origin/master...HEAD | grep -E "\.(md|txt)$" 2>/dev/null; then
            echo "Warning: Code changes detected but no documentation updates"
        fi
    fi
    echo "Documentation check completed"

    echo "🔍 Checking SQLX query preparation..."
    unset DATABASE_URL  # Unset to force offline mode, matching remote CI
    if command -v sqlx &> /dev/null; then
        cargo sqlx prepare --check --workspace
    else
        echo "Installing sqlx-cli..."
        cargo install sqlx-cli --no-default-features --features native-tls,postgres
        cargo sqlx prepare --check --workspace
    fi
    print_status $? "SQLX query preparation check"

    echo "📄 Checking for unused dependencies..."
    if command -v cargo-machete &> /dev/null; then
        cargo machete
    else
        echo "Installing cargo-machete..."
        cargo install cargo-machete
        cargo machete
    fi
    print_status $? "Unused dependencies check"
}

# Main execution
main() {
    check_dependencies
    run_rust_checks
    run_godot_checks
    run_quality_checks

    echo ""
    echo -e "${GREEN}🎉 All CI checks passed!${NC}"
    echo "Your code is ready to be submitted."
}

# Run main function
main "$@"
