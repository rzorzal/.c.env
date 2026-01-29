# Basic Usage Examples

Simple examples demonstrating the fundamentals of C.env.

## Table of Contents

1. [Variables and Literals](#variables-and-literals)
2. [Assignment Statements](#assignment-statements) ⭐ NEW
3. [Simple Arithmetic](#simple-arithmetic)
4. [Comparisons](#comparisons)
5. [Logical Operations](#logical-operations)

---

## Variables and Literals

### Numbers

```javascript
// integers.c.env
private count = 42
private temperature = -10
private quantity = 1000
```

```javascript
// floats.c.env
private price = 19.99
private ratio = 0.5
private pi = 3.14159
```

### Strings

```javascript
// strings.c.env
private name = "Alice"
private greeting = "Hello, World!"
private path = "/usr/local/bin"
private empty = ""
```

### Booleans

```javascript
// booleans.c.env
private isEnabled = true
private hasError = false
private isProduction = true
private debugMode = false
```

### Null Values

```javascript
// null.c.env
private optionalValue = null
private emptyResult = null
```

---

## Assignment Statements

### Simple Reassignment

```javascript
// counter.c.env
private counter = 0
counter = 1
counter = 2
counter = 3
// counter is now 3
```

### Updating Values

```javascript
// temperature.c.env
private temperature = 20
temperature = 25
temperature = temperature + 5  // temperature is now 30
temperature = temperature - 10 // temperature is now 20
```

### Working with Multiple Variables

```javascript
// swap.c.env
private x = 5
private y = 10
private temp = x
x = y
y = temp
// x is now 10, y is now 5
```

### Assignment with Expressions

```javascript
// calculations.c.env
private base = 10
private multiplier = 5
private result = 0

result = base * multiplier           // result is 50
result = result + 100                // result is 150
result = (result - 50) / 2           // result is 50
```

### Boolean Assignments

```javascript
// flags.c.env
private isActive = true
private isReady = false

isActive = false
isReady = true
isActive = isReady & isActive  // isActive is false
isReady = true | false          // isReady is true
```

### Accumulator Pattern

```javascript
// accumulator.c.env
private total = 0
total = total + 10
total = total + 20
total = total + 30
total = total + 40
// total is now 100
```

---

## Simple Arithmetic

### Addition and Subtraction

```javascript
// addition.c.env
private a = 10
private b = 5
private sum = a + b         // 15
private difference = a - b  // 5
```

### Multiplication and Division

```javascript
// multiplication.c.env
private width = 10
private height = 5
private area = width * height      // 50
private half = width / 2           // 5
```

### Modulo

```javascript
// modulo.c.env
private total = 17
private divisor = 5
private remainder = total % divisor // 2

private number = 8
private isEven = number % 2 == 0    // true
```

## Comparisons

### Basic Comparisons

```javascript
// comparisons.c.env
private a = 10
private b = 5

private isGreater = a > b       // true
private isLess = a < b          // false
private isEqual = a == b        // false
private isNotEqual = a != b     // true
```

### Range Checking

```javascript
// ranges.c.env
private age = 25
private minAge = 18
private maxAge = 65

private isAdult = age >= minAge           // true
private isSenior = age >= 65              // false
private isWorkingAge = age >= minAge & age < maxAge  // true
```

## Logical Operations

### AND Operations

```javascript
// and.c.env
private hasPermission = true
private isEnabled = true
private canAccess = hasPermission & isEnabled  // true

private isValid = true
private isActive = false
private shouldProcess = isValid & isActive     // false
```

### OR Operations

```javascript
// or.c.env
private isAdmin = false
private isModerator = true
private hasSpecialAccess = isAdmin | isModerator  // true

private hasError = false
private hasWarning = false
private shouldAlert = hasError | hasWarning       // false
```

## Comments

### Single-Line Comments

```javascript
// comments_single.c.env

// This is a comment explaining the next line
private value = 42

private count = 100  // This comment is at the end of the line

// You can have multiple comment lines
// to explain complex logic
private result = count * 2
```

### Multi-Line Comments

```javascript
// comments_multi.c.env

/*
 * This is a multi-line comment
 * that can span several lines
 * and is useful for longer explanations
 */
private configValue = "production"

/* Inline multi-line comment */ private debug = false
```

## Combined Examples

### Configuration Set

```javascript
// app_config.c.env

// Application settings
private appName = "MyApp"
private appVersion = 1.0
private environment = "production"

// Server configuration
private serverHost = "localhost"
private serverPort = 8080
private useSSL = true

// Limits
private maxUsers = 1000
private maxRequestSize = 5242880  // 5MB in bytes
private timeout = 30              // seconds
```

### Feature Flags

```javascript
// features.c.env

private enableNewUI = true
private enableBetaFeatures = false
private enableAnalytics = true
private enableDebugMode = false

private shouldShowNewUI = enableNewUI & enableBetaFeatures == false
```

### Calculations

```javascript
// calculations.c.env

// Basic calculations
private basePrice = 100
private quantity = 5
private subtotal = basePrice * quantity  // 500

// Tax calculation
private taxRate = 0.2
private taxAmount = subtotal * taxRate   // 100

// Total with tax
private total = subtotal + taxAmount     // 600

// Discount
private discountRate = 0.1
private discountAmount = total * discountRate  // 60
private finalPrice = total - discountAmount    // 540
```

### User Validation

```javascript
// validation.c.env

// User data
private userAge = 25
private userScore = 85
private isPremium = true

// Validation rules
private minAge = 18
private passingScore = 60

// Validation checks
private isOldEnough = userAge >= minAge         // true
private hasPassedTest = userScore >= passingScore // true
private hasFullAccess = isOldEnough & hasPassedTest & isPremium  // true
```

### Environment Setup

```javascript
// environment.c.env

// Environment detection
private isDevelopment = true
private isStaging = false
private isProduction = false

// Feature toggles based on environment
private enableDebugLogs = isDevelopment | isStaging
private enableProfiling = isDevelopment
private useMinifiedAssets = isProduction
private strictErrorChecking = isProduction | isStaging
```

## Next Steps

- Learn about [Operators in Detail](operators.md)
- See [Configuration Examples](configuration.md)
- Explore [Advanced Patterns](advanced.md)
