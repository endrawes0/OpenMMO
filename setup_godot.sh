#!/bin/bash

# Godot 4.x Installation Script for OpenMMO Client
# This script helps install Godot 4.x for development

set -e

echo "OpenMMO - Godot 4.x Setup Script"
echo "=================================="

# Check if Godot is already installed
if command -v godot &> /dev/null; then
    echo "Godot is already installed:"
    godot --version
    echo ""
    echo "You can now open the client project:"
    echo "  cd client && godot --editor"
    exit 0
fi

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Detected Linux"
    
    # Try to download Godot binary
    GODOT_VERSION="4.3"
    GODOT_URL="https://github.com/godotengine/godot/releases/download/${GODOT_VERSION}-stable/Godot_v${GODOT_VERSION}-stable_linux.x86_64.zip"
    
    echo "Downloading Godot ${GODOT_VERSION}..."
    wget -O godot.zip "$GODOT_URL"
    
    echo "Extracting Godot..."
    unzip godot.zip
    chmod +x Godot*
    
    echo "Installing to /usr/local/bin (requires sudo)..."
    sudo mv Godot* /usr/local/bin/godot
    
    echo "Cleaning up..."
    rm godot.zip
    
    echo "Godot installed successfully!"
    godot --version
    
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected macOS"
    echo "Please download Godot from: https://godotengine.org/download/macos/"
    echo "Or use Homebrew: brew install --cask godot"
    
else
    echo "Detected Windows or other OS"
    echo "Please download Godot from: https://godotengine.org/download/windows/"
fi

echo ""
echo "Once Godot is installed, you can open the client project:"
echo "  cd client"
echo "  godot --editor"