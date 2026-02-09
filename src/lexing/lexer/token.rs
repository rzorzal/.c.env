#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Keywords
    Print(String),
    If(String),
    Else(String),
    Private(String),
    For(String),
    In(String),
    Of(String),
    Import(String),
    Range(String),
    ImportAwsSecret(String),
    Break(String),

    // Literals
    IntegerLiteral(i32),
    StringLiteral(String),
    StringTemplate(String),
    TrueLiteral(String),
    FalseLiteral(String),

    // Identifiers
    Identifier(String), // =

    // Operators
    Pow(String),      // ^
    Multiply(String), // *
    Divider(String),  // /
    Mod(String),      // %
    Minus(String),    // -
    Plus(String),     // +
    Assign(String),   // =

    // Punctuation
    LeftParen(String),    // (
    RightParen(String),   // )
    LeftBrace(String),    // {
    RightBrace(String),   // }
    LeftBracket(String),  // [
    RightBracket(String), // ]
    Comma(String),        // ,
    Dot(String),          // .
    QuestionDot(String),  // ?.
    Dollar(String),       // $
    Backtick(String),     // Backtick for template literals ``
    Quote(String),        // Single quote
    DoubleQuote(String),  // Double quote
    Slash(String),        // /
    BackSlash(String),    // \
    Colon(String),        // :
    Semicolon(String),    // ;
    QuestionMark(String), // ?

    // Logical Operators
    GreaterThan(String),        // >
    LessThan(String),           // <
    GreaterThanOrEqual(String), // >=
    LessThanOrEqual(String),    // <=
    Equal(String),              // ==
    NotEqual(String),           // !=

    // Logical Expressions
    And(String), // &
    Or(String),  // |
    Not(String), // !

    // Blank
    Blank(String),

    // EOF
    Eof(String),

    // EOL
    Eol(String),

    // Comment (only # comments are preserved for .env output)
    Comment(String),
}
