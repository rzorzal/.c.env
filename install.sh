#!/bin/sh
# cenv installer
# Downloads the pre-built binary from GitHub Releases and installs it.
# Usage: curl -sSf https://raw.githubusercontent.com/rzorzal/.c.env/main/install.sh | sh

set -e

REPO="rzorzal/.c.env"
BINARY="cenv"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Tracks which config file was modified so we can tell the user what to source
SHELL_CONFIG=""

# ── colors ───────────────────────────────────────────────────────────────────
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
        Darwin*) echo "Darwin" ;;
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

# ── fetch latest release tag from GitHub API ─────────────────────────────────
latest_version() {
    downloader="$1"
    api_url="https://api.github.com/repos/${REPO}/releases/latest"

    if [ "$downloader" = "curl" ]; then
        curl -fsSL "$api_url" \
            | grep '"tag_name"' \
            | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/'
    else
        wget -qO- "$api_url" \
            | grep '"tag_name"' \
            | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/'
    fi
}

# ── resolve the user's shell config file ─────────────────────────────────────
# Uses $SHELL (inherited from the parent terminal, even inside curl | sh)
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
    SHELL_CONFIG="$config"   # store for use in the final message

    # Already present — nothing to do
    if grep -q "$INSTALL_DIR" "$config" 2>/dev/null; then
        info "PATH already configured in $config"
        return
    fi

    # fish uses a different command
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

    # ── system detection ──
    step "Detecting system..."
    OS=$(detect_os)
    ARCH=$(detect_arch)
    DOWNLOADER=$(check_downloader)
    info "OS: $OS, Arch: $ARCH, Downloader: $DOWNLOADER"

    # ── version resolution ──
    step "Resolving version..."
    if [ -n "$CENV_VERSION" ]; then
        VERSION="$CENV_VERSION"
        info "Using requested version: $VERSION"
    else
        VERSION=$(latest_version "$DOWNLOADER")
        [ -n "$VERSION" ] || error "Could not fetch latest version from GitHub.\nSet CENV_VERSION=vX.Y.Z to install a specific version."
        info "Latest release: $VERSION"
    fi

    # ── download ──
    ASSET="${BINARY}-${OS}-${ARCH}.tar.gz"
    URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}"

    step "Downloading..."
    info "$URL"

    TMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TMP_DIR"' EXIT

    download "$URL" "$TMP_DIR/$ASSET" "$DOWNLOADER" \
        || error "Download failed.\nCheck that the release exists: https://github.com/${REPO}/releases"

    # ── extract & install ──
    step "Installing..."
    tar -xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR" \
        || error "Failed to extract $ASSET"

    BIN_PATH=$(find "$TMP_DIR" -type f -name "$BINARY" | head -1)
    [ -n "$BIN_PATH" ] || error "Binary '$BINARY' not found in archive"

    mkdir -p "$INSTALL_DIR"
    cp "$BIN_PATH" "$INSTALL_DIR/$BINARY"
    chmod +x "$INSTALL_DIR/$BINARY"
    info "Installed to $INSTALL_DIR/$BINARY"

    # ── PATH setup ──
    step "Configuring PATH..."
    if echo "$PATH" | grep -q "$INSTALL_DIR"; then
        info "$INSTALL_DIR is already in PATH — nothing to do"
        SHELL_CONFIG=""
    else
        add_to_path
    fi

    # Apply to the current process so the verify step below can find the binary
    export PATH="$PATH:$INSTALL_DIR"

    # ── verify ──
    step "Verifying..."
    if "$INSTALL_DIR/$BINARY" --version > /dev/null 2>&1; then
        VERSION_STR=$("$INSTALL_DIR/$BINARY" --version 2>/dev/null)
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
