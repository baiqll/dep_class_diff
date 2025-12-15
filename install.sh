#!/bin/bash
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing dep_class_diff...${NC}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

case "$OS" in
    linux)
        TARGET="${ARCH}-unknown-linux-gnu"
        ;;
    darwin)
        TARGET="${ARCH}-apple-darwin"
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${NC}"
        exit 1
        ;;
esac

echo -e "${YELLOW}Detected platform: $OS-$ARCH (target: $TARGET)${NC}"

# Get latest release
REPO="baiqll/dep_class_diff"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo -e "${RED}Failed to get latest release${NC}"
    exit 1
fi

echo -e "${YELLOW}Latest version: $LATEST_RELEASE${NC}"

# Download URL
BINARY_NAME="dep_class_diff-${TARGET}"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/$BINARY_NAME"

echo -e "${YELLOW}Downloading from: $DOWNLOAD_URL${NC}"

# Download binary
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

if ! curl -fsSL -o dep_class_diff "$DOWNLOAD_URL"; then
    echo -e "${RED}Failed to download binary${NC}"
    exit 1
fi

chmod +x dep_class_diff

# Install to /usr/local/bin or ~/.local/bin
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

echo -e "${YELLOW}Installing to: $INSTALL_DIR${NC}"

if [ -w "$INSTALL_DIR" ]; then
    mv dep_class_diff "$INSTALL_DIR/"
else
    sudo mv dep_class_diff "$INSTALL_DIR/"
fi

# Cleanup
cd - > /dev/null
rm -rf "$TMP_DIR"

echo -e "${GREEN}✓ Installation complete!${NC}"
echo -e "${GREEN}Run 'dep_class_diff --help' to get started${NC}"

# Check if install dir is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Warning: $INSTALL_DIR is not in your PATH${NC}"
    echo -e "${YELLOW}Add this to your ~/.bashrc or ~/.zshrc:${NC}"
    echo -e "${YELLOW}  export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
fi
