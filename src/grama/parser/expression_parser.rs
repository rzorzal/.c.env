// Expression parsing - coordinates parsing of all expression types

use crate::lexing;
use crate::grama::gramma_rules::{Expr, UnaryOp};
use crate::grama::error::{ParseError, ParseResult};
use super::{operator, array_parser, function_parser, control_flow_parser, literal_parser};

/// Parse an expression from tokens
pub(super) fn parse_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  if tokens.is_empty() {
    return Err(ParseError::invalid_expression("Empty expression", 0));
  }

  // Try to parse binary expressions first
  if let Ok(expr) = operator::parse_binary_expression(tokens, 0) {
    return Ok(expr);
  }

  // If not a binary expression, try to parse other expressions
  parse_primary_expression(tokens)
}

pub(super) fn parse_primary_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  let (expr, _) = parse_primary_expression_with_count(tokens)?;
  Ok(expr)
}

/// Parse a primary expression and return the expression plus the number of tokens consumed
pub(super) fn parse_primary_expression_with_count(tokens: &[lexing::Token]) -> ParseResult<(Expr, usize)> {
  if tokens.is_empty() {
    return Err(ParseError::invalid_expression("Empty expression", 0));
  }

  // First try to parse if expressions (ternary operator)
  if let Ok(if_expr) = control_flow_parser::parse_if_expression(tokens) {
    // If expressions consume all tokens they need, but we don't have an easy way
    // to know how many. For now, return 1 (this is a limitation)
    return Ok((if_expr, 1));
  }

  // Then try to parse find comprehensions
  if let Ok(find_expr) = control_flow_parser::parse_find_comprehension(tokens) {
    return Ok((find_expr, 1));
  }

  let token_pos = tokens[0].start;

  match &tokens[0].token_type {
    // Literals - all consume exactly 1 token
    lexing::TokenType::IntegerLiteral(num) => {
      Ok((Expr::Number(*num as f64), 1))
    },
    lexing::TokenType::StringLiteral(value) => {
      Ok((Expr::StringLiteral(value.clone()), 1))
    },
    lexing::TokenType::StringTemplate(_) => {
      // String templates consume 1 token
      let expr = literal_parser::parse_string_template(tokens)?;
      Ok((expr, 1))
    },
    lexing::TokenType::TrueLiteral(_) => {
      Ok((Expr::Bool(true), 1))
    },
    lexing::TokenType::FalseLiteral(_) => {
      Ok((Expr::Bool(false), 1))
    },
    _ if tokens.len() == 1 && matches!(&tokens[0].token_type, lexing::TokenType::Identifier(id) if id.to_lowercase() == "null") => {
      Ok((Expr::Null, 1))
    },

    // Identifiers
    lexing::TokenType::Identifier(value) => {
      // Check if this is a function call
      if tokens.len() > 1 && matches!(&tokens[1].token_type, lexing::TokenType::LeftParen(_)) {
        return function_parser::parse_function_call_with_count(tokens);
      }

      // Check if this is an array/object access
      if tokens.len() > 1 && matches!(&tokens[1].token_type, lexing::TokenType::LeftBracket(_)) {
        let expr = parse_index_access(tokens)?;
        return Ok((expr, 1));
      }

      // Check if this is a property access using dot notation
      if tokens.len() > 2 && matches!(&tokens[1].token_type, lexing::TokenType::Dot(_)) {
        if let lexing::TokenType::Identifier(prop_name) = &tokens[2].token_type {
          let object = Expr::Ident(value.clone());
          return Ok((Expr::Member {
            object: Box::new(object),
            field: prop_name.clone(),
          }, 3)); // Consumed: identifier, dot, property
        }
      }

      // Regular identifier - consumes 1 token
      Ok((Expr::Ident(value.clone()), 1))
    },

    // Keywords that can be function calls (print, type, len, etc.)
    lexing::TokenType::Print(_) |
    lexing::TokenType::Import(_) |
    lexing::TokenType::Range(_) |
    lexing::TokenType::ImportAwsSecret(_) => {
      // Check if this is a function call
      if tokens.len() > 1 && matches!(&tokens[1].token_type, lexing::TokenType::LeftParen(_)) {
        return function_parser::parse_function_call_with_count(tokens);
      }
      // Keywords without parentheses are errors
      Err(ParseError::unexpected_token(
        "expression",
        &format!("{:?}", tokens[0].token_type),
        tokens[0].start
      ))
    },

    // Parenthesized expressions
    lexing::TokenType::LeftParen(_) => {
      parse_parenthesized_expression(tokens)
    },

    // Unary expressions
    lexing::TokenType::Plus(_) | lexing::TokenType::Minus(_) => {
      parse_unary_expression(tokens)
    },

    // Array literals
    lexing::TokenType::LeftBracket(_) => {
      let expr = array_parser::parse_array_expression(tokens)?;
      // Array expressions consume tokens - for now approximate
      Ok((expr, 1))
    },

    _ => {
      Err(ParseError::unexpected_token(
        "expression",
        &format!("{:?}", tokens[0].token_type),
        token_pos
      ))
    }
  }
}

