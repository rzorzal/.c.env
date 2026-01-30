# C.env Language

A `.env` file compiler that generates environment configuration files from `.cenv` source files with JavaScript-like syntax, module-based imports, and strong typing.

## 🎯 Purpose

C.env compiles `.cenv` source files into `.env` files for different environments (production, staging, development, etc.) using a `--module` argument to control which configuration to load.

## � Installation

### Quick Install (Recommended)

```bash
curl -sSf https://raw.githubusercontent.com/rzorzal/.c.env/main/install.sh | bash
```

### Package Managers

```bash
# Homebrew (macOS/Linux)
brew install rzorzal/cenv

# From source with Cargo
cargo install --git https://github.com/rzorzal/.c.env
```

**[Complete Installation Guide →](INSTALL.md)** - More installation options including APT, pre-built binaries, and building from source.

## 🚀 Quick Start

```bash
# Compile and generate .env file (with production module variable)
cenv examples/config.cenv --module=production

# Compile and generate .env file (with staging module variable)
cenv examples/config.cenv --module=staging

# Custom output filename
cenv examples/config.cenv --module=production --output=.env.production

# Dry run - output to stdout without creating file
cenv examples/config.cenv --module=production --dry

# Get help
cenv --help

# Check version
cenv --version
```

## 📚 Documentation

**[Complete Documentation →](docs/README.md)**

- **[Getting Started](docs/getting-started/README.md)** - Installation and first steps
- **[Language Reference](docs/language-reference/README.md)** - Complete syntax and features
- **[Quick Reference](docs/QUICK_REFERENCE.md)** - Commands & syntax quick reference
- **[Examples](examples/README.md)** - Code examples and working demos
- **[Implementation Summary](docs/PUBLIC_PRIVATE_VARS_SUMMARY.md)** - Public/private variables details

## ✨ Features

- ✅ **Public/Private Variables** - Control which variables appear in the .env output ⭐ NEW
- ✅ **Module Variable** - Special `module` variable set via `--module=value` argument
- ✅ **Dynamic Imports** - Import files based on module: `import('./.cenv.' + module)`
- ✅ **Data Types** - Numbers, strings, booleans, null
- ✅ **Template Strings** - String interpolation with `"text ${variable} text"`
- ✅ **Operators** - Full set with correct precedence (including string concatenation)
- ✅ **Comments** - Single-line (`//`) and multi-line (`/* */`)
- ✅ **Error Handling** - Clear, helpful error messages

### Public vs Private Variables

**Public variables** (no `private` keyword) are exported to the `.env` file:

```cenv
API_URL = "https://api.example.com"  // ✅ Exported to .env
PORT = 8080                          // ✅ Exported to .env
```

**Private variables** (with `private` keyword) are for internal calculations only:

```cenv
private max_pool = 20      // ❌ Not exported to .env
private min_pool = 5       // ❌ Not exported to .env
POOL_SIZE = max_pool       // ✅ Exported to .env (value: 20)
```

### Import System

Import functions return **objects** containing all **public variables** from the imported file:

```cenv
// database.cenv
DATABASE_URL = "postgresql://localhost/db"
DATABASE_PORT = 5432
private connection_timeout = 30  // Not included in import object
```

**Using imports**:

```cenv
// Import returns an object with public variables
private db_config = import("database.cenv")

// Access variables using dot notation
DATABASE_URL = db_config.DATABASE_URL
DATABASE_PORT = db_config.DATABASE_PORT

// Can use in expressions
print("Connecting to:", db_config.DATABASE_URL)
```

**AWS Secrets Manager** (placeholder - returns empty object currently):

```cenv
private secrets = import_aws_secret("my-app/credentials")
// In production: secrets.API_KEY, secrets.SECRET_TOKEN, etc.
```

**Key points**:

- ✅ `import()` and `import_aws_secret()` return objects
- ✅ Only **public variables** are included in the object
- ✅ Access fields using dot notation: `obj.field`
- ✅ Backward compatible: standalone `import("file")` still merges variables

### Output Modes

**File output (default)**:

- Creates `.env` file by default
- Use `--module=<name>` → creates `.env.<name>`
- Use `--output=<filename>` → creates custom filename
- **Priority**: `--output` > `--module` > `.env`

**Dry run** (`--dry` flag):

- No file created - all output to stdout
- Shows both print statements and .env variables
- Useful for testing and debugging

**Examples**:

```bash
cenv config.cenv                           # → .env
cenv config.cenv --module=production       # → .env.production
cenv config.cenv --output=.env.custom      # → .env.custom
cenv config.cenv --module=prod --output=.env.p  # → .env.p (output wins)
cenv config.cenv --dry                     # → stdout only
```

- ✅ **Built-in Functions** - print(), type(), len(), num(), str(), bool()
- ✅ **Import Statements** - Load and share code across files with expression support
- ✅ **Runtime Evaluation** - Execute programs with proper error handling

## 📖 Example

### Module-Based .env Compilation

C.env allows you to maintain different environment configurations and compile them to `.env` files.

#### Main Configuration: `examples/config.cenv`

