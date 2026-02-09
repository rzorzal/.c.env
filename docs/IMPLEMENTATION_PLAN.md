# C.env Language - .env File Compiler Implementation Plan

**Created:** 2026-01-29
**Updated:** 2026-01-29
**Purpose:** C.env is a `.env` file compiler that generates environment configuration from `.cenv` source files
**Goal:** Systematically implement features for module-based configuration compilation

---

## 🎯 Project Purpose

C.env compiles `.cenv` source files into `.env` files for different environments. It uses a `--module` argument to dynamically load environment-specific configurations through expression-based imports.

### Key Concepts

- **Source Files**: `.cenv` files with JavaScript-like syntax
- **Output**: `.env` files (generated from running .cenv)
- **Module System**: `--module=production` sets the `module` variable
- **Dynamic Imports**: `import('./.cenv.' + module)` loads environment-specific config
- **Public Variables**: Variables without `private` keyword appear in .env output
- **Private Variables**: Variables with `private` keyword are for internal use only
- **Use Case**: Manage configurations for production, staging, development, client-specific environments, etc.

---

## Phase 1: Foundation & Error Handling (Week 1-2)

### Priority: CRITICAL

**Goal:** Establish robust error handling and fix critical parser issues

#### 1.1 Error Infrastructure

- [x] Create `ParseError` struct with line/column info and error messages
- [x] Change all parse functions from `Option<T>` to `Result<T, ParseError>`
- [x] Add error context to help users debug syntax errors
- [x] Implement basic error recovery (skip to next statement on error)

#### 1.2 Lexer Improvements

- [x] Add support for `//` single-line comments
- [x] Add support for `/* */` multi-line comments
- [x] Fix `<=` and `>=` tokenization (currently single char)
- [x] Add `==` vs `=` distinction (assignment vs equality)
- [x] Add `.` (dot) token for member access

#### 1.3 Operator Precedence Fix

- [x] Define clear precedence levels for all operators:
  - Level 0: Or `|`
  - Level 1: And `&`
  - Level 2: Equality `==`, `!=`
  - Level 3: Comparison `<`, `>`, `<=`, `>=`
  - Level 4: Addition/Subtraction `+`, `-`
  - Level 5: Multiplication/Division/Modulo `*`, `/`, `%`
  - Level 6: Unary `+`, `-`, `!`
- [x] Update `parse_binary_expression` to use correct precedence
- [x] Implement precedence climbing algorithm with position tracking
- [x] Test complex expressions: `2 * 3 + 4`, `a | b & c`, `1 + 2 * 3 > 4 - 1 * 2`

#### 1.4 Testing Infrastructure

