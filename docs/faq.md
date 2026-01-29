# Frequently Asked Questions (FAQ)

## General Questions

### What is C.env?

C.env is a domain-specific language designed for configuration and environment management. It features a JavaScript-like syntax with strong typing and proper operator precedence.

### Why create another configuration language?

C.env is designed to be:

- **Readable**: Familiar syntax for developers
- **Expressive**: Support for expressions and calculations
- **Safe**: Strong error checking and helpful messages
- **Extensible**: Easy to add new features

### Is C.env production-ready?

C.env is currently in active development. Phase 1 (Foundation & Error Handling) is complete. See the [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) for the development roadmap.

## Installation & Setup

### How do I install C.env?

See the [Installation Guide](getting-started/installation.md) for detailed instructions. The quickest way is:

```bash
git clone <repository-url>
cd c.env.lang
cargo build --release
```

### What are the system requirements?

- Rust 1.70 or later (for building from source)
- Any modern operating system (macOS, Linux, Windows)
- Minimal memory and disk space requirements

### Do I need to know Rust to use C.env?

No! You only need Rust to build the compiler. Using C.env just requires running the compiled binary.

## Language Features

### What data types does C.env support?

Currently supported:

- **Numbers**: Integers and floats (`42`, `3.14`)
- **Strings**: Text in double quotes (`"hello"`)
- **Booleans**: `true` and `false`
- **Null**: `null` value

### Can I reassign variables?

Not yet. Currently, all variables are immutable after declaration. Assignment statements are planned for Phase 2.

### Does C.env support functions?

Function definitions are not yet implemented, but function calls (for built-in functions) are planned.

### What about arrays and objects?

Array and object literals are planned for Phase 3. The infrastructure is partially in place in the AST.

### Are there loops or conditionals?

If expressions (ternary-like) are partially implemented. Traditional loops are not currently planned, but array comprehensions will provide similar functionality.

## Syntax Questions

### Why do I need the `private` keyword?

Currently, all variable declarations require `private`. Public variables (without the keyword) will be added in a future version.

### Can I use single quotes for strings?

No, only double quotes are supported for string literals.

### What comment styles are supported?

- Single-line: `// comment`
- Multi-line: `/* comment */`

### Is whitespace significant?

Whitespace is generally not significant, but statements should be on separate lines (or separated by semicolons in future versions).

## Operators

### What's the operator precedence?

From highest to lowest:

1. `*`, `/`, `%` (Multiplication, Division, Modulo)
2. `+`, `-` (Addition, Subtraction)
3. `<`, `>`, `<=`, `>=` (Comparison)
4. `==`, `!=` (Equality)
5. `&` (Logical AND)
6. `|` (Logical OR)

See the [Operators Reference](language-reference/operators.md) for details.

### Why use `&` and `|` instead of `&&` and `||`?

This is a design choice for brevity. The single-character versions are easier to type for configuration files.

### Is there a unary minus operator?

Yes, you can use negative numbers: `-10`, `-3.14`

## Running Programs

### How do I run a C.env file?

```bash
c_env_lang myfile.c.env
```

See [Running Programs](getting-started/running-programs.md) for details.

### Why is the output so verbose?

The current version shows the complete parsing process including tokens and AST. This is useful for debugging and development. Future versions will have quieter modes.

### Can I suppress the output?

Yes, redirect stderr to /dev/null:

```bash
c_env_lang myfile.c.env 2>/dev/null
```

### What's an AST and why is it shown?

AST (Abstract Syntax Tree) is the internal representation of your program. It's shown for debugging and educational purposes.

## Errors and Debugging

### I get "Unmatched delimiter" errors. What does this mean?

You're missing a closing parenthesis, bracket, or brace. The error message shows where the opening delimiter is and where the closing one is expected.

### How do I debug parsing issues?

1. Check the token output to see how your code is being lexed
2. Simplify your code to isolate the problem
3. Use comments to temporarily disable sections
4. Check the [Error Reference](language-reference/errors.md)

### Are there debugging tools?

The verbose output is currently the main debugging tool. Dedicated debugging features are planned.

## Development

### Can I contribute to C.env?

Yes! Check the [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) for areas that need work.

### How do I run the tests?

```bash
cargo test --lib
```

### How is C.env implemented?

C.env is written in Rust and consists of:

- **Lexer**: Tokenizes source code
- **Parser**: Builds an AST from tokens
- **Error Handler**: Provides helpful error messages

### Where can I report bugs?

File an issue on the project repository with:

- Your C.env code
- The error message or unexpected behavior
- Your system information

## Performance

### Is C.env fast?

For configuration files, performance is excellent. The lexer and parser are very fast on typical configuration file sizes.

### What's the largest file C.env can handle?

There's no hard limit, but very large files (>1MB) may produce verbose output.

## Compatibility

### Can C.env read JSON/YAML/TOML?

No, C.env has its own syntax. However, conversion tools could be created.

### Can I use C.env with other languages?

Currently, C.env is standalone. Future versions may support exporting to other formats.

## Future Features

### When will feature X be implemented?

Check the [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) for the roadmap. Features are being implemented in phases.

### Can I request a feature?

Yes! File an issue with your feature request and use case.

### Will there be a VSCode extension?

A VSCode extension with syntax highlighting and language support is planned but not yet started.

## Troubleshooting

### The compiler won't build

Make sure you have:

- Latest Rust installed: `rustup update`
- All dependencies: `cargo fetch`
- Try: `cargo clean && cargo build`

### I get "expected named lifetime parameter"

This is an internal error. Please file a bug report with your code.

### Tests are failing

Try:

```bash
cargo clean
cargo test --lib
```

If tests still fail, this may indicate a regression. Please file a bug report.

## Learning Resources

### Where should I start?

1. [Installation](getting-started/installation.md)
2. [Quick Start](getting-started/quickstart.md)
3. [Basic Concepts](getting-started/basic-concepts.md)
4. [Examples](examples/README.md)

### Are there tutorials?

The [Quick Start Guide](getting-started/quickstart.md) is the main tutorial. More tutorials are planned.

### Can I see example code?

Yes! Check the `examples/` directory and [Examples Documentation](examples/README.md).

## Getting Help

### Where can I get help?

1. Read the [documentation](README.md)
2. Check the [examples](examples/README.md)
3. Review this FAQ
4. File an issue on the repository

### How do I report a problem?

File an issue with:

- Clear description of the problem
- Your C.env code (minimal example)
- Error messages
- Expected vs actual behavior
