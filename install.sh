#!/bin/bash

# witr-rs Installer Script
# Usage: curl -fsSL https://raw.githubusercontent.com/rewrite-everything-in-rust/witr-rs/main/install.sh | sudo bash

set -e

REPO="rewrite-everything-in-rust/witr-rs"
BINARY_NAME="witr-rs"
INSTALL_DIR="/usr/local/bin"

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux)
        OS_TYPE="linux"
        ;;
    Darwin)
        OS_TYPE="macos"
        ;;
    *)
        echo "Error: Unsupported OS ($OS)"
        exit 1
        ;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64)
        ARCH_TYPE="amd64"
        ;;
    aarch64|arm64)
        ARCH_TYPE="arm64"
        ;;
    *)
        echo "Error: Unsupported Architecture ($ARCH)"
        exit 1
        ;;
esac

# Construct Download URL (using 'latest' magic URL)
ASSET_NAME="${BINARY_NAME}-${OS_TYPE}-${ARCH_TYPE}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET_NAME}"

echo "Downloading ${ASSET_NAME}..."
curl -L --fail "$DOWNLOAD_URL" -o "/tmp/${BINARY_NAME}"

echo "Installing to ${INSTALL_DIR}..."
chmod +x "/tmp/${BINARY_NAME}"
mv "/tmp/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"

echo "witr-rs installed successfully!"
echo "   Run 'witr-rs --help' to get started."
