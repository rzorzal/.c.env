// Function call parsing

use super::expression_parser;
use crate::grama::error::{ParseError, ParseResult};
use crate::grama::gramma_rules::Expr;
use crate::lexing;

pub(super) fn parse_function_call_with_count(
    tokens: &[lexing::Token],
) -> ParseResult<(Expr, usize)> {
    if tokens.len() < 3 || !matches!(&tokens[1].token_type, lexing::TokenType::LeftParen(_)) {
        let pos = if tokens.is_empty() {
            0
        } else {
            tokens[0].start
        };
        return Err(ParseError::invalid_expression(
            "Expected '(' after function name",
            pos,
        ));
    }

    // Parse the callee - extract function name from various token types
    let callee_name = match &tokens[0].token_type {
        lexing::TokenType::Identifier(name) => name.clone(),
        lexing::TokenType::Print(name) => name.clone(),
        lexing::TokenType::Import(name) => name.clone(),
        lexing::TokenType::Range(name) => name.clone(),
        lexing::TokenType::ImportAwsSecret(name) => name.clone(),
        _ => {
            return Err(ParseError::invalid_expression(
                "Expected identifier or function name",
                tokens[0].start,
            ));
        }
    };

    let callee = Expr::Ident(callee_name);

    let opening_pos = tokens[1].start;
    // Find the closing parenthesis
    let mut paren_count = 1;
    let mut end_idx = 2;

    while end_idx < tokens.len() && paren_count > 0 {
        match &tokens[end_idx].token_type {
            lexing::TokenType::LeftParen(_) => paren_count += 1,
            lexing::TokenType::RightParen(_) => paren_count -= 1,
            _ => {}
        }
        end_idx += 1;
    }

    if paren_count != 0 {
        return Err(ParseError::unmatched_delimiter(
            ')',
            opening_pos,
            Some(opening_pos),
        ));
    }

    // Parse arguments
    let args_tokens = &tokens[2..end_idx - 1];
    let args = if args_tokens.is_empty() {
        Vec::new()
    } else {
        expression_parser::parse_comma_separated_expressions(args_tokens)?
    };

    let expr = Expr::Call {
        callee: Box::new(callee),
        args,
    };

    // Return expression and number of tokens consumed
    // Consumed: function_name + ( + args + ) = end_idx tokens
    Ok((expr, end_idx))
}
