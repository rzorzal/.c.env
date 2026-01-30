#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default installation directory
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo -e "${GREEN}C.env Language Installer${NC}"
echo "================================"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo is not installed.${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo -e "${YELLOW}Building C.env in release mode...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Build successful${NC}"
echo ""

# Create installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Copy binary
echo -e "${YELLOW}Installing cenv to $INSTALL_DIR...${NC}"
cp target/release/cenv "$INSTALL_DIR/cenv"
chmod +x "$INSTALL_DIR/cenv"

echo -e "${GREEN}✓ Binary installed${NC}"
echo ""

# Check if INSTALL_DIR is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}$INSTALL_DIR is not in your PATH. Adding it now...${NC}"

    # Detect shell and configuration file
    SHELL_CONFIG=""
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        if [ -f "$HOME/.bashrc" ]; then
            SHELL_CONFIG="$HOME/.bashrc"
        else
            SHELL_CONFIG="$HOME/.bash_profile"
        fi
    elif [ -f "$HOME/.profile" ]; then
        SHELL_CONFIG="$HOME/.profile"
    fi

    if [ -n "$SHELL_CONFIG" ]; then
        # Check if the PATH export is already in the config file
        if ! grep -q "export PATH.*$INSTALL_DIR" "$SHELL_CONFIG" 2>/dev/null; then
            echo "" >> "$SHELL_CONFIG"
            echo "# Added by C.env installer" >> "$SHELL_CONFIG"
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
            echo -e "${GREEN}✓ Added $INSTALL_DIR to PATH in $SHELL_CONFIG${NC}"
            echo ""
            echo -e "${YELLOW}Applying changes to current shell...${NC}"
            # Source the config file to apply changes immediately
            export PATH="$PATH:$INSTALL_DIR"
            echo -e "${GREEN}✓ PATH updated in current shell${NC}"
        else
            echo -e "${GREEN}✓ PATH already configured in $SHELL_CONFIG${NC}"
        fi
    else
        echo -e "${YELLOW}Could not detect shell configuration file.${NC}"
        echo "Please manually add the following line to your shell configuration:"
        echo -e "${GREEN}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
        # Still add to current session
        export PATH="$PATH:$INSTALL_DIR"
    fi
else
    echo -e "${GREEN}✓ $INSTALL_DIR is already in your PATH${NC}"
fi

echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo ""

# Since we can't modify the parent shell's PATH from a subprocess,
# provide instructions to activate cenv in the current shell
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}To use cenv in this shell session, run:${NC}"
    echo -e "${GREEN}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    echo ""
    echo "Or start a new terminal session."
    echo ""
fi

echo "Run 'cenv --help' to get started"
echo "Example: cenv examples/hello.cenv"
