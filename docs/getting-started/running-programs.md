# Running C.env Programs

This guide explains how to run C.env programs and interpret the output.

## Basic Execution

### Command Syntax

```bash
c_env_lang <filename>
```

Or with the development build:

```bash
./target/debug/c_env_lang <filename>
```

### Example

```bash
c_env_lang examples/hello.c.env
```

## Understanding Output

The C.env compiler produces detailed output showing each stage of processing:

### 1. File Information

```
Filename: hello.c.env
File contents:
private greeting = "Hello!"
```

Shows the input file name and contents.

### 2. Lexical Analysis (Tokens)

```
Token { token_type: Private("private"), start: 0, end: 7, ... }
Token { token_type: Identifier("greeting"), start: 8, end: 16, ... }
Token { token_type: Assign("="), start: 17, end: 18, ... }
Token { token_type: StringLiteral("Hello!"), start: 19, end: 27, ... }
```

Shows how the source code is broken into tokens. Each token includes:

- **token_type**: The kind of token (keyword, identifier, operator, etc.)
- **start/end**: Position in the source code
- **value**: The actual text (for identifiers and strings)

### 3. Parsed AST (Abstract Syntax Tree)

```
Program: Program {
    items: [
        VarDecl {
            private_: true,
            name: "greeting",
            value: StringLiteral("Hello!"),
        },
    ],
}
```

Shows the parsed program structure ready for interpretation/compilation.

## Exit Codes

- **0**: Success - Program parsed without errors
- **1**: Parse error - Syntax or semantic error in the code

## Error Messages

When your code has errors, C.env provides helpful error messages:

### Example: Missing Closing Parenthesis

Input (`error.c.env`):

```javascript
private x = (1 + 2
```

Output:

```
Parse Error: Unmatched delimiter ')'
  at position 12

  private x = (1 + 2
              ^

Expected closing parenthesis to match opening '(' at position 12
```

The error message shows:

- **Error type**: What went wrong
- **Location**: Where in the code
- **Context**: The line with a caret (^) pointing to the problem
- **Help**: What was expected

### Example: Missing Closing Bracket

Input:

```javascript
private arr = [1, 2, 3
```

Output:

```
Parse Error: Unmatched delimiter ']'
  at position 14

  private arr = [1, 2, 3
                ^

Expected closing bracket to match opening '[' at position 14
```

## Working with Examples

The repository includes example files in the `examples/` directory:

```bash
# Run a simple example
c_env_lang examples/.c.env.hello

# Run operator precedence examples
c_env_lang examples/.c.env.precedence_test

# Run comment examples
c_env_lang examples/.c.env.comment_edge_cases
```

## Redirecting Output

### Save Output to File

```bash
c_env_lang myfile.c.env > output.txt
```

### Separate Standard Output and Errors

```bash
c_env_lang myfile.c.env > output.txt 2> errors.txt
```

### Silent Mode (Errors Only)

```bash
c_env_lang myfile.c.env 2>&1 | grep -E "(Error|Warning)"
```

## Development Workflow

### 1. Write Code

Create or edit a `.c.env` file:

```javascript
// config.c.env
private appName = "MyApp"
private version = 1.0
private debug = true
```

### 2. Run and Test

```bash
c_env_lang config.c.env
```

### 3. Check for Errors

Review the output for parse errors or unexpected results.

### 4. Iterate

Make changes and run again.

## Using with Scripts

### Shell Script Integration

```bash
#!/bin/bash

CONFIG_FILE="config.c.env"

if c_env_lang "$CONFIG_FILE" 2>/dev/null; then
    echo "Configuration valid!"
else
    echo "Configuration has errors!"
    exit 1
fi
```

### Makefile Integration

```makefile
.PHONY: validate
validate:
	@c_env_lang config.c.env > /dev/null

.PHONY: show
show:
	@c_env_lang config.c.env
```

## Performance Tips

### Large Files

For large configuration files:

- The lexer and parser are fast, but output can be verbose
- Redirect output to a file for later analysis
- Use grep to filter specific information

### Batch Processing

Process multiple files:

```bash
for file in configs/*.c.env; do
    echo "Processing $file..."
    c_env_lang "$file" || echo "Error in $file"
done
```

## Debugging Tips

### 1. Check Token Output

Look at the token stream to verify the lexer is working correctly:

```bash
c_env_lang file.c.env 2>&1 | grep "Token {" | head -20
```

### 2. Check AST Structure

Look at the parsed AST to verify the structure:

```bash
c_env_lang file.c.env 2>&1 | grep -A 50 "Program: Program"
```

### 3. Validate Specific Expressions

Test individual expressions in isolation:

```javascript
// test_expr.c.env
private test = 2 * 3 + 4
```

### 4. Use Comments for Debugging

Temporarily comment out code to isolate issues:

```javascript
private a = 10
// private b = 20
private c = 30
```

## Next Steps

- Learn about [Error Handling](../language-reference/errors.md)
- See [Common Examples](../examples/README.md)
- Read the [Language Reference](../language-reference/README.md)
