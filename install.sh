#!/bin/sh
# cenv installer
# Downloads the pre-built binary from GitHub Releases and installs it.
# Usage: curl -sSf https://raw.githubusercontent.com/rzorzal/.c.env/main/install.sh | sh

set -e

REPO="rzorzal/.c.env"
BINARY="cenv"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
SHELL_CONFIG=""   # set by add_to_path, used in the final message

# ── colors ────────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

info()  { printf "${GREEN}  ✓ %s${NC}\n" "$*"; }
step()  { printf "\n${BOLD}%s${NC}\n" "$*"; }
warn()  { printf "${YELLOW}  ! %s${NC}\n" "$*"; }
error() { printf "\n${RED}error: %s${NC}\n" "$*" >&2; exit 1; }

# ── detect OS ─────────────────────────────────────────────────────────────────
detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "Linux" ;;
        Darwin*) echo "macOS" ;;
        *) error "Unsupported OS: $(uname -s). Install manually: https://github.com/${REPO}/releases" ;;
    esac
}

# ── detect architecture ───────────────────────────────────────────────────────
detect_arch() {
    case "$(uname -m)" in
        x86_64 | amd64)  echo "x86_64" ;;
        aarch64 | arm64) echo "aarch64" ;;
        *) error "Unsupported architecture: $(uname -m). Install manually: https://github.com/${REPO}/releases" ;;
    esac
}

# ── check for a download tool ─────────────────────────────────────────────────
check_downloader() {
    if command -v curl > /dev/null 2>&1; then
        echo "curl"
    elif command -v wget > /dev/null 2>&1; then
        echo "wget"
    else
        error "Neither curl nor wget found. Please install one and try again."
    fi
}

# ── fetch text from a URL ─────────────────────────────────────────────────────
fetch() {
    url="$1"
    downloader="$2"
    if [ "$downloader" = "curl" ]; then
        curl -fsSL "$url"
    else
        wget -qO- "$url"
    fi
}

# ── download a URL to a file ──────────────────────────────────────────────────
download() {
    url="$1"
    dest="$2"
    downloader="$3"
    if [ "$downloader" = "curl" ]; then
        curl -fsSL "$url" -o "$dest"
    else
        wget -q "$url" -O "$dest"
    fi
}

# ── find the download URL for the right asset in a release JSON ──────────────
# Searches browser_download_url lines for entries matching both OS and arch keywords.
# This is resilient to naming changes (e.g. extra version suffixes like ".1.").
find_asset_url() {
    release_json="$1"
    os_keyword="$2"    # e.g. "macOS" or "Linux" or "amd64"
    arch_keyword="$3"  # e.g. "x86_64" or "aarch64" or "arm64"

    echo "$release_json" \
        | grep '"browser_download_url"' \
        | grep "$os_keyword" \
        | grep "$arch_keyword" \
        | sed -E 's/.*"browser_download_url": *"([^"]+)".*/\1/' \
        | head -1
}

# ── resolve the user's shell config file ─────────────────────────────────────
resolve_shell_config() {
    case "$SHELL" in
        */zsh)
            echo "$HOME/.zshrc"
            ;;
        */bash)
            if [ -f "$HOME/.bashrc" ]; then
                echo "$HOME/.bashrc"
            else
                echo "$HOME/.bash_profile"
            fi
            ;;
        */fish)
            echo "$HOME/.config/fish/config.fish"
            ;;
        *)
            echo "$HOME/.profile"
            ;;
    esac
}

# ── write PATH export to shell config ────────────────────────────────────────
add_to_path() {
    config=$(resolve_shell_config)
    SHELL_CONFIG="$config"

    if grep -q "$INSTALL_DIR" "$config" 2>/dev/null; then
        info "PATH already configured in $config"
        return
    fi

    if echo "$SHELL" | grep -q fish; then
        printf '\n# cenv\nfish_add_path "%s"\n' "$INSTALL_DIR" >> "$config"
    else
        printf '\n# cenv\nexport PATH="$PATH:%s"\n' "$INSTALL_DIR" >> "$config"
    fi

    info "Added $INSTALL_DIR to PATH in $config"
}

