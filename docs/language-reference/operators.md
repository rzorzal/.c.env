# Operators Reference

Complete guide to all operators in C.env, their precedence, and usage.

## Operator Precedence Table

Operators are listed from highest to lowest precedence:

| Level | Category       | Operators         | Associativity | Example  |
| ----- | -------------- | ----------------- | ------------- | -------- |
| 5     | Multiplicative | `*` `/` `%`       | Left          | `2 * 3`  |
| 4     | Additive       | `+` `-`           | Left          | `5 + 3`  |
| 3     | Comparison     | `<` `>` `<=` `>=` | Left          | `x < 10` |
| 2     | Equality       | `==` `!=`         | Left          | `a == b` |
| 1     | Logical AND    | `&`               | Left          | `a & b`  |
| 0     | Logical OR     | `\|`              | Left          | `a \| b` |

**Higher precedence = evaluated first**

## Arithmetic Operators

### Multiplication `*`

Multiplies two numbers.

```javascript
private result = 5 * 3       // 15
private area = width * height
private double = value * 2
```

**Precedence:** 5 (highest arithmetic)

### Division `/`

Divides the left operand by the right operand.

```javascript
private half = 10 / 2        // 5
private average = sum / count
```

**Precedence:** 5

### Modulo `%`

Returns the remainder of division.

```javascript
private remainder = 10 % 3   // 1
private isEven = number % 2 == 0
```

**Precedence:** 5

### Addition `+`

Adds two numbers.

```javascript
private sum = 5 + 3          // 8
private total = a + b + c
private increment = value + 1
```

**Precedence:** 4

### Subtraction `-`

Subtracts the right operand from the left.

```javascript
private difference = 10 - 3   // 7
private remaining = total - used
private decrement = value - 1
```

**Precedence:** 4

## Comparison Operators

### Less Than `<`

Returns true if left is less than right.

```javascript
private isSmaller = 5 < 10    // true
private underLimit = value < maxValue
```

**Precedence:** 3

### Greater Than `>`

Returns true if left is greater than right.

```javascript
private isLarger = 10 > 5     // true
private overLimit = value > minValue
```

**Precedence:** 3

### Less Than or Equal `<=`

Returns true if left is less than or equal to right.

```javascript
private isAtMost = 5 <= 5     // true
private withinRange = value <= maxValue
```

**Precedence:** 3

### Greater Than or Equal `>=`

Returns true if left is greater than or equal to right.

```javascript
private isAtLeast = 5 >= 5    // true
private meetsMinimum = value >= minValue
```

**Precedence:** 3

## Equality Operators

### Equal `==`

Returns true if operands are equal.

```javascript
private isSame = 5 == 5       // true
private isMatch = input == expected
```

**Precedence:** 2

### Not Equal `!=`

Returns true if operands are not equal.

```javascript
private isDifferent = 5 != 3  // true
private hasChanged = current != previous
```

**Precedence:** 2

## Logical Operators

### AND `&`

Returns true if both operands are true.

```javascript
private both = true & true    // true
private valid = isEnabled & hasPermission
private inRange = value >= min & value <= max
```

**Precedence:** 1

### OR `|`

Returns true if at least one operand is true.

```javascript
private either = true | false  // true
private shouldProcess = forceProcess | hasData
private isSpecial = isAdmin | isModerator
```

**Precedence:** 0 (lowest)

## Precedence Examples

### Example 1: Arithmetic

```javascript
// 2 * 3 + 4 evaluates as (2 * 3) + 4 = 10
private result = 2 * 3 + 4

// 1 + 2 * 3 evaluates as 1 + (2 * 3) = 7
private result = 1 + 2 * 3
```

**Why:** Multiplication (precedence 5) binds tighter than addition (precedence 4).

### Example 2: Comparison and Arithmetic

```javascript
// 5 + 3 < 10 evaluates as (5 + 3) < 10 = true
private result = 5 + 3 < 10

// 1 + 2 * 3 > 5 evaluates as (1 + (2 * 3)) > 5 = true
private result = 1 + 2 * 3 > 5
```

