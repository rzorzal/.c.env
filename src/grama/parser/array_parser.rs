// Array parsing - array literals and array comprehensions

use super::expression_parser;
use crate::grama::error::{ParseError, ParseResult};
use crate::grama::gramma_rules::{ArrayComp, Expr, IterMode, TemplatePart};
use crate::lexing;

pub(super) fn parse_array_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
    // First token must be left bracket
    if tokens.is_empty() || !matches!(&tokens[0].token_type, lexing::TokenType::LeftBracket(_)) {
        let pos = if tokens.is_empty() {
            0
        } else {
            tokens[0].start
        };
        return Err(ParseError::invalid_expression("Expected '['", pos));
    }

    let opening_pos = tokens[0].start;
    // Find the matching right bracket
    let mut bracket_count = 1;
    let mut end_idx = 1;

    while end_idx < tokens.len() && bracket_count > 0 {
        match &tokens[end_idx].token_type {
            lexing::TokenType::LeftBracket(_) => bracket_count += 1,
            lexing::TokenType::RightBracket(_) => bracket_count -= 1,
            _ => {}
        }
        end_idx += 1;
    }

    if bracket_count != 0 {
        return Err(ParseError::unmatched_delimiter(
            ']',
            opening_pos,
            Some(opening_pos),
        ));
    }

    // Check if this is an array comprehension
    let inner_tokens = &tokens[1..end_idx - 1];
    if let Ok(array_comp) = parse_array_comprehension(inner_tokens) {
        return Ok(array_comp);
    }

    // Parse array elements
    let elements = expression_parser::parse_comma_separated_expressions(inner_tokens)?;

    // Convert to array expression
    // Note: The Expr enum doesn't have a dedicated Array variant in your grammar,
    // but we can create a template with elements as parts
    let template_parts: Vec<_> = elements.into_iter().map(TemplatePart::Expr).collect();

    Ok(Expr::Template(template_parts))
}

pub(super) fn parse_array_comprehension(tokens: &[lexing::Token]) -> ParseResult<Expr> {
    // Array comprehension pattern: expr for var of/in iter [if condition]

    // Find the 'for' keyword
    let mut for_idx = 0;
    while for_idx < tokens.len() {
        if let lexing::TokenType::For(_) = &tokens[for_idx].token_type {
            break;
        }
        for_idx += 1;
    }

    if for_idx >= tokens.len() || for_idx == 0 {
        let pos = if tokens.is_empty() {
            0
        } else {
            tokens[0].start
        };
        return Err(ParseError::invalid_expression(
            "Array comprehension requires 'for' keyword",
            pos,
        ));
    }

    // Parse the expression to include in the array
    let expr_tokens = &tokens[0..for_idx];
    let expr = expression_parser::parse_expression(expr_tokens)?;

    // Parse variable name
    if for_idx + 1 >= tokens.len() {
        return Err(ParseError::invalid_expression(
            "Expected variable name after 'for'",
            tokens[for_idx].end,
        ));
    }

    let var_token = &tokens[for_idx + 1];
    let var = match &var_token.token_type {
        lexing::TokenType::Identifier(name) => name.clone(),
        _ => {
            return Err(ParseError::unexpected_token(
                "variable name",
                &format!("{:?}", var_token.token_type),
                var_token.start,
            ));
        }
    };

    // Check for 'of' or 'in'
    if for_idx + 2 >= tokens.len() {
        return Err(ParseError::invalid_expression(
            "Expected 'of' or 'in' after variable name",
            tokens[for_idx + 1].end,
        ));
    }

    let mode_token = &tokens[for_idx + 2];
    let mode = match &mode_token.token_type {
        lexing::TokenType::Of(_) => IterMode::Of,
        lexing::TokenType::In(_) => IterMode::In,
        _ => {
            return Err(ParseError::unexpected_token(
                "'of' or 'in'",
                &format!("{:?}", mode_token.token_type),
                mode_token.start,
            ));
        }
    };

    // Parse iterator expression
    if for_idx + 3 >= tokens.len() {
        return Err(ParseError::invalid_expression(
            "Expected iterator expression",
            tokens[for_idx + 2].end,
        ));
    }

    let mut iter_end_idx = for_idx + 3;
    while iter_end_idx < tokens.len() {
        if let lexing::TokenType::If(_) = &tokens[iter_end_idx].token_type {
            break;
        }
        iter_end_idx += 1;
    }

    let iter_tokens = &tokens[(for_idx + 3)..iter_end_idx];
    let iter = expression_parser::parse_expression(iter_tokens)?;

    // Parse optional filter condition
    let filter = if iter_end_idx < tokens.len()
        && matches!(&tokens[iter_end_idx].token_type, lexing::TokenType::If(_))
    {
        let filter_tokens = &tokens[(iter_end_idx + 1)..];
        Some(Box::new(expression_parser::parse_expression(
            filter_tokens,
        )?))
    } else {
        None
    };

    // Create array comprehension
    Ok(Expr::ArrayComp(ArrayComp {
        expr: Box::new(expr),
        var,
        mode,
        iter: Box::new(iter),
        filter,
    }))
}
