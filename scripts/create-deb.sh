#!/bin/bash

set -e

VERSION="${1:-0.1.0}"
ARCH="${2:-amd64}"  # amd64, arm64, armhf

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}Creating .deb package for C.env v${VERSION} (${ARCH})${NC}"

# Build the binary
echo -e "${YELLOW}Building release binary...${NC}"
cargo build --release

# Create package structure
PKG_DIR="cenv_${VERSION}_${ARCH}"
mkdir -p "${PKG_DIR}/DEBIAN"
mkdir -p "${PKG_DIR}/usr/local/bin"
mkdir -p "${PKG_DIR}/usr/share/doc/cenv"
mkdir -p "${PKG_DIR}/usr/share/man/man1"

# Copy binary
cp target/release/c_env_lang "${PKG_DIR}/usr/local/bin/cenv"
chmod 755 "${PKG_DIR}/usr/local/bin/cenv"

# Create control file
cat > "${PKG_DIR}/DEBIAN/control" << EOF
Package: cenv
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: Your Name <your.email@example.com>
Description: C.env - A simple configuration language and interpreter
 C.env is a lightweight configuration language designed for environment
 configuration with support for imports, type conversion, and more.
Homepage: https://github.com/rzorzal/.c.env
EOF

# Copy documentation
if [ -f "Readme.md" ]; then
    cp Readme.md "${PKG_DIR}/usr/share/doc/cenv/README.md"
fi

if [ -f "LICENSE" ]; then
    cp LICENSE "${PKG_DIR}/usr/share/doc/cenv/copyright"
fi

# Create changelog
cat > "${PKG_DIR}/usr/share/doc/cenv/changelog.gz" << EOF
cenv (${VERSION}) stable; urgency=low

  * Release version ${VERSION}

 -- Your Name <your.email@example.com>  $(date -R)
EOF
gzip -9 "${PKG_DIR}/usr/share/doc/cenv/changelog.gz"

# Build the package
echo -e "${YELLOW}Building .deb package...${NC}"
dpkg-deb --build "${PKG_DIR}"

echo -e "${GREEN}✓ Package created: ${PKG_DIR}.deb${NC}"
echo ""
echo "To install locally:"
echo "  sudo dpkg -i ${PKG_DIR}.deb"
echo ""
echo "To remove:"
echo "  sudo dpkg -r cenv"
