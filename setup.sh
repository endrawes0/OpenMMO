#!/bin/bash

# OpenMMO Development Environment Setup Script
# This script sets up the complete development environment for OpenMMO

set -e

echo "🎮 OpenMMO Development Environment Setup"
echo "========================================"

# Check if we're running on a supported system
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "✅ Detected Linux system"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "✅ Detected macOS system"
else
    echo "⚠️  Warning: Untested operating system: $OSTYPE"
fi

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check and install Rust
if ! command_exists rustc; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust is already installed"
fi

# Install SQLx CLI if not present
if ! command_exists cargo-sqlx; then
    echo "📦 Installing SQLx CLI..."
    source "$HOME/.cargo/env"
    cargo install sqlx-cli --no-default-features --features native-tls,postgres
else
    echo "✅ SQLx CLI is already installed"
fi

# Check Docker installation
if ! command_exists docker; then
    echo "🐳 Docker not found. Please install Docker:"
    echo ""
    echo "  Ubuntu/Debian:"
    echo "    sudo apt-get update"
    echo "    sudo apt-get install -y docker.io docker-compose"
    echo "    sudo usermod -aG docker \$USER"
    echo "    # Log out and back in for group changes to take effect"
    echo ""
    echo "  macOS:"
    echo "    # Install Docker Desktop from https://www.docker.com/products/docker-desktop"
    echo ""
    echo "  Or visit: https://docs.docker.com/get-docker/"
    echo ""
    echo "After installing Docker, run this script again."
    exit 1
else
    echo "✅ Docker is installed"
fi

# Check Docker Compose
if ! command_exists docker-compose; then
    echo "⚠️  Docker Compose not found. Installing..."
    if command_exists docker; then
        # Try to use docker compose (newer syntax)
        if docker compose version >/dev/null 2>&1; then
            echo "✅ Using 'docker compose' (newer syntax)"
            # Create an alias for compatibility
            echo 'alias docker-compose="docker compose"' >> ~/.bashrc
        else
            echo "❌ Docker Compose is not available. Please install it."
            exit 1
        fi
    fi
else
    echo "✅ Docker Compose is installed"
fi

# Set up environment file
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp .env.example .env
    echo "✅ Created .env file"
else
    echo "✅ .env file already exists"
fi

# Start the database
echo "🗄️  Starting PostgreSQL database..."
if command_exists docker-compose; then
    docker-compose up -d db
elif docker compose version >/dev/null 2>&1; then
    docker compose up -d db
else
    echo "❌ Cannot start database - Docker Compose not available"
    exit 1
fi

# Wait for database to be ready
echo "⏳ Waiting for database to be ready..."
sleep 10

# Run database migrations
echo "🔄 Running database migrations..."
cd server
source "$HOME/.cargo/env"
cargo sqlx migrate run

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Next steps:"
echo "1. Start the server: cd server && cargo run"
echo "2. Check health: curl http://localhost:8080/health"
echo "3. Check database: curl http://localhost:8080/health/db"
echo ""
echo "For more information, see docs/database-setup.md"