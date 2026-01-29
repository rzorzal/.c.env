# Basic Concepts

This guide covers the fundamental concepts of the C.env language.

## Variables

### Declaration

Variables are declared using the `private` keyword:

```javascript
private myVariable = 42
```

**Note:** Currently, all variables must be declared with `private`. Public variables (without the keyword) are planned for future releases.

### Naming Rules

- Variable names must start with a letter or underscore
- Can contain letters, numbers, and underscores
- Case-sensitive: `myVar` and `myvar` are different

```javascript
private userName = "Alice"
private user_age = 25
private _internal = true
```

## Data Types

### Numbers

Both integers and floating-point numbers:

```javascript
private count = 42        // Integer
private price = 19.99     // Float
private negative = -10    // Negative number
```

### Strings

Text enclosed in double quotes:

```javascript
private name = "John Doe"
private path = "/usr/local/bin"
private empty = ""
```

### Booleans

True or false values:

```javascript
private isEnabled = true
private hasError = false
```

### Null

Represents absence of a value:

```javascript
private value = null
```

## Expressions

### Arithmetic Expressions

```javascript
private sum = 10 + 5           // Addition: 15
private difference = 10 - 5    // Subtraction: 5
private product = 10 * 5       // Multiplication: 50
private quotient = 10 / 5      // Division: 2
private remainder = 10 % 3     // Modulo: 1
```

### Comparison Expressions

```javascript
private isEqual = 5 == 5       // Equality: true
private notEqual = 5 != 3      // Inequality: true
private lessThan = 3 < 5       // Less than: true
private lessOrEqual = 5 <= 5   // Less or equal: true
private greaterThan = 5 > 3    // Greater than: true
private greaterOrEqual = 5 >= 5 // Greater or equal: true
```

### Logical Expressions

```javascript
private both = true & false    // AND: false
private either = true | false  // OR: true
```

## Operator Precedence

Operators are evaluated in this order (highest to lowest):

1. **Multiplication, Division, Modulo** - `*`, `/`, `%`
2. **Addition, Subtraction** - `+`, `-`
3. **Comparison** - `<`, `>`, `<=`, `>=`
4. **Equality** - `==`, `!=`
5. **AND** - `&`
6. **OR** - `|`

### Examples

```javascript
// 2 * 3 + 4 is evaluated as (2 * 3) + 4 = 10
private result1 = 2 * 3 + 4

// 1 + 2 * 3 is evaluated as 1 + (2 * 3) = 7
private result2 = 1 + 2 * 3

// Use parentheses to override precedence
private result3 = (1 + 2) * 3  // = 9
```

### Logical Precedence

```javascript
// a | b & c is evaluated as a | (b & c)
private result = a | b & c
```

## Comments

### Single-Line Comments

Start with `//` and continue to the end of the line:

```javascript
// This is a single-line comment
private value = 42  // Comment at end of line
```

### Multi-Line Comments

Enclosed in `/*` and `*/`:

```javascript
/*
 * This is a multi-line comment
 * that spans multiple lines
 */
private config = "production"

/* Inline multi-line comment */ private debug = false
```

## Code Organization

### Multiple Statements

Separate statements with newlines:

```javascript
private firstName = "Alice"
private lastName = "Smith"
private age = 30
```

### Blank Lines

Use blank lines for readability:

```javascript
// User configuration
private userName = "alice"
private userEmail = "alice@example.com"

// Server configuration
private serverHost = "localhost"
private serverPort = 8080
```

## Best Practices

### 1. Use Descriptive Names

```javascript
// Good
private maxRetryAttempts = 3
private isAuthenticationEnabled = true

// Avoid
private x = 3
private flag = true
```

### 2. Group Related Variables

```javascript
// Database configuration
private dbHost = "localhost"
private dbPort = 5432
private dbName = "myapp"

// API configuration
private apiKey = "secret123"
private apiTimeout = 30
```

### 3. Add Comments for Complex Logic

```javascript
// Calculate total price with tax and discount
// Formula: base * (1 + tax) * (1 - discount)
private basePrice = 100
private taxRate = 0.2
private discountRate = 0.1
private finalPrice = basePrice * (1 + taxRate) * (1 - discountRate)
```

### 4. Use Consistent Formatting

```javascript
// Consistent spacing around operators
private sum = a + b
private product = x * y
private isValid = value >= minValue & value <= maxValue
```

## Next Steps

- Learn about [Operators in Detail](../language-reference/operators.md)
- See the complete [Language Reference](../language-reference/README.md)
- Try more [Examples](../examples/README.md)
