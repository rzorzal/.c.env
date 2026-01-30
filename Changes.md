# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Nothing yet

### Changed

- Nothing yet

### Fixed

- Nothing yet

## [0.1.1] - 2026-01-30

### Added

- Initial release of C.env language interpreter
- Variable declarations with `private` keyword for scope control
- Public variables (without `private`) that appear in .env output
- Built-in functions: `print()`, `len()`, `type()`, `str()`, `num()`, `bool()`
- Import system with `import()` and `import_aws_secret()` functions
- Module-based configuration with `--module` flag
- Block statements with `{ }` for grouping statements
- Expression support:
  - Arithmetic operators: `+`, `-`, `*`, `/`, `%`
  - Comparison operators: `<`, `>`, `<=`, `>=`, `==`, `!=`
  - Logical operators: `&` (AND), `|` (OR)
  - String concatenation with `+`
  - Template strings with `${expression}` syntax
- CLI options:
  - `--module=<value>` - Set module variable for conditional imports
  - `--output=<file>` - Specify output filename
  - `--dry` - Output to stdout without creating file
  - `--debug` - Show tokens and AST for debugging
  - `--help` - Display help information
  - `--version` - Show version
- Installation script (`install.sh`) for easy setup
- Automatic PATH configuration in shell rc files
- GitHub Actions workflows:
  - Automated builds for Linux, macOS, Windows
  - Automated releases on version tags
  - Multi-architecture support (x86_64, aarch64)
  - .deb package creation for Debian/Ubuntu
- Homebrew formula template
- Comprehensive test suite (103 tests)
- Documentation:
  - Complete README with examples
  - Installation guide (INSTALL.md)
  - Release guide (RELEASE_GUIDE.md)
  - Quick reference guide

### Implementation Details

- Lexer with support for all token types
- Recursive descent parser
- Tree-walking interpreter/evaluator
- Environment-based variable scoping
- Import resolution with circular dependency detection
- Public/private variable separation for .env generation

### Fixed

- Parser correctly handles public variable declarations (without `private`)
- Block statements preserve parent scope as designed
- Binary naming consistency (`cenv` across all platforms)

---

## Release Instructions

### Creating a New Release

1. **Update this CHANGELOG.md**:
   - Move items from `[Unreleased]` to a new version section
   - Add the release date
   - Follow the format: `## [X.Y.Z] - YYYY-MM-DD`

2. **Update Cargo.toml**:

   ```bash
   # Update version field
   version = "X.Y.Z"
   ```

3. **Commit and Tag**:

   ```bash
   git add CHANGELOG.md Cargo.toml
   git commit -m "Release vX.Y.Z"
   git tag vX.Y.Z
   git push origin main
   git push origin vX.Y.Z
   ```

4. **GitHub Actions** will automatically:
   - Build binaries for all platforms
   - Create GitHub Release
   - Upload artifacts
   - Generate release notes from this changelog

### Version Numbering

- **MAJOR** (X.0.0): Breaking changes
- **MINOR** (0.X.0): New features, backwards compatible
- **PATCH** (0.0.X): Bug fixes, backwards compatible

### Categories for Changelog Entries

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security fixes

---

[unreleased]: https://github.com/rzorzal/c.env.lang/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/rzorzal/c.env.lang/releases/tag/v0.1.1
