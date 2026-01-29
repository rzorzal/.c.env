# Examples

This section contains practical examples demonstrating various features of the C.env language.

## Categories

### [Basic Usage](basic-usage.md)

Simple examples to get started with C.env.

### [Configuration Examples](configuration.md)

Real-world configuration file examples.

### [Operator Examples](operators.md)

Demonstrating all operators and precedence rules.

### [Advanced Patterns](advanced.md)

More complex examples and patterns.

## Quick Examples

### Hello World

```javascript
// hello.c.env
private message = "Hello, C.env!"
private version = 1.0
```

### Configuration File

```javascript
// config.c.env
private appName = "MyApplication"
private environment = "production"
private debug = false
private port = 8080
private maxConnections = 100
```

### Calculations

```javascript
// math.c.env
private width = 10
private height = 5
private area = width * height           // 50
private perimeter = 2 * (width + height) // 30
```

### Conditional Logic

```javascript
// validation.c.env
private age = 25
private minAge = 18
private isAdult = age >= minAge         // true

private score = 85
private passingScore = 60
private hasPassed = score >= passingScore // true
```

## Running Examples

All examples can be run using:

```bash
c_env_lang <example-file.c.env>
```

Or from the examples directory:

```bash
c_env_lang ../examples/.c.env.hello
c_env_lang ../examples/.c.env.precedence_test
```

## Example Files in Repository

The repository includes several example files in the `examples/` directory:

- `.c.env.hello` - Basic hello world
- `.c.env.precedence_test` - Operator precedence examples
- `.c.env.comprehensive_test` - Comprehensive feature test
- `.c.env.comment_edge_cases` - Comment examples
- `.c.env.error_paren` - Error handling example
- `.c.env.error_bracket` - Error handling example

## Next Steps

- Explore [Basic Usage Examples](basic-usage.md)
- See [Configuration Examples](configuration.md)
- Learn from [Operator Examples](operators.md)
