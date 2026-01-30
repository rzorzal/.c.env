# Pre-Release Checklist

## Before Your First Release

### 1. Update Repository Information

Replace `yourusername` with your GitHub username in these files:

- [ ] `.github/workflows/release.yml`
- [ ] `.github/workflows/package.yml`
- [ ] `homebrew/cenv.rb`
- [ ] `scripts/create-deb.sh`
- [ ] `INSTALL.md`
- [ ] `Readme.md`

### 2. Update Maintainer Information

Replace email and name in:

- [ ] `scripts/create-deb.sh` (Maintainer field)
- [ ] `homebrew/cenv.rb` (if needed)

### 3. Test Locally

```bash
# Test build
cargo build --release

# Test binary works
./target/release/c_env_lang --help

# Test installation script
./install.sh

# Verify installed binary
~/.local/bin/cenv --version
```

- [ ] Build succeeds
- [ ] Binary runs
- [ ] Install script works
- [ ] Installed binary works

### 4. Prepare for Homebrew

If you want Homebrew support:

- [ ] Create GitHub repository: `homebrew-cenv`
- [ ] Add README explaining it's a Homebrew tap
- [ ] Note: Formula will be updated after first release with SHA256 hashes

### 5. First Release

```bash
# Make sure everything is committed
git status

# Tag the release
git tag v0.1.0

# Push everything
git push origin main
git push origin v0.1.0
```

- [ ] All changes committed
- [ ] Tag created
- [ ] Tag pushed

### 6. Verify Release

After pushing the tag:

- [ ] Check GitHub Actions tab - all workflows should be green
- [ ] Check Releases page - release should be created
- [ ] Download each binary and test:
  - [ ] Linux x86_64
  - [ ] Linux aarch64
  - [ ] macOS x86_64
  - [ ] macOS aarch64
  - [ ] Windows x86_64
- [ ] Verify `.deb` packages are attached

### 7. Update Homebrew Formula

After first release succeeds:

1. Get SHA256 hashes from package workflow logs or:

```bash
wget <release-url>/cenv-Darwin-aarch64.tar.gz
sha256sum cenv-Darwin-aarch64.tar.gz
```

1. Update `homebrew/cenv.rb`:
   - [ ] Update SHA256 for Darwin aarch64
   - [ ] Update SHA256 for Darwin x86_64
   - [ ] Update SHA256 for Linux aarch64
   - [ ] Update SHA256 for Linux x86_64

2. If using a tap:
   - [ ] Copy formula to `homebrew-cenv` repo
   - [ ] Commit and push to tap repo

### 8. Test Installation Methods

- [ ] Install via script: `curl -sSf <url>/install.sh | bash`
- [ ] Install via Homebrew (if set up): `brew install yourusername/cenv/cenv`
- [ ] Install via .deb: `sudo dpkg -i cenv_*.deb`
- [ ] Install from source: `./install.sh`

### 9. Documentation

- [ ] README.md has correct installation instructions
- [ ] INSTALL.md is complete
- [ ] Examples work (fixdemo if needed)
- [ ] Links in documentation point to correct repository

## For Each Subsequent Release

### 1. Prepare Release

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md` (create if doesn't exist)
- [ ] Test all changes locally
- [ ] Run full test suite: `cargo test`

### 2. Create Release

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to vX.Y.Z"
git tag vX.Y.Z
git push origin main
git push origin vX.Y.Z
```

- [ ] Version updated
- [ ] Changes committed
- [ ] Tag created and pushed

### 3. Verify and Update

- [ ] GitHub Actions completed successfully
- [ ] All binaries available in release
- [ ] Download and test binaries
- [ ] Update Homebrew formula with new SHA256 hashes
- [ ] Test Homebrew installation

### 4. Announce

- [ ] Update README with new features
- [ ] Post release notes
- [ ] Update documentation if APIs changed

## Maintenance

### Monthly

- [ ] Check for security updates in dependencies
- [ ] Review and merge dependabot PRs
- [ ] Check GitHub Actions are still working

### When Issues Arise

- [ ] Check GitHub Actions logs
- [ ] Test locally first
- [ ] Update workflows if GitHub Actions APIs change

## Notes

- **Version format**: Use semantic versioning (vX.Y.Z)
- **GitHub Actions**: Automatically triggered on tag push
- **Homebrew**: Requires manual SHA256 update after each release
- **Testing**: Always test locally before tagging
- **Documentation**: Keep INSTALL.md and README.md in sync

## Quick Commands

```bash
# Create and push release tag
git tag v0.1.0 && git push origin v0.1.0

# Calculate SHA256 for Homebrew
sha256sum <file>.tar.gz

# Test local install
./install.sh

# Clean install for testing
rm ~/.local/bin/cenv && ./install.sh

# Check GitHub Actions status
# Visit: https://github.com/yourusername/c.env.lang/actions
```
