# Quick Reference: has_key() and Optional Chaining (?.)

**Note:** The examples below use object literal syntax `{ key: value }` which is planned but not yet implemented. These features work with objects from other sources (imports, etc.) and will work with object literals once that feature is added to the parser.

## has_key(object, key)

Check if an object has a specific key.

```javascript
// Returns: boolean (true/false)
// Errors: if not (object, string)

private config = { host: "localhost", port: 8080 }

has_key(config, "host")     // true
has_key(config, "missing")  // false
```

## Optional Chaining (?.)

Safely access object properties without errors.

```javascript
// Returns: value or null
// Never errors on missing properties

private config = { host: "localhost" }

config?.host      // "localhost"
config?.missing   // null (no error!)
null?.anything    // null (safe on null)
```

## When to Use Which

### Use `has_key()` when:

- You need to know if a field exists
- You want explicit error handling
- You're checking before critical operations

```javascript
if has_key(config, "api_key") {
  // I know it exists, safe to access
  API_KEY = config.api_key
} else {
  // Handle missing key explicitly
  print("Error: API key required")
}
```

### Use `?.` when:

- You want optional fields with null fallback
- You're okay with null as a default
- You want concise code

```javascript
// Simple: use value or default to null
DEBUG_MODE = config?.debug; // null if not present

// Or combine with conditionals for defaults
TIMEOUT = config?.timeout ?? 30; // Use 30 if null
```

## Common Patterns

### Pattern 1: Required vs Optional Config

```javascript
private config = {
  api_url: "https://api.example.com",
  debug: true
}

// Required field - check explicitly
if has_key(config, "api_url") {
  API_URL = config.api_url
} else {
  print("Error: api_url is required")
}

// Optional field - use ?. with default
DEBUG = config?.debug ?? false
TIMEOUT = config?.timeout ?? 30
```

### Pattern 2: Safe Import Handling

```javascript
// Import might not have all fields
import("./config.cenv")

// Check critical fields
if has_key(imported_config, "database_url") {
  DATABASE_URL = imported_config.database_url
}

// Safe access for optional fields
CACHE_TTL = imported_config?.cache?.ttl
```

### Pattern 3: Environment-Specific Config

```javascript
private base_config = {
  app_name: "MyApp",
  version: "1.0.0"
}

// Try to get environment-specific settings
ENV_PORT = base_config?.production?.port
ENV_DEBUG = base_config?.development?.debug

// Fall back to defaults if not present
PORT = ENV_PORT ?? 3000
DEBUG = ENV_DEBUG ?? false
```

## Comparison

| Feature       | `.` (regular) | `?.` (optional) | `has_key()`      |
| ------------- | ------------- | --------------- | ---------------- |
| Missing field | ❌ Error      | ✅ Returns null | ✅ Returns false |
| On null       | ❌ Error      | ✅ Returns null | ❌ Error         |
| On non-object | ❌ Error      | ✅ Returns null | ❌ Error         |
| Use case      | Known fields  | Optional fields | Explicit checks  |

## Examples from Real Config Files

```javascript
# Production environment
private prod_config = {
  api_url: "https://api.prod.com",
  database: {
    host: "db.prod.com",
    port: 5432
  }
}

# Development environment (missing some fields)
private dev_config = {
  api_url: "http://localhost:3000"
}

# Safe access works for both
API_URL = prod_config?.api_url    // "https://api.prod.com"
API_URL = dev_config?.api_url     // "http://localhost:3000"

DB_HOST = prod_config?.database?.host   // "db.prod.com"
DB_HOST = dev_config?.database?.host    // null (no error!)

# Use has_key for critical checks
if has_key(prod_config, "database") {
  print("Database configured for production")
}

if has_key(dev_config, "database") {
  print("Database configured for development")
} else {
  print("Using default database settings for development")
}
```
