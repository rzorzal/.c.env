# Installation Guide

## System Requirements

- **Operating System:** macOS, Linux, or Windows
- **Rust:** 1.70 or later (for building from source)
- **Memory:** 100MB minimum
- **Disk Space:** 50MB for the compiler

## Installation Methods

### Method 1: Build from Source (Recommended)

1. **Install Rust** (if not already installed):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:

   ```bash
   git clone https://github.com/rzorzal/.c.env.git
   cd c.env.lang
   ```

3. **Build the project**:

   ```bash
   cargo build --release
   ```

4. **Install the binary** (optional):

   ```bash
   cargo install --path .
   ```

   Or copy the binary to your PATH:

   ```bash
   cp target/release/c_env_lang /usr/local/bin/
   ```

5. **Verify installation**:

   ```bash
   c_env_lang --version
   ```

### Method 2: Use Development Build

For development and testing:

```bash
cd c.env.lang
cargo build
./target/debug/c_env_lang <your-file.c.env>
```

## Directory Structure

After building, your directory should look like:

```
c.env.lang/
├── src/           # Source code
├── examples/      # Example .c.env files
├── docs/          # Documentation
├── target/        # Build artifacts
│   ├── debug/     # Debug builds
│   └── release/   # Optimized builds
└── Cargo.toml     # Project configuration
```

## Troubleshooting

### "cargo: command not found"

Make sure Rust is properly installed and in your PATH:

```bash
source $HOME/.cargo/env
```

### Build Errors

Make sure you have the latest Rust version:

```bash
rustup update
```

### Permission Denied

On Unix systems, you may need to make the binary executable:

```bash
chmod +x target/release/c_env_lang
```

## Next Steps

Once installed, proceed to the [Quick Start Guide](quickstart.md) to write your first C.env program.
