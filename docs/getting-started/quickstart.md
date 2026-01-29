# Quick Start Guide

Let's write and run your first C.env program!

## Your First Program

Create a file named `hello.c.env`:

```javascript
// My first C.env program
private greeting = "Hello, C.env!"
private version = 1.0
```

## Running Your Program

### Using the Compiler

```bash
c_env_lang hello.c.env
```

Or if you're using the development build:

```bash
./target/debug/c_env_lang hello.c.env
```

### Expected Output

The program will display:

```
Filename: hello.c.env
File contents:
// My first C.env program
private greeting = "Hello, C.env!"
private version = 1.0

Analyzing code...

[Token output...]

Building statements...

Program: Program {
    items: [
        VarDecl {
            private_: true,
            name: "greeting",
            value: StringLiteral(
                "Hello, C.env!",
            ),
        },
        VarDecl {
            private_: true,
            name: "version",
            value: Number(
                1.0,
            ),
        },
    ],
}
```

## Understanding the Output

The output shows:

1. **File contents** - Your source code
2. **Tokens** - How the lexer breaks down your code
3. **AST (Abstract Syntax Tree)** - The parsed program structure

## Try Some Expressions

Create `math.c.env`:

```javascript
// Mathematical expressions
private a = 2 * 3 + 4       // (2 * 3) + 4 = 10
private b = (2 + 3) * 4     // (2 + 3) * 4 = 20
private c = 10 / 2 - 1      // (10 / 2) - 1 = 4
```

Run it:

```bash
c_env_lang math.c.env
```

## Variables and Types

C.env supports:

```javascript
// Numbers (integers and floats)
private count = 42
private pi = 3.14159

// Strings
private name = "C.env"

// Booleans
private isActive = true
private isDebug = false

// Expressions
private result = 1 + 2 * 3
```

## Comments

Use comments to document your code:

```javascript
// This is a single-line comment

/*
 * This is a multi-line comment
 * spanning several lines
 */

private value = 100  // End-of-line comment
```

## Common Patterns

### Configuration Values

```javascript
private host = "localhost"
private port = 8080
private debug = true
private maxConnections = 100
```

### Computed Values

```javascript
private basePrice = 100
private taxRate = 0.2
private totalPrice = basePrice * (1 + taxRate)  // 120
```

### Comparisons

```javascript
private minAge = 18
private userAge = 25
private isAdult = userAge >= minAge  // true
```

## Error Handling

If you make a syntax error, C.env provides helpful messages:

```javascript
private x = (1 + 2  // Missing closing parenthesis
```

Output:

```
Parse Error: Unmatched delimiter ')'
  at line 1, column 12

  private x = (1 + 2
              ^
Expected closing parenthesis
```

## Next Steps

- Learn about [Basic Concepts](basic-concepts.md)
- Read the [Language Reference](../language-reference/README.md)
- Explore more [Examples](../examples/README.md)

## Running Tests

To verify your installation and run the test suite:

```bash
cargo test --lib
```

You should see:

```
running 17 tests
...
test result: ok. 17 passed; 0 failed
```
