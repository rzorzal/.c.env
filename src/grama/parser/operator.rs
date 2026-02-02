// Operator handling - precedence and binary expression parsing

use super::expression_parser;
use crate::grama::error::{ParseError, ParseResult};
use crate::grama::gramma_rules::{BinOp, Expr};
use crate::lexing;

/// Get the precedence level for a binary operator
/// Higher numbers = higher precedence (binds tighter)
///
/// Precedence levels:
/// - Level 0: Or `|`
/// - Level 1: And `&`
/// - Level 2: Equality `==`, `!=`
/// - Level 3: Comparison `<`, `>`, `<=`, `>=`
/// - Level 4: Addition/Subtraction `+`, `-`
/// - Level 5: Multiplication/Division/Modulo `*`, `/`, `%`
pub(super) fn get_operator_precedence(token_type: &lexing::TokenType) -> Option<(BinOp, u8)> {
    match token_type {
        // Level 0: Logical OR (lowest precedence)
        lexing::TokenType::Or(_) => Some((BinOp::Or, 0)),

        // Level 1: Logical AND
        lexing::TokenType::And(_) => Some((BinOp::And, 1)),

        // Level 2: Equality operators
        lexing::TokenType::Equal(_) => Some((BinOp::Eq, 2)),
        lexing::TokenType::NotEqual(_) => Some((BinOp::Ne, 2)),

        // Level 3: Comparison operators
        lexing::TokenType::LessThan(_) => Some((BinOp::Lt, 3)),
        lexing::TokenType::LessThanOrEqual(_) => Some((BinOp::Le, 3)),
        lexing::TokenType::GreaterThan(_) => Some((BinOp::Gt, 3)),
        lexing::TokenType::GreaterThanOrEqual(_) => Some((BinOp::Ge, 3)),

        // Level 4: Addition and Subtraction
        lexing::TokenType::Plus(_) => Some((BinOp::Add, 4)),
        lexing::TokenType::Minus(_) => Some((BinOp::Sub, 4)),

        // Level 5: Multiplication, Division, and Modulo (highest precedence)
        lexing::TokenType::Multiply(_) => Some((BinOp::Mul, 5)),
        lexing::TokenType::Divider(_) => Some((BinOp::Div, 5)),
        lexing::TokenType::Mod(_) => Some((BinOp::Mod, 5)),

        // Not a binary operator
        _ => None,
    }
}

pub fn parse_binary_expression(tokens: &[lexing::Token], min_precedence: u8) -> ParseResult<Expr> {
    let (expr, _) = parse_binary_expression_impl(tokens, 0, min_precedence)?;
    Ok(expr)
}

fn parse_binary_expression_impl(
    tokens: &[lexing::Token],
    mut pos: usize,
    min_precedence: u8,
) -> ParseResult<(Expr, usize)> {
    if pos >= tokens.len() {
        return Err(ParseError::invalid_expression(
            "Empty expression in binary operation",
            0,
        ));
    }

    // First parse the left-hand side as a primary expression
    let (mut lhs, tokens_consumed) =
        expression_parser::parse_primary_expression_with_count(&tokens[pos..])?;
    pos += tokens_consumed;

    while pos < tokens.len() {
        // Lookahead to find a binary operator
        let op_token = &tokens[pos];

        // Get operator and its precedence
        let (op, precedence) = match get_operator_precedence(&op_token.token_type) {
            Some((op, prec)) => (op, prec),
            None => break, // Not a binary operator, stop parsing
        };

        // If this operator's precedence is too low, break
        if precedence < min_precedence {
            break;
        }

        // Move past the operator
        pos += 1;
        if pos >= tokens.len() {
            return Err(ParseError::invalid_expression(
                "Expected expression after operator",
                op_token.start,
            ));
        }

        // Recursively parse the right-hand side with higher precedence
        let (rhs, new_pos) = parse_binary_expression_impl(tokens, pos, precedence + 1)?;
        pos = new_pos;

        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        };
    }

    Ok((lhs, pos))
}