```javascript
// Import environment-specific config based on module
import("examples/.cenv." + module)

// Private variables for internal calculations
private max_pool_size = 20
private min_pool_size = 5

// Public variables exported to .env
APP_NAME = "MyApplication"
APP_VERSION = "1.0.0"
PORT = 3000

// Calculated public variable using private variable
DATABASE_POOL_SIZE = max_pool_size

// Using imported variables
ENVIRONMENT = module

print("Compiling .env for environment:", module)
print("API URL:", API_URL)
```

#### Environment-Specific Files

```javascript
// examples/.cenv.production
API_URL = "https://prod.api.example.com"
DATABASE_URL = "postgresql://prod-db.example.com:5432/myapp"
DEBUG_MODE = false
LOG_LEVEL = "error"

private internal_cache_ttl = 3600
```

```javascript
// examples/.cenv.staging
API_URL = "https://staging.api.example.com"
DATABASE_URL = "postgresql://staging-db.example.com:5432/myapp"
DEBUG_MODE = true
LOG_LEVEL = "debug"

private internal_cache_ttl = 60
```

#### Compilation

```bash
# Compile for production (creates .env.production)
./target/release/cenv examples/config.cenv --module=production

# Output:
# ✓ Generated .env.production
# Compiling .env for environment: production
# API URL: https://prod.api.example.com

# Output (.env.production file content):
# API_URL=https://prod.api.example.com
# APP_NAME=MyApplication
# APP_VERSION=1.0.0
# DATABASE_POOL_SIZE=20
# DATABASE_URL=postgresql://prod-db.example.com:5432/myapp
# DEBUG_MODE=false
# ENVIRONMENT=production
# LOG_LEVEL=error
# PORT=3000

# Dry run mode - output to stdout without creating file
cargo run --quiet -- examples/config.cenv --module=production --dry
```

// Built-in functions
print("Type of port:", type(port))
print("Port as string:", str(port))

// Calculations
private maxUsers = 100
private bufferSize = maxUsers \* 1024
print("Buffer size:", bufferSize)

// Type conversion
private userInput = "42"
private value = num(userInput)
print("Converted value:", value)

// Import from other files
import("config.cenv")
print("Loaded config:", api_url)

````

### Module-Based Import Example

```javascript
// config.cenv - Main configuration file
import("./.cenv." + module)  // Dynamically import based on --module argument
print("Loaded config for:", module)
````

```javascript
// .cenv.production
private api_url = "https://api.example.com"
private timeout = 30
private max_retries = 3
```

```bash
# Compile with module
./target/release/cenv config.cenv --module=production
# Output: Loaded config for: production
```

## 🏗️ Project Status

**Phase 1 Complete!** ✅ **Phase 2.1 Complete!** ✅ **Phase 2.2 Complete!** ✅ **Phase 2.3 Complete!** ✅

- ✅ Error infrastructure with helpful messages
- ✅ Lexer with comments and multi-char operators
- ✅ Parser with correct operator precedence (6 levels)
- ✅ Assignment statements for variable mutation
- ✅ Runtime evaluator with environment management
- ✅ Built-in functions (print, type, len, num, str, bool)
- ✅ Import statements for code organization
- ✅ Comprehensive test suite (82 tests passing)
- ✅ Modular test organization (separate files per feature)

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for the full roadmap.

## 🚀 Running Programs

Execute a C.env program:

```bash
./target/release/cenv program.cenv
```

Example output:

```bash
$ ./target/release/cenv examples/phase2_2_demo.cenv
Hello, World!
42
true
Name: Alice
Age: 30
Total: 30
```

Debug mode (shows parsing and AST):

```bash
./target/release/cenv program.cenv --debug
```

## 🧪 Running Tests

```bash
cargo test --lib
```

Expected output:

```
running 68 tests
test result: ok. 68 passed; 0 failed
```

## 📝 Quick Reference

### Variable Declaration

```javascript
private variableName = value
```

### Assignment (Reassignment)

```javascript
variableName = newValue;
```

Example:

```javascript
private counter = 0
counter = 10
counter = counter + 1  // counter is now 11
```

### Operators (by precedence)

| Precedence | Operators         | Example  |
| ---------- | ----------------- | -------- |
| Highest    | `*` `/` `%`       | `2 * 3`  |
| ↓          | `+` `-`           | `5 + 3`  |
| ↓          | `<` `>` `<=` `>=` | `x < 10` |
| ↓          | `==` `!=`         | `a == b` |
| ↓          | `&`               | `a & b`  |
| Lowest     | `\|`              | `a \| b` |

### Comments

```javascript
// Single-line comment
/* Multi-line comment */
```

## 🛠️ Building from Source

### Requirements

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build Steps

```bash
# Clone the repository
git clone <repository-url>
cd c.env.lang

# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Install to system (optional)
cargo install --path .
```

## 📂 Project Structure

```
c.env.lang/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library exports
│   ├── lexing/          # Lexer implementation
│   └── grama/           # Parser implementation
├── docs/                # Documentation
├── examples/            # Example .c.env files
└── IMPLEMENTATION_PLAN.md
```

## 🤝 Contributing

Contributions are welcome! See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for areas that need work.

## 📚 Resources

- [Lexical Analysis Reference](https://krypticmouse.hashnode.dev/writing-a-compiler-lexical-analysis)
- [Sox Language Reference](https://github.com/obiesie/sox/tree/main/src)

## 📄 License

[Add license information here]

---

**Need Help?** Check the [FAQ](docs/faq.md) or [Getting Started Guide](docs/getting-started/README.md)