# ── main ──────────────────────────────────────────────────────────────────────
main() {
    printf "\n${BOLD}cenv installer${NC}\n"
    printf "================\n"

    # ── system detection ──────────────────────────────────────────────────────
    step "Detecting system..."
    OS=$(detect_os)
    ARCH=$(detect_arch)
    DOWNLOADER=$(check_downloader)
    info "OS: $OS, Arch: $ARCH, Downloader: $DOWNLOADER"

    # ── resolve release info ──────────────────────────────────────────────────
    step "Fetching release info..."

    if [ -n "$CENV_VERSION" ]; then
        RELEASE_URL="https://api.github.com/repos/${REPO}/releases/tags/${CENV_VERSION}"
    else
        RELEASE_URL="https://api.github.com/repos/${REPO}/releases/latest"
    fi

    RELEASE_JSON=$(fetch "$RELEASE_URL" "$DOWNLOADER") \
        || error "Could not reach GitHub API. Check your internet connection."

    VERSION=$(echo "$RELEASE_JSON" | grep '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')
    [ -n "$VERSION" ] || error "Could not determine release version."
    info "Version: $VERSION"

    # ── find the right asset URL ──────────────────────────────────────────────
    step "Resolving download URL..."

    if [ "$OS" = "macOS" ]; then
        ASSET_URL=$(find_asset_url "$RELEASE_JSON" "macOS" "$ARCH")
        ASSET_TYPE="zip"
    elif [ "$OS" = "Linux" ]; then
        # Linux releases are .deb packages; arch keyword differs between deb naming
        if [ "$ARCH" = "x86_64" ]; then
            ASSET_URL=$(find_asset_url "$RELEASE_JSON" ".deb" "amd64")
        else
            ASSET_URL=$(find_asset_url "$RELEASE_JSON" ".deb" "arm64")
        fi
        ASSET_TYPE="deb"
    fi

    [ -n "$ASSET_URL" ] || error "No matching release asset found for $OS/$ARCH.\nAvailable assets: https://github.com/${REPO}/releases/tag/${VERSION}"
    info "$ASSET_URL"

    # ── download ──────────────────────────────────────────────────────────────
    step "Downloading..."
    TMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TMP_DIR"' EXIT

    ASSET_FILE="$TMP_DIR/cenv-download"
    download "$ASSET_URL" "$ASSET_FILE" "$DOWNLOADER" \
        || error "Download failed. Check the release page: https://github.com/${REPO}/releases"
    info "Download complete"

    # ── extract & install ─────────────────────────────────────────────────────
    step "Installing..."

    if [ "$ASSET_TYPE" = "zip" ]; then
        # macOS — extract zip, find the binary
        command -v unzip > /dev/null 2>&1 || error "'unzip' is required but not installed. Run: brew install unzip"
        unzip -q "$ASSET_FILE" -d "$TMP_DIR/extracted"
        BIN_PATH=$(find "$TMP_DIR/extracted" -type f -name "$BINARY" | head -1)
        [ -n "$BIN_PATH" ] || error "Binary '$BINARY' not found inside the archive."

        mkdir -p "$INSTALL_DIR"
        cp "$BIN_PATH" "$INSTALL_DIR/$BINARY"
        chmod +x "$INSTALL_DIR/$BINARY"
        info "Installed to $INSTALL_DIR/$BINARY"

    elif [ "$ASSET_TYPE" = "deb" ]; then
        # Linux — install the .deb package (requires sudo)
        DEB_FILE="$TMP_DIR/cenv.deb"
        mv "$ASSET_FILE" "$DEB_FILE"

        if command -v dpkg > /dev/null 2>&1; then
            sudo dpkg -i "$DEB_FILE" || sudo apt-get install -f -y
            info "Installed via dpkg"
            # dpkg installs to /usr/bin — adjust INSTALL_DIR so PATH check is correct
            INSTALL_DIR="/usr/bin"
        else
            error "dpkg not found. This installer uses .deb packages on Linux.\nFor manual installation, download the package from:\n  https://github.com/${REPO}/releases/tag/${VERSION}"
        fi
    fi

    # ── PATH setup ────────────────────────────────────────────────────────────
    step "Configuring PATH..."
    if echo "$PATH" | grep -q "$INSTALL_DIR"; then
        info "$INSTALL_DIR is already in PATH"
        SHELL_CONFIG=""
    else
        add_to_path
    fi

    # Apply to the current process so the verify step can find the binary
    export PATH="$PATH:$INSTALL_DIR"

    # ── verify ────────────────────────────────────────────────────────────────
    step "Verifying..."
    if "$INSTALL_DIR/$BINARY" --version > /dev/null 2>&1; then
        VERSION_STR=$("$INSTALL_DIR/$BINARY" --version)
        info "OK — $VERSION_STR"
    else
        error "Installed binary did not run. Please report this at https://github.com/${REPO}/issues"
    fi

    # ── done ──────────────────────────────────────────────────────────────────
    printf "\n${GREEN}${BOLD}Installation complete!${NC}\n\n"

    if [ -n "$SHELL_CONFIG" ]; then
        printf "${CYAN}To use cenv in this terminal without opening a new one, run:${NC}\n\n"
        printf "  ${BOLD}. %s${NC}\n\n" "$SHELL_CONFIG"
        printf "Future terminal sessions will have cenv available automatically.\n\n"
    else
        printf "Run ${BOLD}cenv --help${NC} to get started.\n\n"
    fi
}

main "$@"
