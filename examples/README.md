# C.env Examples

This directory contains working examples demonstrating various features of the C.env language.

## 🎯 Running Examples

```bash
# From the project root - Default mode (creates .env file)
cargo run --quiet -- examples/<filename>
cargo run --quiet -- <filename> --module=<value>

# Dry run mode (output to stdout, no file created)
cargo run --quiet -- examples/<filename> --dry
cargo run --quiet -- <filename> --module=<value> --dry

# Or using the binary directly
./target/release/cenv examples/<filename>
./target/release/cenv <filename> --module=<value>
./target/release/cenv <filename> --dry
```

**Note:** By default, cenv generates a `.env` file (or `.env.<module>` if `--module` is used). Use `--dry` flag to output to stdout instead.

## 📁 Examples

### 1. `.c.env.hello` - Basic Variables and Template Strings

Demonstrates private variables and string template interpolation.

```bash
cargo run --quiet -- examples/.c.env.hello
# or
./target/release/cenv examples/.c.env.hello
```

**Features:**

- Private variable declarations
- String template literals with `${}` interpolation
- Print statements

**Source:**

```javascript
private foo = "bar"
private myString = "my string is: ${foo}"

print(myString)
```

**Output:**

```
my string is: bar
```

---

### 2. `.c.env.functions` - Built-in Functions and Imports

Demonstrates built-in functions, object-based imports, and AWS Secrets integration.

```bash
cargo run --quiet -- examples/.c.env.functions
# or
./target/release/cenv examples/.c.env.functions
```

**Features:**

- **NEW**: Import functions return objects with public variables
- Member access using dot notation (`obj.field`)
- AWS Secrets Manager integration with `import_aws_secret(path)`
- Built-in functions: `type()`, `str()`, `num()`, `len()`
- Arithmetic operations

**Source:**

```javascript
// Import returns an object with all public variables
private config = import("examples/.c.env.hello")

// Access variables using dot notation
print("App Name:", config.APP_NAME)
print("Version:", config.VERSION)
print("Type of config:", type(config))

// AWS secret import also returns an object
private secrets = import_aws_secret("/secret/path")

// Built-in functions
private text = "Hello World"
print("String length:", len(text))
```

**Output:**

```
hello world
App Name: MyApp
Version: 1.0.0
Type of config: object
Note: import_aws_secret('/secret/path') would fetch from AWS Secrets Manager
String length: 11
Type: string
Converted number: 42
Boolean: true
```

---

### 3. `test_public_vars.cenv` - Public vs Private Variables

Demonstrates the difference between public and private variables, and how they affect .env output.

```bash
cargo run --quiet -- examples/test_public_vars.cenv
# or
./target/release/cenv examples/test_public_vars.cenv
```

**Features:**

- **Public variables** (no `private` keyword) - Exported to .env
- **Private variables** (with `private` keyword) - Internal use only
- Calculations using private variables
- .env formatted output

**Source:**

```javascript
// Public variables (exported to .env)
API_URL = "https://api.example.com"
PORT = 8080
DEBUG = false
DATABASE_NAME = "myapp_prod"

// Private variables (not exported)
private internal_temp = 100
private calculated = internal_temp * 10

// Public variable using private calculation
MAX_CONNECTIONS = 9000 + calculated

print("API URL:", API_URL)
```

**Output:**

```
Compiled successfully!
API URL: https://api.example.com

API_URL=https://api.example.com
DATABASE_NAME=myapp_prod
DEBUG=false
MAX_CONNECTIONS=9080
PORT=8080
```

**Note:** Only public variables appear in the .env format section. Private variables (`internal_temp`, `calculated`) are excluded.

---

### 4. Module-Based Configuration - Production & Staging

This example demonstrates the most powerful feature: module-based .env compilation with dynamic imports.

#### Files:

- `examples/config.cenv` - Main configuration file
- `examples/.cenv.production` - Production environment variables
- `examples/.cenv.staging` - Staging environment variables

#### Running:

```bash
# Compile for production (creates .env.production file)
cargo run --quiet -- examples/config.cenv --module=production
# or
./target/release/cenv examples/config.cenv --module=production

# Compile for staging (creates .env.staging file)
cargo run --quiet -- examples/config.cenv --module=staging
# or
./target/release/cenv examples/config.cenv --module=staging

# Dry run - output to stdout without creating file
cargo run --quiet -- examples/config.cenv --module=production --dry
./target/release/cenv examples/config.cenv --module=staging --dry
```

