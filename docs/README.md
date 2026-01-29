# C.env Language Documentation

Welcome to the C.env language documentation! C.env is a configuration and environment language designed for managing environment variables and configuration files with a JavaScript-like syntax.

## Documentation Structure

- **[Getting Started](getting-started/README.md)** - Installation and first steps
- **[Language Reference](language-reference/README.md)** - Complete language syntax and features
- **[Examples](examples/README.md)** - Code examples and use cases
- **[API Reference](api/README.md)** - Parser and lexer API documentation

## Quick Links

- [Installation Guide](getting-started/installation.md)
- [Your First C.env Program](getting-started/quickstart.md)
- [Language Syntax](language-reference/syntax.md)
- [Operators and Precedence](language-reference/operators.md)
- [Common Examples](examples/basic-usage.md)

## What is C.env?

C.env is a domain-specific language for configuration management that combines:

- **Simple syntax** inspired by JavaScript
- **Strong typing** for configuration values
- **Expression evaluation** with proper operator precedence
- **Template strings** for dynamic configuration
- **Array comprehensions** for powerful data transformations

## Key Features

- ✅ **Variables** - Private and public variable declarations
- ✅ **Operators** - Full set of arithmetic, comparison, and logical operators
- ✅ **Expressions** - Complex expressions with proper precedence
- ✅ **Comments** - Single-line (`//`) and multi-line (`/* */`) comments
- ✅ **Error Messages** - Clear, helpful error reporting with line numbers
- 🚧 **Templates** - String interpolation with `${expr}` syntax (in progress)
- 🚧 **Comprehensions** - Array and find comprehensions (in progress)
- 🚧 **Control Flow** - If expressions and conditional logic (in progress)

## Project Status

The C.env language is under active development. Phase 1 (Foundation & Error Handling) is complete, including:

- Comprehensive error handling infrastructure
- Lexer with comment support and multi-character operators
- Parser with correct operator precedence
- Test suite with 17 passing tests

See the [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) for the full development roadmap.

## Getting Help

- Browse the [Language Reference](language-reference/README.md)
- Check out [Examples](examples/README.md)
- Review [Common Issues](troubleshooting.md)
- See the [FAQ](faq.md)

## Contributing

This project is open for contributions. Please see the implementation plan for areas that need work.
