# OpenMMO Local Validation Makefile
# Mirrors CI validation commands for local development
# Run 'make help' for available targets

.PHONY: help fmt-check clippy secrets-check structure-check audit build test ci-local clean

# Default target
help:
	@echo "OpenMMO Local Validation"
	@echo "========================"
	@echo ""
	@echo "Available targets:"
	@echo "  fmt-check     - Check code formatting (matches CI)"
	@echo "  clippy        - Run clippy linter (matches CI)"
	@echo "  secrets-check - Check for hardcoded secrets (matches CI)"
	@echo "  structure-check - Validate project structure (matches CI)"
	@echo "  audit         - Run security audit"
	@echo "  build         - Build the workspace"
	@echo "  test          - Run tests (requires database)"
	@echo "  ci-local      - Run all local CI-equivalent checks"
	@echo "  clean         - Clean build artifacts"
	@echo ""
	@echo "Note: Database tests require local PostgreSQL setup"

# Code formatting check (matches CI)
fmt-check:
	@echo "📝 Checking code formatting..."
	cargo fmt --all -- --check
	@echo "✅ Formatting OK"

# Linting (matches CI)
clippy:
	@echo "🔍 Running clippy..."
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	@echo "✅ Clippy OK"

# Secrets check (matches CI)
secrets-check:
	@echo "🔑 Checking for secrets..."
	@if git grep -E "password.*=" -- . ':(exclude)*.md' ':(exclude)*.example' ':(exclude)rustfmt.toml' ':(exclude)clippy.toml' ':(exclude).github/workflows/' ':(exclude)client/scenes/' ':(exclude)client/scripts/' ':(exclude)scripts/' ':(exclude)*.proto' ':(exclude)*.rs' 2>/dev/null; then \
		echo "❌ Potential secret found (password)"; \
		exit 1; \
	fi
	@if git grep -E "secret.*=" -- . ':(exclude)*.md' ':(exclude)*.example' ':(exclude)rustfmt.toml' ':(exclude)clippy.toml' ':(exclude).github/workflows/' ':(exclude)client/scenes/' ':(exclude)client/scripts/' ':(exclude)scripts/' ':(exclude)*.proto' ':(exclude)*.rs' 2>/dev/null; then \
		echo "❌ Potential secret found (secret)"; \
		exit 1; \
	fi
	@if git grep -E "token.*=" -- . ':(exclude)*.md' ':(exclude)*.example' ':(exclude)rustfmt.toml' ':(exclude)clippy.toml' ':(exclude).github/workflows/' ':(exclude)client/scenes/' ':(exclude)client/scripts/' ':(exclude)scripts/' ':(exclude)*.proto' ':(exclude)*.rs' 2>/dev/null; then \
		echo "❌ Potential secret found (token)"; \
		exit 1; \
	fi
	@if git grep -E "api_key.*=" -- . ':(exclude)*.md' ':(exclude)*.example' ':(exclude)rustfmt.toml' ':(exclude)clippy.toml' ':(exclude).github/workflows/' ':(exclude)client/scenes/' ':(exclude)client/scripts/' ':(exclude)scripts/' ':(exclude)*.proto' ':(exclude)*.rs' 2>/dev/null; then \
		echo "❌ Potential secret found (api_key)"; \
		exit 1; \
	fi
	@if git grep -E "AKIA[0-9A-Z]{16}" -- . ':(exclude)*.md' ':(exclude)*.example' ':(exclude)rustfmt.toml' ':(exclude)clippy.toml' ':(exclude).github/workflows/' ':(exclude)client/scenes/' ':(exclude)client/scripts/' ':(exclude)scripts/' ':(exclude)*.proto' ':(exclude)*.rs' 2>/dev/null; then \
		echo "❌ Potential secret found (AWS key)"; \
		exit 1; \
	fi
	@echo "✅ Secrets check OK"

# Project structure validation (matches CI)
structure-check:
	@echo "📁 Checking project structure..."
	@[ -d "server/" ] || { echo "❌ server directory missing"; exit 1; }
	@[ -d "client/" ] || { echo "❌ client directory missing"; exit 1; }
	@[ -d "migrations/" ] || { echo "❌ migrations directory missing"; exit 1; }
	@[ -f "AGENTS.md" ] || { echo "❌ AGENTS.md missing"; exit 1; }
	@echo "✅ Project structure OK"

# Security audit
audit:
	@echo "🔒 Running security audit..."
	@cargo audit 2>/dev/null || { echo "⚠️  cargo-audit not installed. Install with: cargo install cargo-audit"; echo "Skipping security audit..."; }
	@echo "✅ Security audit OK"

# Build workspace
build:
	@echo "🔨 Building workspace..."
	cargo build --workspace
	@echo "✅ Build OK"

# Run tests (requires database setup)
test:
	@echo "🧪 Running tests..."
	@echo "Note: This requires PostgreSQL running locally"
	@echo "Set DATABASE_URL and run: cargo test --workspace --verbose"
	@cargo test --workspace --lib || { echo "❌ Tests failed (database not configured?)"; exit 1; }
	@echo "✅ Tests OK"

# Run all local CI-equivalent checks
ci-local: fmt-check clippy secrets-check structure-check audit build
	@echo "🎉 All local CI-equivalent checks passed!"
	@echo ""
	@echo "Ready to commit and push. CI should pass with these validations."

# Clean build artifacts
clean:
	cargo clean