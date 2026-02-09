# has_key() and Optional Chaining (?.) Implementation Summary

**Date:** February 9, 2026
**Features:** Object key checking and safe property access

---

## Overview

Added two new features for working with objects more safely:

1. **`has_key(object, key)`** - Built-in function to check if an object has a specific key
2. **`?.` operator** - Optional chaining for safe property access

---

## Implementation Details

### 1. has_key() Built-in Function

**Location:** `src/grama/evaluator.rs`

**Signature:** `has_key(object, key) -> boolean`

**Behavior:**

- Returns `true` if the object has the specified key
- Returns `false` if the object doesn't have the key
- Throws runtime error if first argument is not an object
- Throws runtime error if second argument is not a string

**Implementation:**

```rust
fn builtin_has_key(&mut self, args: &[Expr]) -> EvalResult<Value> {
    if args.len() != 2 {
        return Err(RuntimeError::wrong_arg_count("has_key", 2, args.len()));
    }

    let obj_val = self.eval_expr(&args[0])?;
    let key_val = self.eval_expr(&args[1])?;

    match (obj_val, key_val) {
        (Value::Object(map), Value::String(key)) => {
            Ok(Value::Bool(map.contains_key(&key)))
        }
        (Value::Object(_), other) => Err(RuntimeError::type_error(
            "string",
            other.type_name(),
            "has_key(object, key)",
        )),
        (other, _) => Err(RuntimeError::type_error(
            "object",
            other.type_name(),
            "has_key(object, key)",
        )),
    }
}
```

---

### 2. Optional Chaining (?.) Operator

**Locations:**

- Token: `src/lexing/lexer/token.rs` - Added `QuestionDot` token
- Lexer: `src/lexing/lexer.rs` - Tokenizes `?.` as a single token
- AST: `src/grama/gramma_rules.rs` - Added `Expr::OptionalMember` variant
- Parser: `src/grama/parser/expression_parser.rs` - Parses `?.` syntax
- Evaluator: `src/grama/evaluator.rs` - Evaluates optional member access

**Behavior:**

- Returns the property value if the object has the property
- Returns `null` if the property doesn't exist (instead of throwing an error)
- Returns `null` if used on `null` values
- Returns `null` if used on non-object types

**Implementation:**

```rust
Expr::OptionalMember { object, field } => {
    let obj_val = self.eval_expr(object)?;
    match obj_val {
        Value::Object(map) => Ok(map.get(field).cloned().unwrap_or(Value::Null)),
        Value::Null => Ok(Value::Null),
        _ => Ok(Value::Null),
    }
}
```

---

## Files Modified

1. **src/lexing/lexer/token.rs** - Added `QuestionDot(String)` token variant
2. **src/lexing/lexer.rs** - Added logic to tokenize `?.` sequence
3. **src/grama/gramma_rules.rs** - Added `Expr::OptionalMember` variant
4. **src/grama/parser/expression_parser.rs** - Added parsing for `?.` operator
5. **src/grama/evaluator.rs** - Added `builtin_has_key()` and `OptionalMember` evaluation
6. **src/grama/tests/mod.rs** - Registered new test modules
7. **docs/language-reference/built-in-functions.md** - Added `has_key()` documentation
8. **docs/language-reference/operators.md** - Added `?.` operator documentation

---

## Files Created

1. **src/grama/tests/has_key_tests.rs** - 10 comprehensive tests for `has_key()`
2. **src/grama/tests/optional_member_tests.rs** - 11 comprehensive tests for `?.`
3. **examples/has_key_and_optional_chaining.cenv** - Example usage of both features

---

## Test Coverage

### has_key() Tests (10 tests)

- ✅ Check existing key returns true
- ✅ Check missing key returns false
- ✅ Multiple keys in same object
- ✅ Empty object returns false
- ✅ Error on non-object first argument
- ✅ Error on non-string key
- ✅ Error on wrong argument count (0, 1, or 3+ args)

### Optional Chaining Tests (11 tests)

- ✅ Access existing field returns value
- ✅ Access missing field returns null
- ✅ Safe on null values
- ✅ Safe on non-object types (string, number, array, boolean)
- ✅ Nested object access
- ✅ Multiple fields in same object
- ✅ Comparison: regular `.` vs `?.` behavior

---

## Usage Examples

**Important Note:** The examples below use object literal syntax `{ key: value }` which is planned but not yet implemented in the parser. Currently, `has_key()` and `?.` work with objects created through other means (e.g., imported from files or created programmatically in the evaluator).

### has_key() Usage

```javascript
// Once object literals are implemented:
private config = { api_url: "https://api.example.com", timeout: 30 }

// Check before accessing
if has_key(config, "api_url") {
  API_URL = config.api_url
}

// Check for optional fields
if has_key(config, "api_key") {
  API_KEY = config.api_key
} else {
  print("API key not configured")
}

// Direct boolean usage
private hasDebug = has_key(config, "debug")
print("Debug enabled:", hasDebug)  // false
```

### Optional Chaining Usage

```javascript
private config = { api_url: "https://api.example.com" }

// Access existing property
private url = config?.api_url  // "https://api.example.com"

// Access missing property - returns null instead of error
private key = config?.api_key  // null

// Safe on null
private nullable = null
private safe = nullable?.anything  // null

// Safe on non-objects
private num = 42
private result = num?.value  // null

// Nested access (when implemented)
private user = { settings: { theme: "dark" } }
// Note: Chaining multiple ?. not yet implemented
private theme = user?.settings  // { theme: "dark" }
```

### Combining Both

```javascript
private config = { server: { host: "localhost" } }

// Explicit check with has_key
if has_key(config, "server") {
  SERVER_CONFIG = config.server
}

// Safe access with ?.
SERVER_PORT = config?.server?.port ?? 8080  // Use default if missing

// Check before critical operations
if has_key(config, "credentials") {
  // Proceed with authentication
  CREDS = config.credentials
} else {
  // Fallback: try optional access
  CREDS = config?.default_credentials
}
```

---

## Differences from JavaScript

### JavaScript Optional Chaining

```javascript
// JavaScript supports chained ?.
obj?.prop1?.prop2?.prop3;

// JavaScript has optional call
func?.();

// JavaScript has optional indexing
arr?.[0];
```

### C.env Optional Chaining

```javascript
// Currently only single level
obj?.prop  // ✅ Supported

// Chaining would require multiple statements
private temp = obj?.prop1
private result = temp?.prop2  // Workaround

// No optional call (yet)
// func?.()  // ❌ Not supported

// No optional indexing (yet)
// arr?.[0]  // ❌ Not supported
```

---

## Future Enhancements

1. **Chained Optional Access** - Support `obj?.a?.b?.c` syntax
2. **Optional Indexing** - Support `arr?.[index]` for arrays
3. **Nullish Coalescing** - `??` operator for default values
4. **Optional Call** - `func?.()` for safe function calls

---

## Related Documentation

- [Built-in Functions](docs/language-reference/built-in-functions.md#has_key)
- [Operators](docs/language-reference/operators.md#optional-chaining-)
- [Example Code](examples/has_key_and_optional_chaining.cenv)

---

## Notes

- `has_key()` is strict: requires object and string arguments
- `?.` is permissive: returns `null` for any invalid access
- Use `has_key()` when you need explicit existence checking
- Use `?.` when you want graceful fallback to `null`
- Both features work with imported configurations
- Combine with conditional logic for robust config handling
