
mod token;

pub use token::Token as TokenType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
    pub value: Option<String>,
    pub number_value: Option<i32>,
}


impl Token {
    pub fn new(token_type: TokenType, start: usize, end: usize, value: Option<String>, number_value: Option<i32>) -> Self {
        Token {
            token_type,
            start,
            end,
            value,
            number_value,
        }
    }

    pub fn with_value(token_type: TokenType, start: usize, end: usize, value: String) -> Self {
        Token::new(token_type, start, end, Some(value), None)
    }

    pub fn with_integer_value(token_type: TokenType, start: usize, end: usize, value: i32) -> Self {
        Token::new(token_type, start, end, None, Some(value))
    }

    pub fn without_value(token_type: TokenType, start: usize, end: usize) -> Self {
        Token::new(token_type, start, end, None, None)
    }
}

pub fn analyze_code(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut start: usize = 0;

    let chars: Vec<char> = code.chars().collect();

    while start < chars.len() {
        let current_char: char = chars[start];

        // Handle hash comments (# - preserved for .env output)
        if current_char == '#' {
            let comment_start = start;
            let mut end = start + 1;
            while end < chars.len() && chars[end] != '\n' {
                end += 1;
            }
            // Create a Comment token with the full comment text including #
            let comment_text: String = chars[comment_start..end].iter().collect();
            tokens.push(Token::without_value(TokenType::Comment(comment_text), comment_start, end));
            start = end;
            continue;
        }

        // Handle single-line comments (//)
        if current_char == '/' && start + 1 < chars.len() && chars[start + 1] == '/' {
            let mut end = start + 2;
            while end < chars.len() && chars[end] != '\n' {
                end += 1;
            }
            // Skip the comment, don't create a token for it
            start = end;
            continue;
        }

        // Handle multi-line comments (/* */)
        if current_char == '/' && start + 1 < chars.len() && chars[start + 1] == '*' {
            let mut end = start + 2;
            while end + 1 < chars.len() {
                if chars[end] == '*' && chars[end + 1] == '/' {
                    end += 2;
                    break;
                }
                end += 1;
            }
            // Skip the comment, don't create a token for it
            start = end;
            continue;
        }

        if current_char.is_whitespace() {
            if current_char == '\n' {
                tokens.push(Token::without_value(TokenType::Eol("\n".to_string()), start, start + 1));
            }
            start += 1;
            continue;
        }

        // Example: Handle identifiers and keywords
        if current_char.is_alphabetic() || current_char == '_' {
            let mut end: usize = start + 1;
            while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                end += 1;
            }
            let identifier = &code[start..end];
            let token_type = match identifier {
                "print" => TokenType::Print(identifier.to_string()),
                "if" => TokenType::If(identifier.to_string()),
                "else" => TokenType::Else(identifier.to_string()),
                "for" => TokenType::For(identifier.to_string()),
                "in" => TokenType::In(identifier.to_string()),
                "of" => TokenType::Of(identifier.to_string()),
                "import" => TokenType::Import(identifier.to_string()),
                "range" => TokenType::Range(identifier.to_string()),
                "import_aws_secret" => TokenType::ImportAwsSecret(identifier.to_string()),
                "private" => TokenType::Private(identifier.to_string()),
                "break" => TokenType::Break(identifier.to_string()),
                "true" => TokenType::TrueLiteral(identifier.to_string()),
                "false" => TokenType::FalseLiteral(identifier.to_string()),
                _ => TokenType::Identifier(identifier.to_string()),
            };
            tokens.push(Token::with_value(token_type, start, end, identifier.to_string()));
            start = end;
            continue;
        }

        // Example: Handle integer literals
        if current_char.is_ascii_digit() {
            let mut end: usize = start + 1;
            while end < chars.len() && chars[end].is_ascii_digit() {
                end += 1;
            }
            let integer_literal: i32 = code[start..end].parse().unwrap();
            tokens.push(Token::with_integer_value(TokenType::IntegerLiteral(integer_literal), start, end, integer_literal));
            start = end;
            continue;
        }

        if current_char == '"' {
            let mut end: usize = start + 1;
            while end < chars.len() && chars[end] != '"' {
                end += 1;
            }
            if end < chars.len() {
                end += 1; // Include the closing quote
                let string_literal: String = code[start + 1..end - 1].to_string();
                tokens.push(Token::with_value(TokenType::StringLiteral(string_literal.clone()), start, end, string_literal));
            }
            start = end;
            continue;
        }

        if current_char == '`' {
            let mut end: usize = start + 1;
            while end < chars.len() && chars[end] != '`' {
                end += 1;
            }
            if end < chars.len() {
                end += 1; // Include the closing quote
                let string_literal: String = code[start + 1..end - 1].to_string();
                tokens.push(Token::with_value(TokenType::StringTemplate(string_literal.clone()), start, end, string_literal));
            }
            start = end;
            continue;
        }

        if current_char == '(' {
            tokens.push(Token::without_value(TokenType::LeftParen("(".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == ')' {
            tokens.push(Token::without_value(TokenType::RightParen(")".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '{' {
            tokens.push(Token::without_value(TokenType::LeftBrace("{".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '}' {
            tokens.push(Token::without_value(TokenType::RightBrace("}".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '[' {
            tokens.push(Token::without_value(TokenType::LeftBracket("[".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == ']' {
            tokens.push(Token::without_value(TokenType::RightBracket("]".to_string()), start, start + 1));
            start += 1;
            continue;
        }
        if current_char == ',' {
            tokens.push(Token::without_value(TokenType::Comma(",".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '+' {
            tokens.push(Token::without_value(TokenType::Plus("+".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '-' {
            tokens.push(Token::without_value(TokenType::Minus("-".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '*' {
            tokens.push(Token::without_value(TokenType::Multiply("*".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '/' {
            // Division operator (comments are handled earlier)
            tokens.push(Token::without_value(TokenType::Divider("/".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '%' {
            tokens.push(Token::without_value(TokenType::Mod("%".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '=' {
            if start + 1 < chars.len() && chars[start + 1] == '=' {
                tokens.push(Token::without_value(TokenType::Equal("==".to_string()), start, start + 2));
                start += 2;
            } else {
                tokens.push(Token::without_value(TokenType::Assign("=".to_string()), start, start + 1));
                start += 1;
            }
            continue;
        }

        if current_char == '>' {
            if start + 1 < chars.len() && chars[start + 1] == '=' {
                tokens.push(Token::without_value(TokenType::GreaterThanOrEqual(">=".to_string()), start, start + 2));
                start += 2;
            } else {
                tokens.push(Token::without_value(TokenType::GreaterThan(">".to_string()), start, start + 1));
                start += 1;
            }
            continue;
        }

        if current_char == '<' {
            if start + 1 < chars.len() && chars[start + 1] == '=' {
                tokens.push(Token::without_value(TokenType::LessThanOrEqual("<=".to_string()), start, start + 2));
                start += 2;
            } else {
                tokens.push(Token::without_value(TokenType::LessThan("<".to_string()), start, start + 1));
                start += 1;
            }
            continue;
        }

        if current_char == '!' {
            if start + 1 < chars.len() && chars[start + 1] == '=' {
                tokens.push(Token::without_value(TokenType::NotEqual("!=".to_string()), start, start + 2));
                start += 2; // Move past the '!='
            } else {
                tokens.push(Token::without_value(TokenType::Not("!".to_string()), start, start + 1));
                start += 1;
            }
            continue;
        }

        if current_char == '&' {
            tokens.push(Token::without_value(TokenType::And("&".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '|' {
            tokens.push(Token::without_value(TokenType::Or("|".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '?' {
            tokens.push(Token::without_value(TokenType::QuestionMark("?".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        if current_char == '.' {
            tokens.push(Token::without_value(TokenType::Dot(".".to_string()), start, start + 1));
            start += 1;
            continue;
        }

        panic!("Unexpected character: {} at {}-{}", current_char, start, start + 1);
    }

    tokens
}
