# cenv

A `.env` file compiler. Write your environment configuration once in a `.cenv` source file and compile it into `.env` files for any environment — production, staging, development, and beyond.

```bash
cenv config.cenv --module=production   # generates .env.production
cenv config.cenv --module=staging      # generates .env.staging
```

---

## Table of Contents

- [Why cenv?](#why-cenv)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
  - [Public vs Private Variables](#public-vs-private-variables)
  - [The `module` Variable](#the-module-variable)
  - [Imports](#imports)
  - [Comments](#comments)
- [Language Reference](#language-reference)
  - [Data Types](#data-types)
  - [Variables](#variables)
  - [Template Strings](#template-strings)
  - [Operators](#operators)
  - [Built-in Functions](#built-in-functions)
  - [Blocks](#blocks)
- [CLI Reference](#cli-reference)
- [Multi-Environment Pattern](#multi-environment-pattern)
- [Building from Source](#building-from-source)
- [Contributing](#contributing)

---

## Why cenv?

Managing environment configuration across multiple deployments is painful. The common approaches all have trade-offs:

| Approach | Problem |
|---|---|
| Multiple `.env` files | Copy-paste duplication, easy to miss a change |
| Shell scripts | Hard to read, hard to maintain, not typed |
| CI/CD env vars only | No local development workflow |

**cenv** lets you write configuration logic once and compile it into separate `.env` files per environment. You get variables, imports, expressions, and a clean separation between what goes into the `.env` file and what is just for internal calculation.

---

## Installation

### Quick Install (Unix/Linux/macOS)

```bash
curl -sSf https://raw.githubusercontent.com/rzorzal/.c.env/main/install.sh | bash
```

### Homebrew (macOS and Linux)

```bash
brew tap rzorzal/cenv https://github.com/rzorzal/.c.env
brew install cenv
```

### Cargo (Rust Package Manager)

```bash
cargo install --git https://github.com/rzorzal/.c.env
```

### Pre-built Binaries

Download the binary for your platform from the [releases page](https://github.com/rzorzal/.c.env/releases):

**Linux (x86_64)**
```bash
wget https://github.com/rzorzal/.c.env/releases/latest/download/cenv-Linux-x86_64.tar.gz
tar xzf cenv-Linux-x86_64.tar.gz
sudo mv cenv /usr/local/bin/
```

**macOS (Apple Silicon)**
```bash
wget https://github.com/rzorzal/.c.env/releases/latest/download/cenv-Darwin-aarch64.tar.gz
tar xzf cenv-Darwin-aarch64.tar.gz
sudo mv cenv /usr/local/bin/
```

**macOS (Intel)**
```bash
wget https://github.com/rzorzal/.c.env/releases/latest/download/cenv-Darwin-x86_64.tar.gz
tar xzf cenv-Darwin-x86_64.tar.gz
sudo mv cenv /usr/local/bin/
```

**Windows**  
Download `cenv-Windows-x86_64.zip` from the [releases page](https://github.com/rzorzal/.c.env/releases), extract it, and add the folder to your `PATH`.

### APT (Debian/Ubuntu)

```bash
wget https://github.com/rzorzal/.c.env/releases/latest/download/cenv_amd64.deb
sudo dpkg -i cenv_amd64.deb
```

### Verify Installation

```bash
cenv --version
```

---

## Quick Start

**1. Create a config file**

```cenv
# config.cenv

# Private variables — used for calculations, never exported to .env
private base_url = "example.com"

# Public variables — exported to .env
APP_NAME = "MyApp"
API_URL = "https://api." + base_url
PORT = 3000
DEBUG = false
```

**2. Compile it**

```bash
cenv config.cenv
```

**3. Check the output (`.env` file)**

```env
APP_NAME=MyApp
API_URL=https://api.example.com
PORT=3000
DEBUG=false
```

That's it. Read [Multi-Environment Pattern](#multi-environment-pattern) to see the full power of `--module`.

---

## Core Concepts

### Public vs Private Variables

The key distinction in cenv is whether a variable ends up in the generated `.env` file.

**Public variables** — declared without any keyword — are exported to `.env`:

```cenv
API_URL = "https://api.example.com"    // exported
PORT = 8080                            // exported
DEBUG = false                          // exported
```

**Private variables** — declared with the `private` keyword — are only available during compilation. They never appear in the output:

```cenv
private pool_min = 5
private pool_max = 20

DB_POOL_SIZE = pool_max    // exported — value: 20
```

This lets you do intermediate calculations, build strings, and organize logic without polluting the generated `.env` file.

---

### The `module` Variable

`module` is a special built-in string variable set by the `--module` CLI flag.

```bash
cenv config.cenv --module=production
# module = "production"
```

Its primary use is driving dynamic imports:

```cenv
private env = import("./.cenv." + module)
// --module=production → imports .cenv.production
// --module=staging    → imports .cenv.staging
```

You can also export it directly to your `.env` file:

```cenv
ENVIRONMENT = module    // exported as "production" or "staging"
```

---

### Imports

The `import()` function loads another `.cenv` file and returns an **object** containing all of its **public variables**.

```cenv
// db.cenv
DB_HOST = "localhost"
DB_PORT = 5432
private connection_retries = 3    // stays private, not included in the import object
```

```cenv
// main.cenv
private db = import("./db.cenv")

DB_HOST = db.DB_HOST    // "localhost"
DB_PORT = db.DB_PORT    // 5432
```

**Rules:**
- `import()` **must** be assigned to a variable. Standalone `import()` is a syntax error.
- Only public variables from the imported file are returned in the object.
- Private variables in the imported file stay private and are not accessible.
- Import paths support expressions: `import("./.cenv." + module)`

**Optional chaining (`?.`)** — safely access a field that might not exist. Returns `null` instead of throwing an error:

```cenv
private config = import("./optional-config.cenv")
private key = config?.OPTIONAL_KEY    // null if OPTIONAL_KEY is not defined
```

#### AWS Secrets Manager

`import_aws_secret()` has the same interface as `import()` and is designed for fetching secrets from AWS Secrets Manager:

```cenv
private secrets = import_aws_secret("my-app/production/db")
DB_PASSWORD = secrets?.DB_PASSWORD
```

> **Note:** `import_aws_secret()` is not yet implemented. It currently returns an empty object. AWS integration is planned for a future release.

---

### Comments

cenv has three comment types with different behaviors:

| Syntax | Appears in `.env`? | Use for |
|---|---|---|
| `# comment` | **Yes** | Documenting the deployed `.env` file |
| `// comment` | No | Developer notes in source code |
| `/* comment */` | No | Multi-line developer notes |

**Hash comments (`#`)** are preserved in the generated `.env` file:

```cenv
# Database Configuration
DB_HOST = "localhost"
DB_PORT = "5432"
```

Generated `.env`:

```env
# Database Configuration
DB_HOST=localhost
DB_PORT=5432
```

**Line and block comments** are stripped during compilation and never appear in the output:

```cenv
// This note will NOT appear in .env
/* Neither will this block */
SECRET_KEY = "abc123"
```

---

## Language Reference

### Data Types

| Type | Examples |
|---|---|
| String | `"hello"`, `"https://api.example.com"` |
| Number | `3000`, `42`, `3.14` |
| Boolean | `true`, `false` |
| Null | `null` |

---

### Variables

```cenv
// Public variable — exported to .env
API_URL = "https://api.example.com"

// Private variable — internal use only
private timeout = 30

// Reassignment (works for both public and private variables)
timeout = 60
```

Public variable names are typically `SCREAMING_SNAKE_CASE` — they become the keys in the `.env` file. Private variable names can use any casing you prefer.

---

### Template Strings

Embed expressions inside strings using `${}`:

```cenv
private host = "api.example.com"
private version = "v2"

API_URL = "https://${host}/${version}"
// Result: https://api.example.com/v2
```

You can also build strings with the `+` operator:

```cenv
API_URL = "https://" + host + "/" + version
```

---

### Operators

**Arithmetic**

```cenv
private a = 10
private b = 3

SUM        = a + b    // 13
DIFFERENCE = a - b    // 7
PRODUCT    = a * b    // 30
QUOTIENT   = a / b    // 3.333...
REMAINDER  = a % b    // 1
```

**Comparison** — evaluates to a boolean:

```cenv
private x = 10
IS_BIG  = x > 5      // true
IS_TEN  = x == 10    // true
NOT_TEN = x != 10    // false
```

**Logical**

```cenv
private is_prod  = true
private is_debug = false

CAN_LOG = is_prod & !is_debug    // true  (AND)
EITHER  = is_prod | is_debug     // true  (OR)
NEGATED = !is_prod               // false (NOT)
```

**Operator Precedence** (highest to lowest):

| Level | Operators | Example |
|---|---|---|
| 1 (highest) | `*` `/` `%` | `2 * 3` |
| 2 | `+` `-` | `5 + 3` |
| 3 | `<` `>` `<=` `>=` | `x < 10` |
| 4 | `==` `!=` | `a == b` |
| 5 | `&` | `a & b` |
| 6 (lowest) | `\|` | `a \| b` |

---

### Built-in Functions

**`print(...args)`**  
Writes values to stdout during compilation. Does not affect the `.env` output. Useful for debugging.

```cenv
print("Compiling for:", module)
print("Port:", PORT, "Debug:", DEBUG)
```

**`type(value)`**  
Returns the type name of a value: `"number"`, `"string"`, `"boolean"`, `"null"`, `"array"`, or `"object"`.

```cenv
print(type(3000))    // "number"
print(type("hi"))    // "string"
```

**`len(value)`**  
Returns the character count of a string.

```cenv
private n = len("hello")    // 5
```

**`num(value)`**  
Converts a string to a number.

```cenv
private s = "42"
private n = num(s)    // 42
```

**`str(value)`**  
Converts a number or boolean to a string. Useful when building URLs or connection strings.

```cenv
private port = 5432
DB_URL = "postgres://localhost:" + str(port) + "/myapp"
```

**`bool(value)`**  
Returns the truthiness of a value. `0`, `""`, `null`, and `false` are falsy; everything else is truthy.

```cenv
private a = bool(0)      // false
private b = bool("yes")  // true
private c = bool(null)   // false
```

---

### Blocks

Curly braces group related statements together. Variables declared inside a block are visible outside it.

```cenv
// Group related configuration
{
    private db_host = "localhost"
    private db_port = 5432

    DATABASE_URL = "postgres://" + db_host + ":" + str(db_port) + "/myapp"
}

// DATABASE_URL is still accessible here
print("Connecting to:", DATABASE_URL)
```

Blocks can be nested:

```cenv
{
    private base = 10
    {
        private factor = 4
        MAX_CONNECTIONS = base * factor    // 40
    }
}
```

---

## CLI Reference

```
cenv <file> [options]
```

| Flag | Description |
|---|---|
| `--module=<name>` | Sets the `module` variable; output file is named `.env.<name>` |
| `--output=<filename>` | Override the output filename |
| `--dry` | Print output to stdout, do not write a file |
| `--debug` | Show the token stream and parsed AST |
| `--version` | Print the installed version |
| `--help` | Print usage information |

**Output file naming:**

| Command | Output |
|---|---|
| `cenv config.cenv` | `.env` |
| `cenv config.cenv --module=production` | `.env.production` |
| `cenv config.cenv --output=.env.custom` | `.env.custom` |
| `cenv config.cenv --module=prod --output=custom.env` | `custom.env` (`--output` takes priority) |
| `cenv config.cenv --dry` | stdout only, no file created |

---

## Multi-Environment Pattern

This is the recommended way to manage configuration across multiple environments with cenv.

### File Layout

```
your-project/
├── config/
│   ├── config.cenv          # main config — the file you compile
│   ├── .cenv.production     # production-specific values
│   ├── .cenv.staging        # staging-specific values
│   └── .cenv.development    # development-specific values
```

### `config/config.cenv`

```cenv
// Load environment-specific values
private env = import("./.cenv." + module)

// Shared values — same across all environments
APP_NAME = "MyApplication"
APP_VERSION = "1.2.0"

// Derived values using private variables
private base_pool = 5
DB_POOL_SIZE = base_pool * 4

// Environment-specific values come from the imported file
API_URL      = env.API_URL
DATABASE_URL = env.DATABASE_URL
DEBUG        = env.DEBUG
LOG_LEVEL    = env.LOG_LEVEL

// Export the current environment name
ENVIRONMENT = module

print("Built .env for:", module)
```

### `config/.cenv.production`

```cenv
API_URL      = "https://api.example.com"
DATABASE_URL = "postgresql://prod-db.example.com:5432/myapp"
DEBUG        = false
LOG_LEVEL    = "error"

private cache_ttl = 3600    // internal only
```

### `config/.cenv.staging`

```cenv
API_URL      = "https://staging.api.example.com"
DATABASE_URL = "postgresql://staging-db.example.com:5432/myapp"
DEBUG        = true
LOG_LEVEL    = "debug"

private cache_ttl = 60
```

### `config/.cenv.development`

```cenv
API_URL      = "http://localhost:8080"
DATABASE_URL = "postgresql://localhost:5432/myapp_dev"
DEBUG        = true
LOG_LEVEL    = "debug"

private cache_ttl = 10
```

### Compiling

```bash
# Production
cenv config/config.cenv --module=production
# Creates: .env.production

# Staging
cenv config/config.cenv --module=staging
# Creates: .env.staging

# Development
cenv config/config.cenv --module=development
# Creates: .env.development

# Preview without writing a file
cenv config/config.cenv --module=production --dry
```

### Generated `.env.production`

```env
API_URL=https://api.example.com
APP_NAME=MyApplication
APP_VERSION=1.2.0
DATABASE_URL=postgresql://prod-db.example.com:5432/myapp
DB_POOL_SIZE=20
DEBUG=false
ENVIRONMENT=production
LOG_LEVEL=error
```

### Gitignore

Commit your `.cenv` source files and ignore the generated `.env` files:

```gitignore
# Generated files — do not commit
.env
.env.*

# Keep source files
!config/.cenv.*
```

### CI/CD Integration

Add a compile step to your pipeline before deploying:

```yaml
# GitHub Actions
- name: Compile environment config
  run: cenv config/config.cenv --module=production
```

```bash
# Generic shell script
cenv config/config.cenv --module=$DEPLOY_ENV
```

---

## Building from Source

**Requirements:** Rust 1.70+ and Cargo.

```bash
# Clone
git clone https://github.com/rzorzal/.c.env.git
cd c.env.lang

# Build optimized binary
cargo build --release

# Copy to PATH
sudo cp target/release/cenv /usr/local/bin/cenv

# Or install with Cargo directly
cargo install --path .
```

**Run the test suite:**

```bash
cargo test --lib
```

---

## Uninstalling

```bash
# curl / manual install
rm ~/.local/bin/cenv

# Homebrew
brew uninstall cenv

# APT
sudo dpkg -r cenv

# Cargo
cargo uninstall c_env_lang
```

---

## Contributing

Contributions are welcome. Check [docs/IMPLEMENTATION_PLAN.md](docs/IMPLEMENTATION_PLAN.md) for the roadmap — planned features include control flow (`if`/`else`), loops, object literals, and full AWS Secrets Manager integration.

1. Fork the repository on [GitHub](https://github.com/rzorzal/.c.env)
2. Create a feature branch
3. Add tests for your changes (`cargo test --lib`)
4. Open a pull request

---

## License

[Add license information here]
