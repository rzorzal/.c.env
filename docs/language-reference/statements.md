# Statements

Complete reference for all statement types in C.env.

## Table of Contents

1. [Variable Declarations](#variable-declarations)
2. [Assignment Statements](#assignment-statements)
3. [Expression Statements](#expression-statements)
4. [Import Statements](#import-statements)

---

## Variable Declarations

Variable declarations create a new variable and initialize it with a value. All variables must be declared with the `private` keyword.

### Syntax

```javascript
private variableName = value
```

### Examples

```javascript
// Number variables
private age = 25
private price = 19.99
private temperature = -5

// String variables
private name = "John"
private message = "Hello, World!"

// Boolean variables
private isActive = true
private hasError = false

// Null values
private data = null

// Complex expressions
private total = 10 + 5 * 2
private result = (100 - 20) / 4
```

### Rules

- Variables must be declared before they can be assigned or used
- The `private` keyword is required for all declarations
- Variable names must start with a letter and can contain letters, numbers, and underscores
- Each declaration must include an initial value (no uninitialized variables)

---

## Assignment Statements

Assignment statements modify the value of an existing variable. The variable must have been declared previously.

### Syntax

```javascript
variableName = newValue;
```

### Examples

```javascript
// Simple assignment
private counter = 0
counter = 10

// Assignment with expression
private total = 0
total = 5 + 3 * 2  // total becomes 11

// Assignment with variable reference
private x = 5
private y = 10
x = y  // x becomes 10

// Multiple reassignments
private value = 1
value = 2
value = 3
value = value + 1  // value becomes 4

// Assignment with complex expressions
private result = 0
result = (10 + 5) * 2 - 3  // result becomes 27

// Assignment with logical operators
private flag = true
flag = true & false  // flag becomes false
flag = true | false  // flag becomes true
```

### Difference from Declaration

| Feature       | Declaration          | Assignment |
| ------------- | -------------------- | ---------- |
| Keyword       | `private` (required) | None       |
| Creates var   | Yes                  | No         |
| Modifies var  | No                   | Yes        |
| Requires init | Yes                  | N/A        |
| Example       | `private x = 5`      | `x = 10`   |

### Rules

- The variable must exist (must have been declared first)
- No `private` keyword is used in assignments
- The entire right-hand side expression is evaluated before assignment
- Supports all expression types (arithmetic, logical, comparisons, etc.)

---

## Expression Statements

An expression statement is a standalone expression that is evaluated but whose result is not stored.

### Syntax

```javascript
expression;
```

### Examples

```javascript
// Function calls (when supported)
print("Hello");

// Standalone expressions (evaluated but result is discarded)
42;
("text");
x + y;
```

### Notes

- Expression statements are less common in C.env
- They're mainly used for function calls with side effects
- The result of the expression is not stored anywhere

---

## Import Statements

Import statements load and execute code from other `.cenv` files, making their variables and definitions available in the current file. Import paths can be **string expressions**, enabling dynamic, module-based imports.

### Syntax

```javascript
import(path_expression);
import_aws_secret(path_expression);
```

### Parameters

- **path_expression** (string expression): Expression that evaluates to a file path. Can be:
  - String literal: `"./config.cenv"` or `"utils/helpers.cenv"`
  - String concatenation: `"./.cenv." + module`
  - Any expression that evaluates to a string
  - Relative to current file's directory

### The `module` Variable

When running C.env with the `--module=value` argument, a special variable called `module` is automatically defined with the specified value. This enables environment-specific configuration loading.

```bash
# Set module variable to "production"
./c_env_lang config.cenv --module=production

# Set module variable to "staging"
./c_env_lang config.cenv --module=staging
```

### Examples

#### Dynamic Module-Based Import ⭐ PRIMARY USE CASE

This is the core feature of C.env - compiling different configurations based on the module:

```javascript
// config.cenv - Main configuration file
import("./.cenv." + module); // Dynamically loads environment-specific config

print("Loaded environment:", module);
print("API URL:", api_url);
print("Database:", database);
```

```javascript
// .cenv.production - Production configuration
private api_url = "https://prod.api.example.com"
private database = "prod-db"
private debug_mode = false
```

```javascript
// .cenv.staging - Staging configuration
private api_url = "https://staging.api.example.com"
private database = "staging-db"
private debug_mode = true
```

```bash
# Compile for production
./c_env_lang config.cenv --module=production
# Loads .cenv.production, outputs production values

# Compile for staging
./c_env_lang config.cenv --module=staging
# Loads .cenv.staging, outputs staging values
```

#### Client-Specific Modules

```javascript
// Multi-level module names
import("./.cenv." + module); // module = "myclient.production"
// Loads .cenv.myclient.production
```

```bash
# Client-specific production config
./c_env_lang config.cenv --module=myclient.production
```

#### Basic Import (String Literal)

```javascript
// config.cenv
private api_url = "https://api.example.com"
private timeout = 30
```

```javascript
// main.cenv
import("config.cenv");
print(api_url); // "https://api.example.com"
print(timeout); // 30
```

#### Multiple Imports

```javascript
import("config.cenv")
import("utils.cenv")
import("helpers.cenv")

// All variables from imported files are now available
private combined = config_var + helper_func
```

#### Conditional Imports with Expressions

```javascript
// Import different files based on module value
import(module == "production" ? "./prod.cenv" : "./dev.cenv");

// Nested module paths
import("./configs/" + module + "/settings.cenv");
```

#### AWS Secrets Import (Placeholder)

```javascript
// Future feature - currently outputs placeholder message
import_aws_secret("production/database");
```

### Behavior

- **Execution**: Imported files are executed immediately when the import statement runs
- **Variables**: All `private` variables from the imported file become available in the current scope
- **Shadowing**: If an imported file defines a variable that already exists, the imported value overwrites the current value
- **Circular Imports**: Detected and prevented - if file A imports B and B imports A, an error is raised
- **Single Execution**: Each file is only executed once, even if imported multiple times
- **Code Execution**: All statements in the imported file run (including `print` statements)

### Example: Variable Shadowing

```javascript
// shadowing.cenv
private x = 100
```

```javascript
// main.cenv
private x = 5
print("Before:", x)  // 5
import("shadowing.cenv")
print("After:", x)   // 100 (overwritten)
```

### Example: Circular Import Detection

```javascript
// file_a.cenv
import("file_b.cenv"); // ERROR: Circular import detected
```

```javascript
// file_b.cenv
import("file_a.cenv");
```

### Rules

- Import path **can be any expression** that evaluates to a string
- String concatenation is supported: `"./.cenv." + module`
- The special `module` variable is set via `--module=value` argument
- Imported files must have `.cenv` extension
- Circular imports are not allowed
- Files are executed in the directory where they are located
- All imports are processed at statement execution time (runtime evaluation)
- Module variable is available throughout the entire program
- If no `--module` argument is provided, `module` is undefined

### Common Patterns

#### Environment-Specific Configuration (PRIMARY USE CASE)

```javascript
// main.cenv - Main entry point
import("./.cenv." + module); // Load environment config

print("Running in:", module);
print("Database:", database_url);
```

```javascript
// .cenv.production
private database_url = "postgres://prod-db:5432/myapp"
private api_endpoint = "https://api.example.com"
private debug = false
```

```javascript
// .cenv.development
private database_url = "postgres://localhost:5432/myapp_dev"
private api_endpoint = "http://localhost:3000"
private debug = true
```

```bash
# Compile for production
./c_env_lang main.cenv --module=production

# Compile for development
./c_env_lang main.cenv --module=development
```

#### Multi-Tenant Configuration

```javascript
// config.cenv
import("./.cenv." + module); // module = "client1.production"

print("Client:", client_name);
print("Environment:", environment);
```

```javascript
// .cenv.client1.production
private client_name = "Client 1"
private environment = "production"
private api_key = "client1-prod-key"
```

```bash
./c_env_lang config.cenv --module=client1.production
```

#### Static Import (Backwards Compatible)

```javascript
// config.cenv
private env = "production"
private debug = false
private api_key = "secret-key"
```

```javascript
// app.cenv
import("config.cenv");

if (debug) {
  print("Debug mode enabled");
}
```

#### Shared Constants

```javascript
// constants.cenv
private PI = 3.14159
private MAX_RETRIES = 3
private TIMEOUT_MS = 5000
```

```javascript
// app.cenv
import("constants.cenv")
private circle_area = PI * radius * radius
```

### See Also

- [Built-in Functions](built-in-functions.md) - Functions available for use with imported values
- [Variables](variables.md) - Variable scoping and naming rules

---

## Statement Ordering

Statements are executed in the order they appear in the source file.

### Example

```javascript
private a = 1       // Statement 1: Declare and initialize a
private b = 2       // Statement 2: Declare and initialize b
a = 3               // Statement 3: Assign new value to a
private c = a + b   // Statement 4: Declare c with value of a + b (5)
b = 10              // Statement 5: Assign new value to b
a = b * 2           // Statement 6: Assign to a the value b * 2 (20)
```

After execution:

- `a` = 20
- `b` = 10
- `c` = 5 (unchanged since statement 4)

---

## Common Patterns

### Counter Pattern

```javascript
private counter = 0
counter = counter + 1
counter = counter + 1
// counter is now 2
```

### Swap Pattern (requires temporary variable)

```javascript
private x = 5
private y = 10
private temp = x
x = y
y = temp
// x is now 10, y is now 5
```

### Accumulator Pattern

```javascript
private sum = 0
sum = sum + 10
sum = sum + 20
sum = sum + 30
// sum is now 60
```

### Toggle Pattern

```javascript
private enabled = true
enabled = false
enabled = true
// enabled is true
```

---

## Error Cases

### Assigning to Undeclared Variable

```javascript
// ERROR: 'x' was never declared
x = 10;
```

**Fix**: Declare the variable first

```javascript
private x = 0
x = 10  // OK
```

### Missing Value in Assignment

```javascript
private x = 5
// ERROR: Missing value after '='
x =
```

**Fix**: Provide a value

```javascript
x = 10; // OK
```

### Using 'private' in Assignment

```javascript
private x = 5
// ERROR: Unexpected token (this is a redeclaration, not assignment)
private x = 10
```

**Fix**: Remove `private` keyword for assignment

```javascript
x = 10; // OK
```

---

## See Also

- [Variables](variables.md) - Variable naming and scoping rules
- [Expressions](expressions.md) - Expression types and evaluation
- [Operators](operators.md) - Operator precedence and usage
