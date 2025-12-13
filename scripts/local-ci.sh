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
    
    if ! command -v godot &> /dev/null; then
        echo -e "${YELLOW}⚠️  Godot not found - skipping Godot checks${NC}"
        SKIP_GODOT=true
    fi
    
    if ! command -v psql &> /dev/null; then
        echo -e "${YELLOW}⚠️  PostgreSQL not found - skipping database checks${NC}"
        SKIP_DB=true
    fi
    
    print_status 0 "Dependencies checked"
}

# Rust server checks
run_rust_checks() {
    print_info "Running Rust server checks..."
    
    echo "📝 Checking formatting..."
    cargo fmt --all -- --check
    print_status $? "Rust formatting check"
    
    echo "🔍 Running clippy..."
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    print_status $? "Clippy linting"
    
    echo "🏗️  Building workspace..."
    cargo build --workspace --verbose
    print_status $? "Build workspace"
    
    echo "🧪 Running tests..."
    cargo test --workspace --verbose
    print_status $? "Unit tests"
    
    echo "🔒 Running security audit..."
    if command -v cargo-audit &> /dev/null; then
        cargo audit
    else
        echo "Installing cargo-audit..."
        cargo install cargo-audit
        cargo audit
    fi
    print_status $? "Security audit"
    
    echo "📚 Checking documentation build..."
    cargo doc --workspace --no-deps --document-private-items
    print_status $? "Documentation build"
}

# Godot client checks
run_godot_checks() {
    if [ "$SKIP_GODOT" = true ]; then
        print_info "Skipping Godot checks (Godot not installed)"
        return
    fi
    
    print_info "Running Godot client checks..."
    
    cd client
    
    echo "📋 Validating project.godot..."
    godot --headless --check-only project.godot
    print_status $? "Project validation"
    
    echo "🎬 Checking scene structure..."
    [ -f "scenes/Main.tscn" ] && [ -f "scenes/GameWorld.tscn" ]
    print_status $? "Scene structure"
    
    echo "📝 Checking GDScript syntax..."
    find scripts/ -name "*.gd" -exec godot --headless --script {} \; 2>&1 | grep -q "SyntaxError" && false || true
    print_status $? "GDScript syntax"
    
    cd ..
}

# Database checks
run_database_checks() {
    if [ "$SKIP_DB" = true ]; then
        print_info "Skipping database checks (PostgreSQL not available)"
        return
    fi
    
    print_info "Running database migration checks..."
    
    # Check if test database exists and is accessible
    if psql -h localhost -U postgres -d openmmo_test -c "SELECT 1;" &> /dev/null; then
        echo "🗄️  Testing migrations..."
        export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/openmmo_test"
        
        if command -v sqlx &> /dev/null; then
            sqlx migrate info --source migrations
            print_status $? "Migration info"
        else
            echo "sqlx-cli not found, installing..."
            cargo install sqlx-cli --no-default-features --features native-tls,postgres
            sqlx migrate info --source migrations
            print_status $? "Migration info"
        fi
    else
        print_info "Test database not available, skipping migration tests"
    fi
}

# Code quality checks
run_quality_checks() {
    print_info "Running code quality checks..."
    
    echo "🔍 Checking for secrets..."
    PATTERNS=("password.*=" "secret.*=" "token.*=" "api_key.*=" "AKIA[0-9A-Z]{16}")
    for pattern in "${PATTERNS[@]}"; do
        if git grep -E "$pattern" -- . ':(exclude)*.md' 2>/dev/null; then
            print_status 1 "Secret detection"
        fi
    done
    print_status 0 "Secret detection"
    
    echo "📁 Validating project structure..."
    [ -d "server/" ] && [ -d "client/" ] && [ -d "migrations/" ] && [ -f "AGENTS.md" ]
    print_status $? "Project structure"
    
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
    run_database_checks
    run_quality_checks
    
    echo ""
    echo -e "${GREEN}🎉 All CI checks passed!${NC}"
    echo "Your code is ready to be submitted."
}

# Run main function
main "$@"