**Why:** Arithmetic operators (4-5) have higher precedence than comparison (3).

### Example 3: Logical Operators

```javascript
// a | b & c evaluates as a | (b & c)
private result = a | b & c

// true | false & true evaluates as true | (false & true) = true
private result = true | false & true
```

**Why:** AND (precedence 1) binds tighter than OR (precedence 0).

### Example 4: Complex Expression

```javascript
// 1 + 2 * 3 > 4 - 1 * 2
// Step 1: 2 * 3 = 6 and 1 * 2 = 2  (precedence 5)
// Step 2: 1 + 6 = 7 and 4 - 2 = 2  (precedence 4)
// Step 3: 7 > 2 = true              (precedence 3)
private result = 1 + 2 * 3 > 4 - 1 * 2  // true
```

### Example 5: All Levels Combined

```javascript
// 1 | 2 & 3 == 4 < 5 + 6 * 7
// Step 1: 6 * 7 = 42        (precedence 5)
// Step 2: 5 + 42 = 47       (precedence 4)
// Step 3: 4 < 47 = true     (precedence 3)
// Step 4: 3 == true = false (precedence 2)
// Step 5: 2 & false = false (precedence 1)
// Step 6: 1 | false = true  (precedence 0)
private result = 1 | 2 & 3 == 4 < 5 + 6 * 7  // true
```

## Overriding Precedence with Parentheses

Use parentheses `()` to explicitly control evaluation order:

```javascript
// Without parentheses: 2 * 3 + 4 = 10
private a = 2 * 3 + 4

// With parentheses: 2 * (3 + 4) = 14
private b = 2 * (3 + 4)

// Multiple levels
private c = ((1 + 2) * 3 + 4) / 5  // ((3 * 3) + 4) / 5 = 13 / 5 = 2.6
```

## Associativity

All binary operators are **left-associative**, meaning they evaluate from left to right when operators have the same precedence:

```javascript
// 10 - 5 - 2 evaluates as (10 - 5) - 2 = 3
private result = 10 - 5 - 2

// 8 / 4 / 2 evaluates as (8 / 4) / 2 = 1
private result = 8 / 4 / 2

// 1 + 2 + 3 + 4 evaluates as ((1 + 2) + 3) + 4 = 10
private result = 1 + 2 + 3 + 4
```

## Operator Behavior

### Type Coercion

Currently, C.env does not perform automatic type coercion. All operands must be of compatible types:

- Arithmetic operators require numbers
- Comparison operators work with numbers
- Logical operators work with booleans
- Equality operators can compare values of the same type

### Short-Circuit Evaluation

**Note:** Short-circuit evaluation for logical operators (`&`, `|`) is not yet implemented. Currently, both operands are always evaluated.

## Common Pitfalls

### 1. Precedence Confusion

```javascript
// WRONG: Expecting (1 + 2) * 3 = 9
private result = 1 + 2 * 3  // Actually: 1 + (2 * 3) = 7

// CORRECT: Use parentheses
private result = (1 + 2) * 3  // 9
```

### 2. Comparison Chains

```javascript
// WRONG: This doesn't work as expected
private inRange = 1 < x < 10  // Parsed as (1 < x) < 10

// CORRECT: Use logical AND
private inRange = x > 1 & x < 10
```

### 3. AND vs OR Precedence

```javascript
// a | b & c is NOT the same as (a | b) & c
private wrong = a | b & c      // Evaluates as: a | (b & c)
private correct = (a | b) & c  // Explicitly grouped
```

## Best Practices

1. **Use parentheses for clarity** even when not strictly needed
2. **Break complex expressions** into multiple statements
3. **Document tricky precedence** with comments
4. **Test expressions** in isolation to verify behavior

```javascript
// Good: Clear intent
private isValid = (age >= 18) & (hasPermission == true)

// Better: Even clearer with intermediate variables
private isAdult = age >= 18
private hasAccess = hasPermission == true
private isValid = isAdult & hasAccess
```
