#!/bin/bash
# Usage: ./scripts/create-deb.sh <version> <arch> [binary_path]
#
#   version      Package version, e.g. 0.1.5
#   arch         Debian architecture: amd64 | arm64
#   binary_path  Path to the pre-built cenv binary (optional).
#                If omitted, builds from source with `cargo build --release`.
#
# Examples:
#   ./scripts/create-deb.sh 0.1.5 amd64
#   ./scripts/create-deb.sh 0.1.5 amd64 target/release/cenv

set -e

VERSION="${1:?Usage: $0 <version> <arch> [binary_path]}"
ARCH="${2:?Usage: $0 <version> <arch> [binary_path]}"
BINARY_PATH="${3:-}"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}Creating .deb package — cenv v${VERSION} (${ARCH})${NC}"

# Build from source only if no binary was provided
if [ -z "$BINARY_PATH" ]; then
    echo -e "${YELLOW}No binary provided — building from source...${NC}"
    cargo build --release
    BINARY_PATH="target/release/cenv"
fi

[ -f "$BINARY_PATH" ] || { echo "error: binary not found at '$BINARY_PATH'"; exit 1; }

# Package directory structure
PKG="cenv_${VERSION}_${ARCH}"
mkdir -p "${PKG}/DEBIAN"
mkdir -p "${PKG}/usr/bin"
mkdir -p "${PKG}/usr/share/doc/cenv"

cp "$BINARY_PATH" "${PKG}/usr/bin/cenv"
chmod 755 "${PKG}/usr/bin/cenv"

cat > "${PKG}/DEBIAN/control" <<EOF
Package: cenv
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: Ricardo Zorzal
Description: .env file compiler with JavaScript-like syntax
 Write environment configuration once in a .cenv file and compile
 it into .env files for any environment (production, staging, etc.)
Homepage: https://github.com/rzorzal/.c.env
EOF

[ -f "Readme.md" ] && cp Readme.md "${PKG}/usr/share/doc/cenv/README.md"
[ -f "LICENSE"   ] && cp LICENSE   "${PKG}/usr/share/doc/cenv/copyright"

dpkg-deb --build "${PKG}"
rm -rf "${PKG}"

echo -e "${GREEN}✓ Created: ${PKG}.deb${NC}"
