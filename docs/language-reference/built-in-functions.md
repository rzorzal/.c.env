# Built-in Functions

Complete reference for all built-in functions in C.env.

## Table of Contents

1. [Output Functions](#output-functions)
   - [print()](#print)
2. [Type Conversion](#type-conversion)
   - [num()](#num)
   - [str()](#str)
   - [bool()](#bool)
3. [Type Inspection](#type-inspection)
   - [type()](#type)
4. [String Operations](#string-operations)
   - [len()](#len)
5. [Object Operations](#object-operations)
   - [has_key()](#has_key)

---

## Output Functions

### print()

Print values to standard output.

**Syntax:**

```javascript
print(value1, value2, ..., valueN)
```

**Parameters:**

- `value1, value2, ..., valueN`: Zero or more values to print
- Values are separated by spaces in the output

**Returns:** Nothing (void)

**Examples:**

```javascript
// Print a simple message
print("Hello, World!")           // Output: Hello, World!

// Print multiple values
print("Age:", 25)                 // Output: Age: 25

// Print different types
print(42, true, "text", null)     // Output: 42 true text null

// Print variables
private name = "Alice"
private age = 30
print("Name:", name, "Age:", age) // Output: Name: Alice Age: 30

// Print expressions
print("Result:", 10 + 5)          // Output: Result: 15

// Print with no arguments
print()                           // Output: (empty line)
```

**Notes:**

- Each call to `print()` outputs a new line
- Values are automatically converted to strings for display
- Numbers are displayed without trailing `.0` for whole numbers

---

## Type Conversion

### num()

Convert a value to a number.

**Syntax:**

```javascript
num(value);
```

**Parameters:**

- `value`: Value to convert to a number

**Returns:** Number

**Conversion Rules:**

- **String**: Parses the string as a number
- **Boolean**: `true` → `1`, `false` → `0`
- **Number**: Returns the same number
- **Null**: Returns `0`

**Examples:**

```javascript
// Convert strings to numbers
private str1 = "42"
private num1 = num(str1)
print(num1)                        // Output: 42

private str2 = "3.14"
private num2 = num(str2)
print(num2)                        // Output: 3.14

// Convert booleans
print(num(true))                   // Output: 1
print(num(false))                  // Output: 0

// Convert null
print(num(null))                   // Output: 0

// Use in expressions
private a = num("10")
private b = num("20")
private total = a + b
print(total)                       // Output: 30
```

**Error Cases:**

```javascript
// Invalid string format
num("abc"); // Runtime Error: Cannot convert 'abc' to number
num("12.34.56"); // Runtime Error: Cannot convert '12.34.56' to number
```

---

### str()

Convert a value to a string.

**Syntax:**

```javascript
str(value);
```

**Parameters:**

- `value`: Value to convert to a string

**Returns:** String

**Conversion Rules:**

- **Number**: Converts to string representation
- **Boolean**: `true` → `"true"`, `false` → `"false"`
- **String**: Returns the same string
- **Null**: Returns `"null"`

**Examples:**

```javascript
// Convert numbers
print(str(42))                     // Output: 42
print(str(3.14))                   // Output: 3.14

// Convert booleans
print(str(true))                   // Output: true
print(str(false))                  // Output: false

// Convert null
print(str(null))                   // Output: null

// String concatenation
private age = 30
private message = "Age: " + str(age)
print(message)                     // Output: Age: 30

// Use in expressions
private result = str(10 + 5)
print("Result: " + result)         // Output: Result: 15
```

---

### bool()

Convert a value to a boolean.

**Syntax:**

```javascript
bool(value);
```

**Parameters:**

- `value`: Value to convert to a boolean

**Returns:** Boolean (`true` or `false`)

**Conversion Rules:**

- **Number**: `0` → `false`, any other number → `true`
- **String**: Empty string `""` → `false`, any other string → `true`
- **Boolean**: Returns the same boolean
- **Null**: Returns `false`

**Examples:**

```javascript
// Convert numbers
print(bool(0))                     // Output: false
print(bool(1))                     // Output: true
print(bool(-5))                    // Output: true
print(bool(3.14))                  // Output: true

// Convert strings
print(bool(""))                    // Output: false
print(bool("hello"))               // Output: true
print(bool("0"))                   // Output: true (non-empty string)

// Convert null
print(bool(null))                  // Output: false

// Use in conditions
private count = 0
private hasItems = bool(count)
print(hasItems)                    // Output: false
```

---

## Type Inspection

### type()

Get the type name of a value.

**Syntax:**

```javascript
type(value);
```

**Parameters:**

- `value`: Value to inspect

**Returns:** String (one of: `"number"`, `"string"`, `"boolean"`, `"null"`)

**Examples:**

```javascript
// Check types of literals
print(type(42))                    // Output: number
print(type(3.14))                  // Output: number
print(type("hello"))               // Output: string
print(type(true))                  // Output: boolean
print(type(false))                 // Output: boolean
print(type(null))                  // Output: null

// Check types of variables
private age = 30
private name = "Alice"
private isActive = true

print(type(age))                   // Output: number
print(type(name))                  // Output: string
print(type(isActive))              // Output: boolean

// Use in conditions or prints
private value = "123"
print("Value type:", type(value))  // Output: Value type: string
```

---

## String Operations

### len()

Get the length of a string.

**Syntax:**

```javascript
len(string);
```

**Parameters:**

- `string`: String to measure

**Returns:** Number (length of the string)

**Examples:**

```javascript
// Get length of literals
print(len(""))                     // Output: 0
print(len("hello"))                // Output: 5
print(len("Hello, World!"))        // Output: 13

// Get length of variables
private name = "Alice"
print(len(name))                   // Output: 5

private message = "Welcome to C.env"
private msgLen = len(message)
print("Length:", msgLen)           // Output: Length: 16

// Use in expressions
private text = "test"
private doubled = len(text) * 2
print(doubled)                     // Output: 8
```

**Error Cases:**

```javascript
// Wrong argument type
len(42); // Runtime Error: len() expects a string
len(true); // Runtime Error: len() expects a string
len(null); // Runtime Error: len() expects a string

// Wrong number of arguments
len(); // Runtime Error: len() expects exactly 1 argument
len("a", "b"); // Runtime Error: len() expects exactly 1 argument
```

---

## Function Call Syntax

All built-in functions follow standard function call syntax:

```javascript
functionName(arg1, arg2, ..., argN)
```

**Rules:**

- Function names are case-sensitive
- Arguments are separated by commas
- Arguments are expressions (can be literals, variables, or complex expressions)
- Functions can be nested: `print(type(str(42)))`
- Functions can be used in any expression context

**Examples of Function Composition:**

```javascript
// Nested function calls
print(type(str(42)))               // Output: string
print(len(str(100)))               // Output: 3

// Functions in expressions
private result = num("10") + num("20") * 2
print(result)                      // Output: 50

// Functions in assignments
private converted = num("42")
private message = "Value: " + str(converted)

// Functions in print arguments
print("Type:", type(num("123")))   // Output: Type: number
```

---

## Error Handling

Built-in functions perform runtime type checking and will produce errors for:

1. **Wrong argument types:**

   ```javascript
   len(42); // Error: len() expects a string
   ```

2. **Wrong argument count:**

   ```javascript
   type(); // Error: type() expects exactly 1 argument
   len("a", "b"); // Error: len() expects exactly 1 argument
   ```

3. **Invalid conversions:**

   ```javascript
   num("abc"); // Error: Cannot convert 'abc' to number
   ```

4. **Undefined functions:**
   ```javascript
   unknown(); // Error: Unknown function: 'unknown'
   ```

When an error occurs, the program stops and displays a descriptive error message.

---

## Best Practices

### 1. Type Checking Before Conversion

```javascript
private input = "42"
print("Type:", type(input))        // Check type first
private value = num(input)         // Then convert
print("Value:", value)
```

### 2. String Building

```javascript
private count = 10
private total = 100

// Build complex messages
private message = "Items: " + str(count) + ", Total: $" + str(total)
print(message)                     // Output: Items: 10, Total: $100
```

### 3. Debugging with Type Inspection

```javascript
private value = num("42")
print("Value:", value, "Type:", type(value))
```

### 4. Input Validation Pattern

```javascript
private input = "123"
private inputType = type(input)
print("Input type:", inputType)

private value = num(input)
print("Converted:", value, "Type:", type(value))
```

---

## Common Use Cases

### Configuration Value Parsing

```javascript
// Parse configuration strings
private portStr = "8080"
private port = num(portStr)
print("Server running on port:", port)
```

### Message Formatting

```javascript
private name = "Alice"
private age = 30
private message = name + " is " + str(age) + " years old"
print(message)                     // Output: Alice is 30 years old
```

---

## Object Operations

### has_key()

Check if an object has a specific key.

**Note:** Object literals `{ key: value }` are not yet implemented in the parser. This function will be most useful once object literals are added, or when working with objects from imported files.

**Syntax:**

```javascript
has_key(object, key);
```

**Parameters:**

- `object`: The object to check
- `key`: The key name to look for (must be a string)

**Returns:** Boolean (`true` if the key exists in the object, `false` otherwise)

**Examples:**

```javascript
// Create an object
private user = {
  name: "Alice",
  age: 30,
  email: "alice@example.com"
}

// Check for existing keys
print(has_key(user, "name"))       // Output: true
print(has_key(user, "age"))        // Output: true

// Check for non-existing keys
print(has_key(user, "phone"))      // Output: false
print(has_key(user, "address"))    // Output: false

// Use in conditional logic
private config = { debug: true, port: 8080 }

if has_key(config, "port") {
  print("Port is configured")
}

// Check before accessing
private settings = { theme: "dark" }
if has_key(settings, "language") {
  print("Language:", settings.language)
} else {
  print("No language setting found")
}

// Empty object
private empty = {}
print(has_key(empty, "anything"))  // Output: false
```

**Error Cases:**

```javascript
// Non-object first argument
has_key("not an object", "key")    // Runtime Error: Expected object

// Non-string key
private obj = { name: "Test" }
has_key(obj, 123)                  // Runtime Error: Expected string key

// Wrong number of arguments
has_key(obj)                       // Runtime Error: Expected 2 arguments
has_key()                          // Runtime Error: Expected 2 arguments
```

**Notes:**

- Only works with objects, not arrays or other types
- The key parameter must be a string
- Returns `false` for empty objects
- Use in combination with `?.` operator for safe property access

---

## Combined Examples

### Conditional Logic with Type Conversion

```javascript
private countStr = "0"
private count = num(countStr)
private hasItems = bool(count)
print("Has items:", hasItems)      // Output: Has items: false
```

### Safe Object Access

```javascript
private config = { api_url: "https://api.example.com" }

// Check before accessing
if has_key(config, "api_url") {
  API_URL = config.api_url
} else {
  API_URL = "https://default.example.com"
}

// Or use optional chaining
API_KEY = config?.api_key          // Returns null if not present
```

### Debugging and Introspection

```javascript
private value = 42
print("Value:", value)
print("Type:", type(value))
print("As string:", str(value))
print("As bool:", bool(value))
```

---

## See Also

- [Expressions](expressions.md) - Expression syntax and evaluation
- [Operators](operators.md) - Operator reference and optional chaining (`?.`)
- [Data Types](types.md) - Type system overview