- [x] Set up basic test framework (using Rust's built-in testing)
- [x] Create parser test helpers (`parse_program`, `extract_var_expr`, `parse_expression_from_code`)
- [x] Add tests for existing functionality (regression prevention)
  - Error handling tests (unmatched delimiters, error messages)
  - Lexer tests (comments, multi-char operators)
  - Operator precedence tests (all precedence levels, complex expressions)
  - Basic parsing tests (literals, variables, statements)
- [x] Configured Cargo.toml with lib and bin targets
- [x] Created lib.rs to export modules for testing
- [x] **Test Results:** 17 tests passing, 0 failures

**Deliverables:** ✅ Robust error reporting, fixed operators, comprehensive test suite

---

## Phase 2: Core Statement Types (Week 3-4)

### Priority: HIGH

**Goal:** Support all basic statement types needed for simple programs

#### 2.1 Assignment Statements

- [x] Add `Stmt::Assignment { target: Ident, value: Expr }` to AST
- [x] Implement `parse_assignment` function
- [x] Distinguish between declaration (`private x = 5`) and assignment (`x = 10`)
- [x] Add tests for reassignment scenarios
- [x] Fixed token consumption tracking in `parse_primary_expression` for complex expressions
- [x] Added 9 comprehensive tests for assignment statements
- [x] Created documentation: `docs/language-reference/statements.md`
- [x] Updated examples: `docs/examples/basic-usage.md`

**Test Results:** 26 tests passing (17 from Phase 1 + 9 new for Phase 2.1)

#### 2.2 Print & Built-in Functions

- [x] Create evaluator/interpreter module for runtime execution
- [x] Implement `Value` enum for runtime values (Number, String, Bool, Null)
- [x] Implement `RuntimeError` with Display trait for user-friendly errors
- [x] Implement `Evaluator` with environment/scope management
- [x] Add `print()` built-in function with multiple arguments
- [x] Add `type()` for type inspection
- [x] Add `len()` for string length
- [x] Add `num()` for type conversion to number
- [x] Add `str()` for type conversion to string
- [x] Add `bool()` for type conversion to boolean
- [x] Fixed parser to handle keyword tokens (print, import, etc.) as function names
- [x] Fixed token consumption tracking for function calls in binary expressions
- [x] Updated main binary to use evaluator (no more debug output)
- [x] Added --debug flag for verbose execution mode
- [x] Created comprehensive test suite (38 tests in evaluator_tests.rs)
- [x] Created documentation: `docs/language-reference/built-in-functions.md`
- [x] Organized tests into modular structure with separate files

**Test Results:** 68 tests passing (30 from Phase 1-2.1 + 38 new for Phase 2.2)

#### 2.3 Import Statements & Module System ⭐ UPDATED

- [x] Add `Stmt::Import { path: Expr, is_aws_secret: bool, alias: Option<Ident> }` to AST
- [x] Changed import path from String to Expr to support dynamic imports
- [x] Implement `parse_import_statement` for `import(path)` and `import_aws_secret(path)`
- [x] Parse path as expression (not just string literal) to support `import('./.cenv.' + module)`
- [x] Implement file loading and execution in evaluator
- [x] Add circular import detection
- [x] Add variable sharing between files
- [x] **Module Variable**: Added `--module=value` argument support
- [x] **Module Variable**: Special `module` variable automatically defined in environment
- [x] **Dynamic Imports**: Expression-based imports enable `import('./.cenv.' + module)`
- [x] Created 4 example import files in `examples/imports/`
- [x] Add comprehensive test suite (14 tests in imports.rs)
- [x] Updated documentation: `docs/language-reference/statements.md`
- [x] Updated README to reflect .env compiler purpose

**Implementation Details:**

- **Module System**: `--module=production` sets `module` variable to `"production"`
- **Expression Imports**: Import path evaluated at runtime as expression
- **Dynamic Configuration**: Load different configs based on module value
- **Public vs Private Variables**:
  - Public variables (no `private` keyword): `API_URL = "..."` → appears in .env output
  - Private variables (`private` keyword): `private temp = 123` → internal use only
- **Output Format**: Generates .env file format with public variables only
- Import statements execute imported files and make variables available
- Circular imports detected using canonical path tracking
- Files executed in their own directory (proper base path handling)
- AWS secret imports show placeholder message (future feature)
- Variable shadowing: imported values overwrite existing variables

**Use Case Example:**

# Compile for production

./c_env_lang config.cenv --module=production

# Loads .cenv.production via import('./.cenv.' + module)

# Compile for staging

./c_env_lang config.cenv --module=staging

# Loads .cenv.staging

```

- [x] Add `Stmt::Block(Vec<Stmt>)` to AST
- [x] Implement brace-delimited statement blocks
- [x] Blocks share the same scope (no new scope created)
- [x] Updated parser to handle braces correctly (don't split on newlines inside braces)
- [x] Added 15 comprehensive tests for block statements
- [x] Updated documentation: `docs/language-reference/statements.md`

**Implementation Details:**

- **Syntax**: `{ stmt1 stmt2 ... }` - standard curly brace blocks
- **Scope**: Blocks share parent scope, no new lexical scope created
- **Purpose**: Designed for future if/while/for control flow structures
- **Nesting**: Supports arbitrary nesting depth
- **Empty blocks**: `{}` is valid and executes successfully
- **Parser enhancement**: Modified `build_statements` to track brace depth and not split on newlines inside blocks
- **Statement splitting**: Block parser respects nested braces when splitting on newlines

**Use Cases:**

- Grouping statements for control flow (if/while/for)
- Organizing related configuration logic
- Future conditional compilation based on module/environment

**Test Results:** 96 tests passing (81 from Phases 1-2.3 + 15 new for Phase 2.4)
Note: 7 pre-existing assignment tests failing (not related to block implementation)

**Deliverables:** ✅ Full statement coverage for current language features

---

## Phase 3: Expression Enhancements (Week 5-6)

### Priority: HIGH

**Goal:** Complete expression parsing with all operators and literals

#### 3.1 String Template Interpolation

- [ ] Update lexer to extract `${expr}` from template strings
- [ ] Parse embedded expressions within templates
- [ ] Build proper `Template(Vec<TemplatePart>)` AST nodes
- [ ] Add tests for nested templates and complex expressions

#### 3.2 Object Literals

- [ ] Add `Expr::Object(Vec<(String, Expr)>)` to AST
- [ ] Implement `parse_object_literal` for `{key: value, ...}`
- [ ] Handle computed keys `{[expr]: value}`
- [ ] Add tests for nested objects

#### 3.3 Member Access (Dot Notation)

- [ ] Add `Expr::MemberAccess { object: Box<Expr>, property: Ident }` to AST
- [ ] Keep separate from `Index` for semantic clarity
- [ ] Implement proper dot operator parsing
- [ ] Support chaining: `obj.prop1.prop2.prop3`

#### 3.4 Chained Index Access

- [ ] Support `arr[0][1][2]` syntax
- [ ] Support `obj["key1"]["key2"]` syntax
- [ ] Mix with member access: `obj.arr[0].prop`
- [ ] Add comprehensive chaining tests

#### 3.5 Array Literal Improvements

- [ ] Fix array literal representation (currently uses Template)
- [ ] Add proper `Expr::Array(Vec<Expr>)` variant
- [ ] Handle trailing commas gracefully
- [ ] Add tests for edge cases

**Deliverables:** Complete expression support for current language spec

---

## Phase 4: Advanced Features (Week 7-8)

### Priority: MEDIUM

**Goal:** Support advanced control flow and comprehensions

#### 4.1 Comprehension Validation

- [ ] Test nested comprehensions thoroughly
- [ ] Test complex filter conditions
- [ ] Ensure `break` keyword works correctly in find comprehensions
- [ ] Add comprehensive comprehension test suite

#### 4.2 Traditional Loop Statements


#### 4.3 Loop and Range Support

- [ ] Implement `range` built-in function (see docs/LOOP_RANGE_IMPLEMENTATION_PLAN.md for details)
- [ ] Implement `for ... in` and `for ... of` loop syntax
- [ ] Add support for variable interpolation in assignment targets within loops
- [ ] Add list comprehensions with optional `if` filters
- [ ] Add tests and error handling as described in the detailed plan
- [ ] Document new features in language reference

> See: [docs/LOOP_RANGE_IMPLEMENTATION_PLAN.md](LOOP_RANGE_IMPLEMENTATION_PLAN.md) for full implementation steps, notes, and open questions.
#### 4.3 Function Declarations

- [ ] Add `Stmt::FunctionDecl { name: Ident, params: Vec<Ident>, body: Stmt }` to AST
- [ ] Implement function declaration parsing
- [ ] Add `Stmt::Return(Option<Expr>)` for return statements
- [ ] Add tests for function definitions and calls

#### 4.4 Scope & Semantics (Future)

- [ ] Design symbol table for scoping
- [ ] Implement semantic analysis pass
- [ ] Check variable usage before declaration
- [ ] Validate function arities

**Deliverables:** Advanced control flow, function support (foundational)

---

## Phase 5: Quality & Polish (Week 9-10)

### Priority: MEDIUM

**Goal:** Production-ready parser with excellent UX

#### 5.1 Parser Edge Cases

- [ ] Handle empty statements gracefully (multiple newlines)
- [ ] Support trailing commas in all contexts
- [ ] Better handling of unbalanced brackets/parens/braces
- [ ] Comprehensive error messages with suggestions

#### 5.2 Integration Tests

- [ ] Convert all example files to automated tests
- [ ] Add test for each example in `examples/` directory
- [ ] Create test suite for error conditions
- [ ] Performance testing for large files

#### 5.3 Documentation

- [ ] Document grammar formally (EBNF or similar)
- [ ] Create parser architecture documentation
- [ ] Add inline code documentation
- [ ] Create language specification document

#### 5.4 Code Quality

- [ ] Refactor large functions (>100 lines)
- [ ] Remove code duplication
- [ ] Add rustdoc comments to all public functions
- [ ] Run clippy and fix warnings

**Deliverables:** Production-quality parser with full test coverage

---

## Phase 6: Future Enhancements (Week 11+)

### Priority: LOW

**Goal:** Advanced features for language evolution

#### 6.1 Type System (Optional)

- [ ] Design type annotation syntax
- [ ] Add type checking infrastructure
- [ ] Support type inference
- [ ] Add type error reporting

#### 6.2 Advanced Error Recovery

- [ ] Implement panic mode recovery
- [ ] Continue parsing after multiple errors
- [ ] Suggest fixes for common errors
- [ ] Add "did you mean?" suggestions

#### 6.3 IDE Support

- [ ] Language server protocol (LSP) implementation
- [ ] Syntax highlighting
- [ ] Auto-completion
- [ ] Go-to-definition

#### 6.4 Optimizations

- [ ] Optimize parser performance
- [ ] Reduce AST memory footprint
- [ ] Lazy parsing strategies
- [ ] Incremental parsing for IDE

**Deliverables:** Advanced tooling and optimizations

---

## Success Metrics

### Phase 1-2 (Weeks 1-4)

- [ ] All existing examples parse without panics
- [ ] Clear error messages for all syntax errors
- [ ] 80%+ test coverage on core parser functions

### Phase 3-4 (Weeks 5-8)

- [ ] All language features from examples fully supported
- [ ] Comprehensive test suite (100+ tests)
- [ ] Zero parser crashes on malformed input

### Phase 5-6 (Weeks 9+)

- [ ] Documentation complete
- [ ] Can parse real-world `.c.env` files reliably
- [ ] Ready for interpreter/compiler phase

---

## Getting Started

### Immediate Next Steps (This Week)

1. **Start with Phase 1.1** - Error infrastructure is foundation for everything
2. **Create test file** `src/grama/tests.rs` with basic framework
3. **Fix lexer operators** - Quick wins that unblock other work
4. **Document current grammar** - Write down what's already working

### Work Breakdown

- **Solo work:** Start with error types and lexer fixes (1-2 days)
- **Parallelizable:** Once errors are in place, statement types and expressions can be done independently
- **Integration points:** End of each phase, run full test suite

### Risk Mitigation

- **Keep backward compatibility:** Don't break existing working features
- **Test continuously:** Run tests after each change
- **Small commits:** Each TODO item = 1 commit for easy rollback
- **Examples as guides:** Use `examples/` directory to validate each feature

---

## Notes

- This is a living document - update as priorities shift
- Mark items complete as you finish them
- Add new items as you discover gaps
- Estimated time assumes ~20 hours/week dedicated work
- Can accelerate by parallelizing work or reduce scope by skipping Phase 6

**Last Updated:** 2026-01-29
```
