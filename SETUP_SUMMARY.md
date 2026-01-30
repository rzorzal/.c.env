# Installation & Release Setup - Summary

## ✅ What's Been Created

### 1. Installation Scripts

- **`install.sh`** - Universal installation script for Unix/Linux/macOS
  - Builds the release binary
  - Installs to `~/.local/bin/cenv`
  - Provides PATH setup instructions
  - Usage: `./install.sh` or `curl -sSf https://your-repo/install.sh | bash`

- **`scripts/create-deb.sh`** - Creates `.deb` packages for Debian/Ubuntu
  - Usage: `./scripts/create-deb.sh 0.1.0 amd64`

### 2. GitHub Actions Workflows

- **`.github/workflows/release.yml`** - Automated releases
  - Triggers on version tags (e.g., `v0.1.0`)
  - Builds binaries for:
    - Linux (x86_64, aarch64)
    - macOS (Intel, Apple Silicon)
    - Windows (x86_64)
  - Creates GitHub Release with all binaries
  - Uses `houseabsolute/actions-rust-release` as requested

- **`.github/workflows/build.yml`** - CI/CD
  - Runs tests on push/PR
  - Tests on Linux, macOS, Windows
  - Runs clippy and format checks

- **`.github/workflows/package.yml`** - Package creation
  - Creates `.deb` packages
  - Calculates SHA256 hashes for Homebrew

### 3. Package Manager Support

- **`homebrew/cenv.rb`** - Homebrew formula
  - Supports both macOS (Intel & ARM) and Linux (x86_64 & ARM64)
  - Needs SHA256 updates after each release

### 4. Documentation

- **`INSTALL.md`** - Complete installation guide for end users
- **`RELEASE_GUIDE.md`** - Step-by-step guide for creating releases
- **`SETUP_SUMMARY.md`** - This file

### 5. Updated Files

- **`Readme.md`** - Added installation section
- **`install.sh`** - Made executable
- **`scripts/create-deb.sh`** - Made executable

## 🚀 How to Create Your First Release

### 1. Update Version

```bash
# Update Cargo.toml
sed -i 's/version = "0.1.0"/version = "0.1.0"/' Cargo.toml
```

### 2. Commit and Tag

```bash
git add .
git commit -m "Release v0.1.0"
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

### 3. GitHub Actions Handles the Rest!

- Builds binaries for all platforms
- Creates GitHub Release
- Uploads all artifacts
- Generates release notes

## 📦 Installation Methods for Users

### 1. Quick Install Script (Easiest)

```bash
curl -sSf https://raw.githubusercontent.com/yourusername/c.env.lang/main/install.sh | bash
```

### 2. Homebrew

First, create a tap repository on GitHub named `homebrew-cenv`:

```bash
brew tap yourusername/cenv
brew install cenv
```

### 3. APT (Debian/Ubuntu)

```bash
wget https://github.com/yourusername/c.env.lang/releases/download/v0.1.0/cenv_0.1.0_amd64.deb
sudo dpkg -i cenv_0.1.0_amd64.deb
```

### 4. Pre-built Binaries

Download from GitHub Releases:

- `cenv-Linux-x86_64.tar.gz`
- `cenv-Darwin-aarch64.tar.gz` (Mac Apple Silicon)
- `cenv-Darwin-x86_64.tar.gz` (Mac Intel)
- `cenv-Windows-x86_64.zip`

### 5. Build from Source

```bash
git clone https://github.com/yourusername/c.env.lang.git
cd c.env.lang
./install.sh
```

## ⚙️ Configuration Needed

Before your first release, update these files with your information:

1. **All workflow files** (`.github/workflows/*.yml`):
   - URLs reference `yourusername` - replace with your GitHub username

2. **`homebrew/cenv.rb`**:
   - Update `homepage` URL
   - Update `url` with your repository name
   - SHA256 hashes updated automatically after first release

3. **`scripts/create-deb.sh`**:
   - Update `Maintainer` field
   - Update `Homepage` URL

4. **`INSTALL.md`**:
   - Replace all instances of `yourusername` with your GitHub username

5. **`Readme.md`**:
   - Installation URLs already reference `yourusername`

## 🧪 Testing the Installation

```bash
# Test the install script locally
./install.sh

# Verify installation
~/.local/bin/cenv --version

# Test with an example (once examples are fixed)
~/.local/bin/cenv examples/config.cenv --dry
```

## 📋 Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update version in `homebrew/cenv.rb` (optional, can be done after)
- [ ] Test build locally: `cargo build --release`
- [ ] Test installation: `./install.sh`
- [ ] Commit changes: `git commit -am "Release vX.Y.Z"`
- [ ] Create tag: `git tag vX.Y.Z`
- [ ] Push: `git push origin main && git push origin vX.Y.Z`
- [ ] Wait for GitHub Actions (check Actions tab)
- [ ] Verify release on GitHub Releases page
- [ ] Download and test binaries
- [ ] Update Homebrew formula with SHA256 hashes
- [ ] Test installation via each method

## 🏗️ Next Steps for Full Package Support

### For Homebrew Core (after project matures):

1. Build user base
2. Submit PR to `Homebrew/homebrew-core`
3. Maintain formula

### For APT Repository (advanced):

1. Set up server for hosting packages
2. Create repository structure
3. Sign packages with GPG
4. Document repository setup for users

### Other Package Managers to Consider:

- **Snap**: Cross-distro Linux package
- **Flatpak**: Another cross-distro option
- **Chocolatey**: Windows package manager
- **Scoop**: Alternative Windows package manager
- **cargo-binstall**: For Rust users

## 📊 Current Status

✅ **Ready to Use:**

- Installation script
- GitHub Actions for releases
- Multi-platform binary builds
- Homebrew tap support
- Debian package creation
- Documentation

⚠️ **Needs Configuration:**

- Replace `yourusername` with actual GitHub username
- Test first release workflow
- Update Homebrew formula after first release

🔮 **Future Enhancements:**

- Homebrew Core submission
- Self-hosted APT repository
- Additional package managers
- Automated Homebrew formula updates
- Automated version bumping

## 🆘 Troubleshooting

### Install script fails

- Check Rust is installed: `cargo --version`
- Check build succeeds: `cargo build --release`
- Verify `~/.local/bin` exists: `mkdir -p ~/.local/bin`

### GitHub Actions fails

- Check Actions tab for error logs
- Verify tag format is `vX.Y.Z`
- Ensure GITHUB_TOKEN has permissions

### Homebrew formula doesn't work

- Verify SHA256 hashes match releases
- Test formula locally: `brew install --build-from-source homebrew/cenv.rb`
- Check URLs are accessible

## 📚 Documentation Files

- **INSTALL.md**: User installation guide
- **RELEASE_GUIDE.md**: Maintainer release guide
- **SETUP_SUMMARY.md**: This file - quick reference

## ✨ Key Features

- **One-command installation**: `curl | bash` installer
- **Multi-platform support**: Linux, macOS (both architectures), Windows
- **Automated releases**: Tag and forget - GitHub Actions handles everything
- **Multiple installation methods**: Script, Homebrew, APT, direct download
- **Professional packaging**: Proper `.deb` packages with metadata
- **Comprehensive docs**: Installation and release guides

---

## Quick Command Reference

```bash
# Local installation
./install.sh

# Create release
git tag v0.1.0 && git push origin v0.1.0

# Create .deb package
./scripts/create-deb.sh 0.1.0 amd64

# Test installed binary
~/.local/bin/cenv --version

# Uninstall
rm ~/.local/bin/cenv
```
