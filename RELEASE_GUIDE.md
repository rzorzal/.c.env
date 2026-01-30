# Release and Distribution Setup Guide

## Overview

This guide explains how to set up releases and distribution for the C.env language interpreter.

## 📋 What's Been Created

### 1. Installation Scripts

- **`install.sh`** - Universal installer for Unix/Linux/macOS
- **`scripts/create-deb.sh`** - Debian package creator

### 2. GitHub Actions Workflows

- **`.github/workflows/release.yml`** - Automatic binary releases for all platforms
- **`.github/workflows/build.yml`** - CI/CD for testing and building
- **`.github/workflows/package.yml`** - Package creation for APT and Homebrew

### 3. Package Manager Support

- **`homebrew/cenv.rb`** - Homebrew formula template
- **Debian packages** - Via create-deb.sh script

### 4. Documentation

- **`INSTALL.md`** - Complete installation guide for users

## 🚀 How to Create Your First Release

### Step 1: Update Version in Cargo.toml

```toml
[package]
name = "c_env_lang"
version = "0.1.0"  # Update this
```

### Step 2: Commit and Tag

```bash
git add .
git commit -m "Release v0.1.0"
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

### Step 3: GitHub Actions Takes Over

Once you push the tag, GitHub Actions will:

1. Build binaries for Linux (x86_64, aarch64)
2. Build binaries for macOS (Intel, Apple Silicon)
3. Build binaries for Windows
4. Create a GitHub Release with all binaries attached
5. Generate release notes automatically

### Step 4: Create .deb Packages (Optional)

After the release is published, the package workflow will create `.deb` files for Debian/Ubuntu.

## 📦 Package Manager Setup

### Homebrew

#### Option 1: Homebrew Tap (Recommended for your own project)

1. Create a new repository named `homebrew-cenv`:

   ```bash
   # On GitHub, create: yourusername/homebrew-cenv
   ```

2. Copy `homebrew/cenv.rb` to the tap repository as `Formula/cenv.rb`

3. After each release, update the formula:
   - Update version number
   - Update SHA256 hashes (the package.yml workflow prints these)
   - Update URLs to point to your release

4. Users install with:
   ```bash
   brew tap yourusername/cenv
   brew install cenv
   ```

#### Option 2: Submit to Homebrew Core (For popular tools)

Once your tool gains traction, you can submit to [Homebrew/homebrew-core](https://github.com/Homebrew/homebrew-core).

### APT (Debian/Ubuntu)

#### Option 1: Direct .deb Installation (Easiest)

Users download `.deb` from releases:

```bash
wget https://github.com/yourusername/c.env.lang/releases/download/v0.1.0/cenv_0.1.0_amd64.deb
sudo dpkg -i cenv_0.1.0_amd64.deb
```

#### Option 2: PPA (Personal Package Archive) - Ubuntu Only

1. Create a Launchpad account
2. Create a PPA: https://launchpad.net/~yourusername/+activate-ppa
3. Upload packages using `dput`

#### Option 3: Self-Hosted APT Repository (Advanced)

1. Set up a server with apt repository structure
2. Host `.deb` files
3. Users add your repository:
   ```bash
   echo "deb https://apt.yourdomain.com stable main" | sudo tee /etc/apt/sources.list.d/cenv.list
   curl -fsSL https://apt.yourdomain.com/key.gpg | sudo apt-key add -
   sudo apt update
   sudo apt install cenv
   ```

## 🔧 Configuration Required

### Before Your First Release:

1. **Update repository URLs** in all files:
   - Replace `yourusername` with your GitHub username
   - Replace `your.email@example.com` with your email

   Files to update:
   - `homebrew/cenv.rb`
   - `scripts/create-deb.sh`
   - `INSTALL.md`
   - All workflow files

2. **Update the binary name** (if you want it different from `cenv`):
   - In `install.sh`: Change `cenv` to your preferred name
   - In workflows: Update `executable-name`

3. **Test locally**:

   ```bash
   # Test the installer
   ./install.sh

   # Test the binary
   cenv --version
   ```

## 📝 Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update version in `homebrew/cenv.rb`
- [ ] Update CHANGELOG.md (create if needed)
- [ ] Test build locally: `cargo build --release`
- [ ] Test installation: `./install.sh`
- [ ] Commit changes
- [ ] Create and push tag
- [ ] Wait for GitHub Actions to complete
- [ ] Verify release artifacts on GitHub
- [ ] Update Homebrew formula with new SHA256 hashes
- [ ] Test installation from released binaries

## 🧪 Testing Before Release

```bash
# Build and test locally
cargo build --release
./target/release/c_env_lang examples/hello.cenv

# Test the installer
./install.sh

# Verify it's in PATH
which cenv
cenv --version

# Test on a clean system (Docker)
docker run -it --rm -v $(pwd):/app rust:latest bash
cd /app
./install.sh
cenv --version
```

## 🔄 Update Process

When you want to release a new version:

```bash
# 1. Update version
sed -i 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml

# 2. Commit and tag
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git tag v0.2.0

# 3. Push
git push origin main
git push origin v0.2.0

# GitHub Actions handles the rest!
```

## 📊 Distribution Channels Summary

| Channel           | Setup Difficulty | User Base        | Maintenance        |
| ----------------- | ---------------- | ---------------- | ------------------ |
| GitHub Releases   | ⭐ Easy          | Everyone         | Automatic          |
| Direct install.sh | ⭐ Easy          | Unix/Linux/macOS | Minimal            |
| Homebrew Tap      | ⭐⭐ Medium      | macOS/Linux      | Manual updates     |
| Homebrew Core     | ⭐⭐⭐⭐ Hard    | macOS (large)    | PR-based           |
| Direct .deb       | ⭐⭐ Medium      | Debian/Ubuntu    | Automatic          |
| PPA               | ⭐⭐⭐ Medium    | Ubuntu           | Upload per release |
| Self-hosted APT   | ⭐⭐⭐⭐ Hard    | Debian/Ubuntu    | Server maintenance |
| Cargo             | ⭐ Easy          | Rust developers  | Automatic          |

## 🎯 Recommended Setup for Starting Out

1. **Immediate** (already done):
   - ✅ GitHub Releases workflow
   - ✅ install.sh script
   - ✅ Build workflow for CI

2. **First Week**:
   - Create Homebrew tap
   - Test .deb package creation
   - Update all URLs/emails in files

3. **After Some Adoption**:
   - Submit to Homebrew Core
   - Set up PPA for Ubuntu users
   - Consider other package managers (Snap, Flatpak, etc.)

## 🆘 Troubleshooting

### Release workflow fails

- Check GitHub Actions logs
- Verify Rust toolchain is compatible
- Ensure all targets are available

### Homebrew formula doesn't work

- Verify SHA256 hashes match
- Check URLs are accessible
- Test formula locally: `brew install --build-from-source homebrew/cenv.rb`

### .deb package won't install

- Check dependencies in control file
- Verify architecture matches
- Test with: `dpkg -I cenv_*.deb`

## 📚 Additional Resources

- [GitHub Actions for Rust](https://github.com/actions-rs)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Debian Packaging Guide](https://www.debian.org/doc/manuals/debmake-doc/)
- [actions-rust-release](https://github.com/houseabsolute/actions-rust-release)