fn parse_parenthesized_expression(tokens: &[lexing::Token]) -> ParseResult<(Expr, usize)> {
  // Find the matching right parenthesis
  let mut paren_count = 1;
  let mut end_idx = 1;
  let opening_pos = tokens[0].start;

  while end_idx < tokens.len() && paren_count > 0 {
    match &tokens[end_idx].token_type {
      lexing::TokenType::LeftParen(_) => paren_count += 1,
      lexing::TokenType::RightParen(_) => paren_count -= 1,
      _ => {}
    }
    end_idx += 1;
  }

  if paren_count != 0 {
    return Err(ParseError::unmatched_delimiter(')', opening_pos, Some(opening_pos)));
  }

  if end_idx > 1 {
    // Parse the expression inside the parentheses
    let expr = parse_expression(&tokens[1..end_idx-1])?;
    // Consumed: opening paren + inner tokens + closing paren = end_idx tokens
    return Ok((expr, end_idx));
  }

  Err(ParseError::invalid_expression("Empty parentheses", opening_pos))
}

fn parse_unary_expression(tokens: &[lexing::Token]) -> ParseResult<(Expr, usize)> {
  let token_pos = tokens[0].start;
  let op = match &tokens[0].token_type {
    lexing::TokenType::Plus(_) => UnaryOp::Plus,
    lexing::TokenType::Minus(_) => UnaryOp::Minus,
    _ => unreachable!(),
  };

  if tokens.len() <= 1 {
    return Err(ParseError::invalid_expression(
      "Expected expression after unary operator",
      token_pos
    ));
  }

  let expr = parse_expression(&tokens[1..])?;
  // Consumed: operator + all tokens for the expression
  // This is approximate - ideally we'd know exactly how many
  Ok((Expr::Unary { op, rhs: Box::new(expr) }, tokens.len()))
}

fn parse_index_access(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  // Array/object index access pattern: target[index]
  if tokens.len() < 3 {
    let pos = if tokens.is_empty() { 0 } else { tokens[0].start };
    return Err(ParseError::invalid_expression("Index access requires target[index] syntax", pos));
  }

  // Parse the target expression (usually an identifier)
  let target = match &tokens[0].token_type {
    lexing::TokenType::Identifier(name) => Expr::Ident(name.clone()),
    _ => return Err(ParseError::unexpected_token(
      "identifier",
      &format!("{:?}", tokens[0].token_type),
      tokens[0].start
    )),
  };

  // Ensure the second token is a left bracket
  if !matches!(&tokens[1].token_type, lexing::TokenType::LeftBracket(_)) {
    return Err(ParseError::unexpected_token(
      "'['",
      &format!("{:?}", tokens[1].token_type),
      tokens[1].start
    ));
  }

  let opening_pos = tokens[1].start;
  // Find the matching right bracket
  let mut bracket_count = 1;
  let mut end_idx = 2;

  while end_idx < tokens.len() && bracket_count > 0 {
    match &tokens[end_idx].token_type {
      lexing::TokenType::LeftBracket(_) => bracket_count += 1,
      lexing::TokenType::RightBracket(_) => bracket_count -= 1,
      _ => {}
    }
    end_idx += 1;
  }

  if bracket_count != 0 {
    return Err(ParseError::unmatched_delimiter(']', opening_pos, Some(opening_pos)));
  }

  if end_idx <= 3 {
    return Err(ParseError::invalid_expression("Empty index brackets", opening_pos));
  }

  // Parse the index expression
  let index_tokens = &tokens[2..end_idx-1];
  let index = parse_expression(index_tokens)?;

  // Create the index access expression
  Ok(Expr::Index {
    target: Box::new(target),
    index: Box::new(index),
  })
}

/// Parse comma-separated expressions (used by function calls and arrays)
pub(super) fn parse_comma_separated_expressions(tokens: &[lexing::Token]) -> ParseResult<Vec<Expr>> {
  if tokens.is_empty() {
    return Ok(Vec::new());
  }

  let mut expressions = Vec::new();
  let mut start_idx = 0;
  let mut paren_count: usize = 0;
  let mut bracket_count: usize = 0;
  let mut brace_count: usize = 0;

  for i in 0..=tokens.len() {
    if i == tokens.len() ||
       (i < tokens.len() && matches!(&tokens[i].token_type, lexing::TokenType::Comma(_)) &&
        paren_count == 0 && bracket_count == 0 && brace_count == 0) {

      if i > start_idx {
        let expr = parse_expression(&tokens[start_idx..i])?;
        expressions.push(expr);
      }

      start_idx = i + 1;
      continue;
    }

    if i < tokens.len() {
      match &tokens[i].token_type {
        lexing::TokenType::LeftParen(_) => paren_count += 1,
        lexing::TokenType::RightParen(_) => paren_count = paren_count.saturating_sub(1),
        lexing::TokenType::LeftBracket(_) => bracket_count += 1,
        lexing::TokenType::RightBracket(_) => bracket_count = bracket_count.saturating_sub(1),
        lexing::TokenType::LeftBrace(_) => brace_count += 1,
        lexing::TokenType::RightBrace(_) => brace_count = brace_count.saturating_sub(1),
        _ => {}
      }
    }
  }

  Ok(expressions)
}
