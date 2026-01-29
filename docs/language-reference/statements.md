# Statements

Complete reference for all statement types in C.env.

## Table of Contents

1. [Variable Declarations](#variable-declarations)
2. [Assignment Statements](#assignment-statements)
3. [Expression Statements](#expression-statements)

---

## Variable Declarations

Variable declarations create a new variable and initialize it with a value. All variables must be declared with the `private` keyword.

### Syntax

```javascript
private variableName = value
```

### Examples

```javascript
// Number variables
private age = 25
private price = 19.99
private temperature = -5

// String variables
private name = "John"
private message = "Hello, World!"

// Boolean variables
private isActive = true
private hasError = false

// Null values
private data = null

// Complex expressions
private total = 10 + 5 * 2
private result = (100 - 20) / 4
```

### Rules

- Variables must be declared before they can be assigned or used
- The `private` keyword is required for all declarations
- Variable names must start with a letter and can contain letters, numbers, and underscores
- Each declaration must include an initial value (no uninitialized variables)

---

## Assignment Statements

Assignment statements modify the value of an existing variable. The variable must have been declared previously.

### Syntax

```javascript
variableName = newValue;
```

### Examples

```javascript
// Simple assignment
private counter = 0
counter = 10

// Assignment with expression
private total = 0
total = 5 + 3 * 2  // total becomes 11

// Assignment with variable reference
private x = 5
private y = 10
x = y  // x becomes 10

// Multiple reassignments
private value = 1
value = 2
value = 3
value = value + 1  // value becomes 4

// Assignment with complex expressions
private result = 0
result = (10 + 5) * 2 - 3  // result becomes 27

// Assignment with logical operators
private flag = true
flag = true & false  // flag becomes false
flag = true | false  // flag becomes true
```

### Difference from Declaration

| Feature       | Declaration          | Assignment |
| ------------- | -------------------- | ---------- |
| Keyword       | `private` (required) | None       |
| Creates var   | Yes                  | No         |
| Modifies var  | No                   | Yes        |
| Requires init | Yes                  | N/A        |
| Example       | `private x = 5`      | `x = 10`   |

### Rules

- The variable must exist (must have been declared first)
- No `private` keyword is used in assignments
- The entire right-hand side expression is evaluated before assignment
- Supports all expression types (arithmetic, logical, comparisons, etc.)

---

## Expression Statements

An expression statement is a standalone expression that is evaluated but whose result is not stored.

### Syntax

```javascript
expression;
```

### Examples

```javascript
// Function calls (when supported)
print("Hello");

// Standalone expressions (evaluated but result is discarded)
42;
("text");
x + y;
```

### Notes

- Expression statements are less common in C.env
- They're mainly used for function calls with side effects
- The result of the expression is not stored anywhere

---

## Statement Ordering

Statements are executed in the order they appear in the source file.

### Example

```javascript
private a = 1       // Statement 1: Declare and initialize a
private b = 2       // Statement 2: Declare and initialize b
a = 3               // Statement 3: Assign new value to a
private c = a + b   // Statement 4: Declare c with value of a + b (5)
b = 10              // Statement 5: Assign new value to b
a = b * 2           // Statement 6: Assign to a the value b * 2 (20)
```

After execution:

- `a` = 20
- `b` = 10
- `c` = 5 (unchanged since statement 4)

---

## Common Patterns

### Counter Pattern

```javascript
private counter = 0
counter = counter + 1
counter = counter + 1
// counter is now 2
```

### Swap Pattern (requires temporary variable)

```javascript
private x = 5
private y = 10
private temp = x
x = y
y = temp
// x is now 10, y is now 5
```

### Accumulator Pattern

```javascript
private sum = 0
sum = sum + 10
sum = sum + 20
sum = sum + 30
// sum is now 60
```

### Toggle Pattern

```javascript
private enabled = true
enabled = false
enabled = true
// enabled is true
```

---

## Error Cases

### Assigning to Undeclared Variable

```javascript
// ERROR: 'x' was never declared
x = 10;
```

**Fix**: Declare the variable first

```javascript
private x = 0
x = 10  // OK
```

### Missing Value in Assignment

```javascript
private x = 5
// ERROR: Missing value after '='
x =
```

**Fix**: Provide a value

```javascript
x = 10; // OK
```

### Using 'private' in Assignment

```javascript
private x = 5
// ERROR: Unexpected token (this is a redeclaration, not assignment)
private x = 10
```

**Fix**: Remove `private` keyword for assignment

```javascript
x = 10; // OK
```

---

## See Also

- [Variables](variables.md) - Variable naming and scoping rules
- [Expressions](expressions.md) - Expression types and evaluation
- [Operators](operators.md) - Operator precedence and usage
