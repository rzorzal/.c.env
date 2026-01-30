# Installation Guide

## Quick Install (Unix/Linux/macOS)

```bash
curl -sSf https://raw.githubusercontent.com/rzorzal/.c.env/main/install.sh | bash
```

Or download and run manually:

```bash
git clone https://github.com/rzorzal/.c.env.git
cd c.env.lang
chmod +x install.sh
./install.sh
```

## Package Managers

### Homebrew (macOS and Linux)

```bash
# Add the tap (once)
brew tap rzorzal/cenv https://github.com/rzorzal/.c.env

# Install
brew install cenv

# Update
brew upgrade cenv
```

### APT (Debian/Ubuntu)

Download the `.deb` package from the [releases page](https://github.com/rzorzal/.c.env/releases):

```bash
# Download the latest .deb
wget https://github.com/rzorzal/.c.env/releases/download/v0.1.1/cenv_0.1.1_amd64.deb

# Install
sudo dpkg -i cenv_0.1.1_amd64.deb

# If dependencies are missing
sudo apt-get install -f
```

### Cargo (Rust Package Manager)

```bash
cargo install --git https://github.com/rzorzal/.c.env
```

## Pre-built Binaries

Download pre-built binaries from the [releases page](https://github.com/rzorzal/.c.env/releases):

### Linux (x86_64)

```bash
wget https://github.com/rzorzal/.c.env/releases/download/v0.1.1/cenv-Linux-x86_64.tar.gz
tar xzf cenv-Linux-x86_64.tar.gz
sudo mv cenv /usr/local/bin/
```

### macOS (Apple Silicon)

```bash
wget https://github.com/rzorzal/.c.env/releases/download/v0.1.1/cenv-Darwin-aarch64.tar.gz
tar xzf cenv-Darwin-aarch64.tar.gz
sudo mv cenv /usr/local/bin/
```

### macOS (Intel)

```bash
wget https://github.com/rzorzal/.c.env/releases/download/v0.1.1/cenv-Darwin-x86_64.tar.gz
tar xzf cenv-Darwin-x86_64.tar.gz
sudo mv cenv /usr/local/bin/
```

### Windows

Download `cenv-Windows-x86_64.zip` from releases, extract, and add to PATH.

## Build from Source

### Requirements

- Rust 1.70 or later
- Cargo

### Steps

```bash
# Clone the repository
git clone https://github.com/rzorzal/.c.env.git
cd c.env.lang

# Build
cargo build --release

# The binary will be at target/release/cenv
# Copy it to your PATH as 'cenv'
sudo cp target/release/cenv /usr/local/bin/cenv
```

## Verify Installation

```bash
cenv --version
```

## Usage

```bash
# Run a C.env file
cenv myconfig.cenv

# Run with verbose output
cenv -v myconfig.cenv

# Get help
cenv --help
```

## Uninstall

### If installed with install.sh

```bash
rm ~/.local/bin/cenv
```

### If installed with Homebrew

```bash
brew uninstall cenv
```

### If installed with APT

```bash
sudo dpkg -r cenv
```

### If installed manually

```bash
sudo rm /usr/local/bin/cenv
```

## Updating

### Homebrew

```bash
brew upgrade cenv
```

### APT

Download and install the new `.deb` package.

### Manual/install.sh

Run the installation process again:

```bash
curl -sSf https://raw.githubusercontent.com/rzorzal/.c.env/main/install.sh | bash
```
