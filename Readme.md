# C.env Language

A configuration and environment language with JavaScript-like syntax, strong typing, and proper operator precedence.

## 🚀 Quick Start

```bash
# Build the project
cargo build --release

# Run a C.env file
./target/release/c_env_lang examples/.c.env.hello
```

## 📚 Documentation

**[Complete Documentation →](docs/README.md)**

- **[Getting Started](docs/getting-started/README.md)** - Installation and first steps
- **[Language Reference](docs/language-reference/README.md)** - Complete syntax and features
- **[Examples](docs/examples/README.md)** - Code examples and use cases
- **[FAQ](docs/faq.md)** - Frequently asked questions

## ✨ Features

- ✅ **Variables** - Private variable declarations
- ✅ **Assignments** - Reassign variable values
- ✅ **Data Types** - Numbers, strings, booleans, null
- ✅ **Operators** - Full set with correct precedence
- ✅ **Comments** - Single-line (`//`) and multi-line (`/* */`)
- ✅ **Error Handling** - Clear, helpful error messages
- ✅ **Built-in Functions** - print(), type(), len(), num(), str(), bool() ⭐ NEW
- ✅ **Runtime Evaluation** - Execute programs with proper error handling ⭐ NEW

## 📖 Example

```javascript
// config.c.env
private appName = "MyApp"
private port = 8080
private debug = false

// Print values
print("App:", appName)
print("Port:", port)

// Reassign values
port = 3000
debug = true

// Built-in functions
print("Type of port:", type(port))
print("Port as string:", str(port))

// Calculations
private maxUsers = 100
private bufferSize = maxUsers * 1024
print("Buffer size:", bufferSize)

// Type conversion
private userInput = "42"
private value = num(userInput)
print("Converted value:", value)
```

## 🏗️ Project Status

**Phase 1 Complete!** ✅ **Phase 2.1 Complete!** ✅ **Phase 2.2 Complete!** ✅

- ✅ Error infrastructure with helpful messages
- ✅ Lexer with comments and multi-char operators
- ✅ Parser with correct operator precedence (6 levels)
- ✅ Assignment statements for variable mutation
- ✅ Runtime evaluator with environment management
- ✅ Built-in functions (print, type, len, num, str, bool)
- ✅ Comprehensive test suite (68 tests passing)
- ✅ Modular test organization (separate files per feature)

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for the full roadmap.

## 🚀 Running Programs

Execute a C.env program:

```bash
./target/release/c_env_lang program.cenv
```

Example output:

```bash
$ ./target/release/c_env_lang examples/phase2_2_demo.cenv
Hello, World!
42
true
Name: Alice
Age: 30
Total: 30
```

Debug mode (shows parsing and AST):

```bash
./target/release/c_env_lang program.cenv --debug
```

## 🧪 Running Tests

```bash
cargo test --lib
```

Expected output:

```
running 68 tests
test result: ok. 68 passed; 0 failed
```

## 📝 Quick Reference

### Variable Declaration

```javascript
private variableName = value
```

### Assignment (Reassignment)

```javascript
variableName = newValue;
```

Example:

```javascript
private counter = 0
counter = 10
counter = counter + 1  // counter is now 11
```

### Operators (by precedence)

| Precedence | Operators         | Example  |
| ---------- | ----------------- | -------- |
| Highest    | `*` `/` `%`       | `2 * 3`  |
| ↓          | `+` `-`           | `5 + 3`  |
| ↓          | `<` `>` `<=` `>=` | `x < 10` |
| ↓          | `==` `!=`         | `a == b` |
| ↓          | `&`               | `a & b`  |
| Lowest     | `\|`              | `a \| b` |

### Comments

```javascript
// Single-line comment
/* Multi-line comment */
```

## 🛠️ Building from Source

### Requirements

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build Steps

```bash
# Clone the repository
git clone <repository-url>
cd c.env.lang

# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Install to system (optional)
cargo install --path .
```

## 📂 Project Structure

```
c.env.lang/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library exports
│   ├── lexing/          # Lexer implementation
│   └── grama/           # Parser implementation
├── docs/                # Documentation
├── examples/            # Example .c.env files
└── IMPLEMENTATION_PLAN.md
```

## 🤝 Contributing

Contributions are welcome! See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for areas that need work.

## 📚 Resources

- [Lexical Analysis Reference](https://krypticmouse.hashnode.dev/writing-a-compiler-lexical-analysis)
- [Sox Language Reference](https://github.com/obiesie/sox/tree/main/src)

## 📄 License

[Add license information here]

---

**Need Help?** Check the [FAQ](docs/faq.md) or [Getting Started Guide](docs/getting-started/README.md)
