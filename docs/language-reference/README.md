# Language Reference

Complete reference documentation for the C.env language.

## Table of Contents

1. [Syntax Overview](syntax.md)
2. [Statements](statements.md)
3. [Built-in Functions](built-in-functions.md) ⭐ NEW
4. [Operators](operators.md)
5. [Data Types](types.md)
6. [Variables](variables.md)
7. [Expressions](expressions.md)
8. [Error Messages](errors.md)

## Quick Reference

### Variable Declaration

```javascript
private variableName = value
```

### Data Types

- **Number**: `42`, `3.14`, `-10`
- **String**: `"text"`, `""`
- **Boolean**: `true`, `false`
- **Null**: `null`

### Operators

| Category       | Operators         | Precedence  |
| -------------- | ----------------- | ----------- |
| Multiplicative | `*` `/` `%`       | Highest (5) |
| Additive       | `+` `-`           | 4           |
| Comparison     | `<` `>` `<=` `>=` | 3           |
| Equality       | `==` `!=`         | 2           |
| Logical AND    | `&`               | 1           |
| Logical OR     | `\|`              | Lowest (0)  |

### Comments

```javascript
// Single-line comment
/* Multi-line comment */
```

## Language Features

### Currently Implemented ✅

- Variable declarations with `private` keyword
- All basic data types (Number, String, Boolean, Null)
- All operators with correct precedence
- Single-line and multi-line comments
- Error reporting with line numbers and context

### In Development 🚧

- String template interpolation (`${expr}`)
- If expressions (ternary-like)
- Array and object literals
- Comprehensions
- Function calls
- Member access (dot notation)

### Planned 📋

- Assignment statements (reassignment)
- Import statements
- Print and built-in functions
- Control flow statements
- Advanced features (see IMPLEMENTATION_PLAN.md)

## Specification Details

For detailed specification of each feature, see the individual reference pages listed in the Table of Contents above.
