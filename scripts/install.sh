#!/usr/bin/env bash
set -e

REPO="Reqvire/reqvire"
VERSION="${VERSION:-latest}"

# 1) detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"

# 2) detect ARCH suffix
ARCH_RAW="$(uname -m)"
case "$ARCH_RAW" in
  x86_64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="arm64" ;;
  *) echo "Unsupported architecture: $ARCH_RAW"; exit 1 ;;
esac

# 3) choose extension + archive name
case "$OS" in
  linux|darwin)
    EXT="tar.gz"
    PLATFORM="${OS}-${ARCH}"
    ;;
  msys*|mingw*|cygwin*)
    EXT="zip"
    PLATFORM="windows-${ARCH}"
    ;;
  *)
    echo "Unsupported OS: $OS"; exit 1
    ;;
esac

ARCHIVE="reqvire-${PLATFORM}.${EXT}"

# 4) resolve "latest" → actual tag
if [ "$VERSION" = "latest" ]; then
  VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
    | sed -nE 's/.*"tag_name": *"([^"]+)".*/\1/p')
fi

URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE}"
echo "Downloading $ARCHIVE (version $VERSION) from $URL…"

curl -fsSL "$URL" -o "$ARCHIVE"

# 5) unpack into temp dir
TMPDIR="$(mktemp -d)"
if [ "$EXT" = "tar.gz" ]; then
  tar -xzf "$ARCHIVE" -C "$TMPDIR"
else
  unzip -q "$ARCHIVE" -d "$TMPDIR"
fi

# the extracted file will be named "reqvire-${PLATFORM}" (or .exe on Windows)
EXTRACTED="$(find "$TMPDIR" -maxdepth 1 -type f -name "reqvire*")"
chmod +x "$EXTRACTED"

# 6) pick install dir
if [ -z "$INSTALL_DIR" ]; then
  if [ "$OS" = "darwin" ]; then
    # prefer Homebrew prefix on Apple Silicon
    if [ -d "/opt/homebrew/bin" ]; then
      INSTALL_DIR="/opt/homebrew/bin"
    else
      INSTALL_DIR="/usr/local/bin"
    fi
  else
    INSTALL_DIR="/usr/local/bin"
  fi
fi

echo "Installing to $INSTALL_DIR/reqvire"
sudo mv "$EXTRACTED" "$INSTALL_DIR/reqvire"

# 7) cleanup
rm -rf "$TMPDIR" "$ARCHIVE"

echo "✔ Installed reqvire $VERSION to $INSTALL_DIR/reqvire"

