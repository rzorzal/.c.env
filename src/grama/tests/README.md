# Grammar Parser Tests

Organized test suite for the C.env grammar parser.

## Test Organization

Tests are organized by feature area into separate modules:

### `helpers.rs`

Common test utilities and helper functions:

- `parse_program()` - Parse source code into a Program AST
- `extract_var_expr()` - Extract expression from variable declaration
- `extract_assignment_expr()` - Extract expression from assignment statement
- `parse_expression_from_code()` - Parse standalone expression

### `error_handling.rs` - Phase 1.1

Error detection and reporting tests:

- Unmatched delimiters (parentheses, brackets)
- Error message quality
- Successful parsing of valid input

### `lexer.rs` - Phase 1.2

Tokenization and lexer tests:

- Single-line comments (`//`)
- Multi-line comments (`/* */`)
- Multi-character operators (`<=`, `>=`, `==`, `!=`)

### `precedence.rs` - Phase 1.3

Operator precedence and associativity tests:

- Multiplication before addition
- Parentheses override precedence
- Logical AND before OR
- Complex multi-operator expressions
- Left-associativity for same-precedence operators

### `statements.rs`

Basic statement parsing tests:

- Variable declarations
- String literals
- Boolean literals
- Multiple statements

### `assignments.rs` - Phase 2.1

Assignment statement tests:

- Simple reassignment
- Assignment with expressions
- Assignment with variable references
- Assignment chains
- Complex expression assignments
- Logical operator assignments
- Error cases (missing value, etc.)
- Mixed declarations and assignments

## Running Tests

```bash
# Run all tests
cargo test --lib

# Run specific test module
cargo test --lib grama::tests::assignments
cargo test --lib grama::tests::precedence

# Run specific test
cargo test --lib test_simple_assignment

# Run with output
cargo test --lib -- --show-output
```

## Adding New Tests

1. Identify the appropriate module for your test
2. Add the test function with `#[test]` attribute
3. Use helper functions from `helpers.rs`
4. Follow naming convention: `test_<feature>_<scenario>`

Example:

```rust
#[test]
fn test_my_new_feature() {
    let code = "private x = 42";
    let program = parse_program(code).expect("Should parse");
    assert_eq!(program.items.len(), 1);
}
```

## Test Coverage

Current coverage: **26 tests**

- Error Handling: 3 tests
- Lexer: 4 tests
- Precedence: 6 tests
- Statements: 4 tests
- Assignments: 9 tests