#### `config.cenv`:

```javascript
// Import environment-specific config
import("examples/.cenv." + module)

// Private variables for calculations
private max_pool_size = 20
private min_pool_size = 5

// Public variables
APP_NAME = "MyApplication"
APP_VERSION = "1.0.0"
PORT = 3000

// Calculated public variable
DATABASE_POOL_SIZE = max_pool_size

// Using module variable
ENVIRONMENT = module

print("Compiling .env for environment:", module)
print("API URL:", API_URL)
print("Debug mode:", DEBUG_MODE)
```

#### `.cenv.production`:

```javascript
// Public variables (exported to .env)
API_URL = "https://prod.api.example.com"
DATABASE_URL = "postgresql://prod-db.example.com:5432/myapp"
DEBUG_MODE = false
LOG_LEVEL = "error"

// Private variables (internal use only)
private internal_cache_ttl = 3600
```

#### `.cenv.staging`:

```javascript
// Public variables (exported to .env)
API_URL = "https://staging.api.example.com"
DATABASE_URL = "postgresql://staging-db.example.com:5432/myapp"
DEBUG_MODE = true
LOG_LEVEL = "debug"

// Private variables (internal use only)
private internal_cache_ttl = 60
```

#### Production Output:

```bash
$ cenv examples/config.cenv --module=production

✓ Generated .env.production
Compiling .env for environment: production
API URL: https://prod.api.example.com
Debug mode: false

API_URL=https://prod.api.example.com
APP_NAME=MyApplication
APP_VERSION=1.0.0
DATABASE_POOL_SIZE=20
DATABASE_URL=postgresql://prod-db.example.com:5432/myapp
DEBUG_MODE=false
ENVIRONMENT=production
LOG_LEVEL=error
PORT=3000
```

#### Staging Output:

```bash
$ cenv examples/config.cenv --module=staging

✓ Generated .env.staging
Compiling .env for environment: staging
API URL: https://staging.api.example.com
Debug mode: true

API_URL=https://staging.api.example.com
APP_NAME=MyApplication
APP_VERSION=1.0.0
DATABASE_POOL_SIZE=20
DATABASE_URL=postgresql://staging-db.example.com:5432/myapp
DEBUG_MODE=true
ENVIRONMENT=staging
LOG_LEVEL=debug
PORT=3000
```

---

## 🔑 Key Concepts

### Public vs Private Variables

| Type        | Syntax                | Exported to .env? | Use Case                                   |
| ----------- | --------------------- | ----------------- | ------------------------------------------ |
| **Public**  | `VAR = value`         | ✅ Yes            | Environment variables for your application |
| **Private** | `private var = value` | ❌ No             | Temporary calculations, internal logic     |

### Module Variable

The special `module` variable is set via the `--module=<value>` command-line argument:

```bash
cargo run --quiet -- examples/config.cenv --module=production
# Sets: module = "production"

cargo run --quiet -- examples/config.cenv --module=staging
# Sets: module = "staging"
```

This allows dynamic imports:

```javascript
import("./.cenv." + module);
// --module=production → imports .cenv.production
// --module=staging → imports .cenv.staging
```

### .env Output

**Default mode** - Creates .env file:

```bash
cenv examples/config.cenv --module=production
# Creates: .env.production file
# Stdout: Print statements only
```

**Dry run mode** (`--dry` flag) - Output to stdout:

```bash
cenv examples/config.cenv --module=production --dry
# Creates: Nothing (no file)
# Stdout: Print statements + .env variables
```

The compiler generates:

1. **Print statements** - Debug output during compilation (always to stdout)
2. **.env variables** - Public variables in `KEY=value` format
   - Default mode: Written to `.env` or `.env.<module>` file
   - Dry run mode: Written to stdout

```bash
cargo run config.cenv --module=production 2>/dev/null | tail -n +4 > .env
```

This command:

- `2>/dev/null` - Suppresses compiler warnings
- `tail -n +4` - Skips print statements (adjust number based on your print count)
- `> .env` - Redirects to .env file

---

## 🚀 Next Steps

1. Try modifying the examples
2. Create your own `.cenv` files
3. Add more environment configurations (development, testing, etc.)
4. Use the generated `.env` files with your applications

## 📚 More Information

See the [main README](../Readme.md) for complete language documentation.
