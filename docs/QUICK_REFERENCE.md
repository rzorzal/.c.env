# Quick Reference: Generating .env Files

## 🚀 Basic Commands

### Run and View Output


```bash
# View full output (print statements + .env format)
cargo run config.cenv --module=production

# Suppress warnings
cargo run --quiet config.cenv --module=production 2>/dev/null
```

### Generate .env Files

```bash
# Production
cargo run --quiet config.cenv --module=production 2>/dev/null | tail -n +4 > .env.production

# Staging
cargo run --quiet config.cenv --module=staging 2>/dev/null | tail -n +4 > .env.staging

# Development
cargo run --quiet config.cenv --module=development 2>/dev/null | tail -n +4 > .env.development

# Generic .env (usually for local development)
cargo run --quiet config.cenv --module=local 2>/dev/null | tail -n +4 > .env
```

### Command Breakdown

```bash
cargo run --quiet config.cenv --module=production 2>/dev/null | tail -n +4 > .env.production
│              │                │                  │             │           │
│              │                │                  │             │           └── Output file
│              │                │                  │             └── Skip first 3 lines (print statements)
│              │                │                  └── Suppress stderr (warnings)
│              │                └── Set module variable
│              └── .cenv source file
└── Build and run (quietly)
```


**Note:** Adjust `tail -n +4` based on how many print statements you have:

- `+2` = skip 1 line
- `+3` = skip 2 lines
- `+4` = skip 3 lines (default for config.cenv)

## 📁 File Structure

```
your-project/
├── config.cenv           # Main config file
├── .cenv.production      # Production variables
├── .cenv.staging         # Staging variables
├── .cenv.development     # Development variables
├── .cenv.local           # Local development variables
└── .env.*                # Generated files (git ignore these)
```

## 📝 Syntax Quick Reference


### Public Variables (Exported to .env)

```javascript
API_URL = "https://api.example.com";
PORT = 8080;
DEBUG = false;
APP_NAME = "MyApp";

```

### Private Variables (Not exported)

```javascript
private max_connections = 100
private buffer_size = max_connections * 1024

private api_key = "secret"  // Use for calculations, not exported
```

### Using Private in Public

```javascript

private max_pool = 20
DATABASE_POOL_SIZE = max_pool  // Public variable with private value
```

### Module Variable

```javascript

// Set via: --module=production
ENVIRONMENT = module; // Sets ENVIRONMENT=production
NODE_ENV = module; // Sets NODE_ENV=production
```


### Dynamic Imports

```javascript
import("./.cenv." + module); // Loads .cenv.production when --module=production
```

### Template Strings


```javascript
private env = "prod"
API_URL = "https://api.${env}.example.com"  // https://api.prod.example.com
```

## 🔧 Common Workflows

### 1. Development Setup


```bash
# Generate local .env
cargo run --quiet config.cenv --module=local 2>/dev/null | tail -n +4 > .env

# Start your app
npm start  # or python app.py, etc.
```

### 2. Deploy to Production


```bash
# Generate production .env
cargo run --quiet config.cenv --module=production 2>/dev/null | tail -n +4 > .env.production

# Copy to server
scp .env.production server:/app/.env
```


### 3. Multiple Environments

```bash
# Generate all environments
for env in production staging development; do
  cargo run --quiet config.cenv --module=$env 2>/dev/null | tail -n +4 > .env.$env
  echo "✅ Generated .env.$env"
done
```

### 4. Verify Before Deploy


```bash
# Compare environments
diff .env.production .env.staging

# Show only differences
diff --side-by-side .env.production .env.staging | grep '|'
```


## 🎯 Best Practices

### 1. Git Ignore Generated Files

```gitignore
# .gitignore

.env
.env.*
!.env.example
```

### 2. Track Source Files



```gitignore
# Track .cenv source files
!*.cenv
!.cenv.*
```

### 3. Environment-Specific Files

- `.cenv.production` - Production-only variables (API URLs, DB connections)
- `.cenv.staging` - Staging-specific variables
- `.cenv.development` - Development variables (debug=true, local DBs)
- `.cenv.local` - Local developer overrides (not committed)

### 4. Shared Variables


Put shared variables in `config.cenv`:

```javascript
// config.cenv
import("./.cenv." + module);

// Shared across all environments
APP_NAME = "MyApplication";
APP_VERSION = "1.0.0";
PORT = 3000;

// Environment-specific (from imported file)
// API_URL, DATABASE_URL, DEBUG_MODE, etc.
```


### 5. Secrets Management

```javascript
// For AWS Secrets Manager
import_aws_secret("/prod/api/keys")

// For calculations only (not exported)
private api_key = "use-secrets-manager-instead"
private temp_value = 123

// Public variables should be safe to commit
API_URL = "https://api.example.com"
```

## 📊 Output Examples


### Production Output

```bash
$ cargo run config.cenv --module=production

Compiling .env for environment: production
API URL: https://prod.api.example.com
Debug mode: false

API_URL=https://prod.api.example.com
APP_NAME=MyApplication
DATABASE_POOL_SIZE=20
DEBUG_MODE=false
ENVIRONMENT=production
PORT=3000
```


### Staging Output


```bash
$ cargo run config.cenv --module=staging

Compiling .env for environment: staging

API URL: https://staging.api.example.com
Debug mode: true

API_URL=https://staging.api.example.com
APP_NAME=MyApplication

DATABASE_POOL_SIZE=20

DEBUG_MODE=true
ENVIRONMENT=staging
PORT=3000
```


## 🆘 Troubleshooting

### Variables Not Showing in .env

❌ Problem: Variable marked as private


```javascript

private API_URL = "https://api.example.com"  // Won't appear in .env
```

✅ Solution: Remove `private` keyword


```javascript
API_URL = "https://api.example.com"; // Will appear in .env
```

### Import Not Found



❌ Problem: Wrong import path

```javascript

import("./.cenv." + module); // Looking for .cenv.production

```

✅ Solution: Ensure file exists and matches module name

```bash
ls -la .cenv.*
# Should show: .cenv.production, .cenv.staging, etc.
```

### Too Many/Few Lines Skipped

❌ Problem: Wrong tail parameter

```bash
tail -n +4  # Skips 3 lines, but you have 5 print statements
```

✅ Solution: Adjust tail parameter

```bash
tail -n +6  # Skip 5 print statements (n = prints + 1)
```

### Values with Spaces Not Quoted

If your value contains spaces and isn't quoted in the output, the formatter handles it automatically:

```javascript
APP_NAME = "My Application Name";
```

Output:

```
APP_NAME="My Application Name"
```

## 🎓 Learn More

- See [examples/README.md](examples/README.md) for detailed examples
- See [Readme.md](Readme.md) for complete language reference
- See [PUBLIC_PRIVATE_VARS_SUMMARY.md](PUBLIC_PRIVATE_VARS_SUMMARY.md) for implementation details
