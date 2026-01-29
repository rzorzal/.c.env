use std::fmt;

/// Represents a parse error with location and context information
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// The kind of error that occurred
    pub kind: ParseErrorKind,
    /// The byte position in the source where the error occurred
    pub position: usize,
    /// Optional end position for errors spanning multiple tokens
    pub end_position: Option<usize>,
    /// Additional context or suggestions for fixing the error
    pub context: Option<String>,
}

/// The kind of parse error
#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    /// Unexpected end of input
    UnexpectedEof,
    /// Expected a specific token but found another
    UnexpectedToken {
        expected: String,
        found: String,
    },
    /// Invalid expression syntax
    InvalidExpression(String),
    /// Invalid statement syntax
    InvalidStatement(String),
    /// Unmatched delimiter (parenthesis, bracket, brace)
    UnmatchedDelimiter {
        delimiter: char,
        opening_pos: Option<usize>,
    },
    /// Invalid variable declaration
    InvalidVarDeclaration(String),
    /// Invalid function call
    InvalidFunctionCall(String),
    /// Invalid array comprehension
    InvalidComprehension(String),
    /// Invalid if expression (ternary)
    InvalidIfExpression(String),
    /// Invalid string template
    InvalidTemplate(String),
    /// Empty or invalid input
    EmptyInput,
    /// Generic error with message
    Other(String),
}

impl ParseError {
    /// Create a new parse error
    pub fn new(kind: ParseErrorKind, position: usize) -> Self {
        ParseError {
            kind,
            position,
            end_position: None,
            context: None,
        }
    }

    /// Create a parse error with an end position
    pub fn with_span(kind: ParseErrorKind, start: usize, end: usize) -> Self {
        ParseError {
            kind,
            position: start,
            end_position: Some(end),
            context: None,
        }
    }

    /// Add context information to the error
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    /// Create an unexpected EOF error
    pub fn unexpected_eof(position: usize) -> Self {
        ParseError::new(ParseErrorKind::UnexpectedEof, position)
    }

    /// Create an unexpected token error
    pub fn unexpected_token(expected: &str, found: &str, position: usize) -> Self {
        ParseError::new(
            ParseErrorKind::UnexpectedToken {
                expected: expected.to_string(),
                found: found.to_string(),
            },
            position,
        )
    }

    /// Create an unmatched delimiter error
    pub fn unmatched_delimiter(delimiter: char, position: usize, opening_pos: Option<usize>) -> Self {
        ParseError::new(
            ParseErrorKind::UnmatchedDelimiter {
                delimiter,
                opening_pos,
            },
            position,
        )
    }

    /// Create an invalid expression error
    pub fn invalid_expression(msg: &str, position: usize) -> Self {
        ParseError::new(ParseErrorKind::InvalidExpression(msg.to_string()), position)
    }

    /// Create an invalid statement error
    pub fn invalid_statement(msg: &str, position: usize) -> Self {
        ParseError::new(ParseErrorKind::InvalidStatement(msg.to_string()), position)
    }

    /// Get a line and column number from a byte position in source code
    pub fn get_line_column(source: &str, position: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;

        for (idx, ch) in source.chars().enumerate() {
            if idx >= position {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    /// Format the error with source code context
    pub fn format_with_source(&self, source: &str) -> String {
        let (line, column) = Self::get_line_column(source, self.position);
        let lines: Vec<&str> = source.lines().collect();

        let mut output = String::new();
        output.push_str(&format!("Parse error at line {}, column {}: ", line, column));
        output.push_str(&self.kind.to_string());
        output.push('\n');

        // Show the relevant line of code
        if line > 0 && line <= lines.len() {
            let line_text = lines[line - 1];
            output.push_str(&format!("\n{:4} | {}\n", line, line_text));

            // Add a caret pointing to the error position
            output.push_str("     | ");
            for _ in 0..column.saturating_sub(1) {
                output.push(' ');
            }
            output.push('^');

            if let Some(end_pos) = self.end_position {
                let (end_line, end_column) = Self::get_line_column(source, end_pos);
                if end_line == line && end_column > column {
                    for _ in column..end_column.min(line_text.len()) {
                        output.push('~');
                    }
                }
            }
            output.push('\n');
        }

        // Add context if available
        if let Some(ref ctx) = self.context {
            output.push_str(&format!("\nHelp: {}\n", ctx));
        }

        output
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at position {}: {}", self.position, self.kind)
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::UnexpectedEof => {
                write!(f, "Unexpected end of input")
            }
            ParseErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "Expected {}, but found {}", expected, found)
            }
            ParseErrorKind::InvalidExpression(msg) => {
                write!(f, "Invalid expression: {}", msg)
            }
            ParseErrorKind::InvalidStatement(msg) => {
                write!(f, "Invalid statement: {}", msg)
            }
            ParseErrorKind::UnmatchedDelimiter { delimiter, opening_pos } => {
                if let Some(pos) = opening_pos {
                    write!(f, "Unmatched '{}' (opened at position {})", delimiter, pos)
                } else {
                    write!(f, "Unmatched '{}'", delimiter)
                }
            }
            ParseErrorKind::InvalidVarDeclaration(msg) => {
                write!(f, "Invalid variable declaration: {}", msg)
            }
            ParseErrorKind::InvalidFunctionCall(msg) => {
                write!(f, "Invalid function call: {}", msg)
            }
            ParseErrorKind::InvalidComprehension(msg) => {
                write!(f, "Invalid comprehension: {}", msg)
            }
            ParseErrorKind::InvalidIfExpression(msg) => {
                write!(f, "Invalid if expression: {}", msg)
            }
            ParseErrorKind::InvalidTemplate(msg) => {
                write!(f, "Invalid template: {}", msg)
            }
            ParseErrorKind::EmptyInput => {
                write!(f, "Empty or invalid input")
            }
            ParseErrorKind::Other(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Type alias for parse results
pub type ParseResult<T> = Result<T, ParseError>;
