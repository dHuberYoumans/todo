#!/usr/bin/env sh
set -e

REPO="dHuberYoumans/todo"
BIN="todo"
INSTALL_DIR="$HOME/.local/bin"

VERSION="${VERSION:-latest}"

# ------------------ 
# Detect OS and ARCH
# ------------------
OS="$(uname -s)"
case "$OS" in
  Darwin) OS="apple-darwin" ;;
  Linux)  OS="unknown-linux-gnu" ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

ARCH="$(uname -m)"
case "$ARCH" in
  x86_64)  ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

TARGET="${ARCH}-${OS}"

# ------------------ 
# Resolve version
# ------------------ 
if [ "$VERSION" = "latest" ]; then
  VERSION="$(curl -fsSL https://api.github.com/repos/${REPO}/releases/latest \
    | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p')"
fi
BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"

ARCHIVE="${BIN}-${VERSION}-${TARGET}.tar.gz"
URL="${BASE_URL}/${ARCHIVE}"

# ------------------ 
# Install
# ------------------ 
echo "Installing ${BIN} ${VERSION}..."

mkdir -p "$INSTALL_DIR"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

curl -fsSL "$URL" -o "$TMP_DIR/$ARCHIVE"
tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"

chmod +x "$TMP_DIR/$BIN"
mv "$TMP_DIR/$BIN" "$INSTALL_DIR/$BIN"

echo "✔ Installed to $INSTALL_DIR/$BIN"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo
  echo "⚠️  $INSTALL_DIR is not on your PATH"
  echo "Add this to your shell config:"
  echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo "Installing auto-completions..."
shell=${SHELL##*/}
todo completions install "$shell"
