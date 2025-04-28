#!/usr/bin/env bash

set -e

REPO="ilijaljubicic/Reqvire"
VERSION="${VERSION:-latest}"

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"

# Detect ARCH
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64) ARCH="amd64" ;;
  arm64|aarch64) ARCH="arm64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Set default INSTALL_DIR
if [ -z "$INSTALL_DIR" ]; then
  if [ "$OS" = "darwin" ]; then
    # macOS Intel or Apple Silicon
    if [ -d "/opt/homebrew/bin" ]; then
      INSTALL_DIR="/opt/homebrew/bin"
    else
      INSTALL_DIR="/usr/local/bin"
    fi
  else
    INSTALL_DIR="/usr/local/bin"
  fi
fi

# Handle VERSION
if [ "$VERSION" == "latest" ]; then
  VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep -oP '"tag_name": "\K(.*)(?=")')
fi

BINARY_NAME="reqvire-${OS}-${ARCH}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY_NAME}"

echo "Downloading $BINARY_NAME version $VERSION from $DOWNLOAD_URL"

curl -fsSL "$DOWNLOAD_URL" -o "$BINARY_NAME"
chmod +x "$BINARY_NAME"
sudo mv "$BINARY_NAME" "$INSTALL_DIR/reqvire"

echo "Installed reqvire to $INSTALL_DIR/reqvire"

