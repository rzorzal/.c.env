# Loop and Range Implementation Plan

## 1. Define Syntax

- Support both `for (var in iterable) { ... }` and `for (var of iterable) { ... }` forms.
- Allow list comprehensions: `[expr for var of iterable]` and `[expr if cond for var of iterable]`.
- Support variable interpolation in assignments (e.g., `MY_NUMBERS_${number} = number`).

## 2. Update the Parser

- Extend the parser to recognize:
  - `for` loops with `in` and `of`
  - List comprehensions with optional `if` filters
  - Variable interpolation in assignment targets
- Parse the `range()` function as an expression.

## 3. Implement the `range` Function

- Add a built-in `range(start, end, step=1)` function.
- Return a list/array of numbers, e.g., `range(10)` → `[0,1,2,3,4,5,6,7,8,9]`.
- Support single-argument (end), two-argument (start, end), and optional step.

## 4. Implement Loop Evaluation

- For `for (var in iterable) { ... }`, iterate over indices.
- For `for (var of iterable) { ... }`, iterate over values.
- Bind the loop variable in the current scope for each iteration.
- Support `break` and `continue` (if needed).

## 5. Implement List Comprehensions

- Parse and evaluate comprehensions, supporting optional `if` filters.
- Evaluate the expression for each item, collecting results into a new list.

## 6. Variable Interpolation

- Implement logic to resolve `${var}` in assignment targets.

## 7. Error Handling

- Type-check `range` arguments.
- Handle invalid loop constructs and comprehensions gracefully.

## 8. Testing

- Add tests for:
  - `range` with various arguments
  - `for` loops (in/of)
  - List comprehensions (with/without `if`)
  - Variable interpolation in assignments

---

## Notes (When to Implement)

- Your example shows both index-based and value-based iteration—support both early, as they are core to your language's ergonomics.
- Comprehensions and variable interpolation are powerful features; implement them after basic loop/range support is stable.
- Document the new features in your language reference for users as soon as the syntax is finalized.
- Consider how scoping works for variables declared inside loops—clarify this before implementation to avoid confusion.

---

## Questions for Future Implementation

- Should `range` be inclusive or exclusive of the end value? (e.g., `range(0, 5)` → `[0,1,2,3,4]` or `[0,1,2,3,4,5]`?)
  - It can have a new argument to decide it
- Should negative steps be supported in `range`? (e.g., `range(5, 0, -1)`)
  - Yes, it will be good.
- How should `break` and `continue` be handled in comprehensions?
  - We don't need to support those keywords in comprehensions.
- Should variable interpolation support expressions, or only identifiers? (e.g., `${number+1}`)
  - Just variables for now
- What should happen if a loop variable shadows an existing variable?
  - The value will be overwritten
- Should comprehensions support nested loops?
  - Not, comprehensions is something for just one line, so just inline for's are supported, if needed nested use the other way of loops
- How should errors in the loop body or comprehension be reported?
  - We need to show the user the file, index or value, line and col the error appear. Remember to add more info if needed.
- Can we handle ${value} = "test" ?
  - No, the notation ${} is only valid inside loops body as a suffix or prefix for a variable name.
