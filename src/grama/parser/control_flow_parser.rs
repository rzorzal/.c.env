// Control flow parsing - if expressions and find comprehensions

use crate::lexing;
use crate::grama::gramma_rules::{Expr, FindComp};
use crate::grama::error::{ParseError, ParseResult};
use super::{expression_parser, array_parser};

pub(super) fn parse_if_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  // If expression pattern: if <cond> ? <then_expr> else <else_expr>

  // First find the 'if' keyword
  if tokens.is_empty() || !matches!(&tokens[0].token_type, lexing::TokenType::If(_)) {
    let pos = if tokens.is_empty() { 0 } else { tokens[0].start };
    return Err(ParseError::invalid_expression("Expected 'if' keyword", pos));
  }

  // Find the '?' token
  let mut question_idx = 1;
  let mut paren_count: usize = 0;

  while question_idx < tokens.len() {
    match &tokens[question_idx].token_type {
      lexing::TokenType::LeftParen(_) => paren_count += 1,
      lexing::TokenType::RightParen(_) => {
        paren_count = paren_count.saturating_sub(1);
      },
      lexing::TokenType::QuestionMark(_) if paren_count == 0 => break,
      _ => {}
    }
    question_idx += 1;
  }

  if question_idx >= tokens.len() {
    return Err(ParseError::invalid_expression(
      "If expression requires '?' after condition",
      tokens[0].end
    ));
  }

  // Parse the condition
  let cond_tokens = &tokens[1..question_idx];
  let cond = expression_parser::parse_expression(cond_tokens)?;

  // Find the 'else' keyword
  let mut else_idx = question_idx + 1;
  paren_count = 0;

  while else_idx < tokens.len() {
    match &tokens[else_idx].token_type {
      lexing::TokenType::LeftParen(_) => paren_count += 1,
      lexing::TokenType::RightParen(_) => {
        paren_count = paren_count.saturating_sub(1);
      },
      lexing::TokenType::Else(_) if paren_count == 0 => break,
      _ => {}
    }
    else_idx += 1;
  }

  if else_idx >= tokens.len() {
    return Err(ParseError::invalid_expression(
      "If expression requires 'else' branch",
      tokens[question_idx].end
    ));
  }

  // Parse the then expression
  let then_tokens = &tokens[(question_idx + 1)..else_idx];
  let then_expr = expression_parser::parse_expression(then_tokens)?;

  // Parse the else expression
  let else_tokens = &tokens[(else_idx + 1)..];
  let else_expr = expression_parser::parse_expression(else_tokens)?;

  Ok(Expr::IfExpr {
    cond: Box::new(cond),
    then_: Box::new(then_expr),
    else_: Box::new(else_expr),
  })
}

pub(super) fn parse_find_comprehension(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  // Find comprehension pattern: expr & break for var of/in iter [if condition]

  // First find the '&' token and then 'break'
  let mut and_idx = 0;
  while and_idx < tokens.len() {
    if let lexing::TokenType::And(_) = &tokens[and_idx].token_type {
      break;
    }
    and_idx += 1;
  }

  if and_idx >= tokens.len() || and_idx == 0 {
    let pos = if tokens.is_empty() { 0 } else { tokens[0].start };
    return Err(ParseError::invalid_expression(
      "Find comprehension requires '& break' pattern",
      pos
    ));
  }

  // Check if the next token is 'break'
  if and_idx + 1 >= tokens.len() {
    return Err(ParseError::invalid_expression(
      "Expected 'break' after '&'",
      tokens[and_idx].end
    ));
  }

  if !matches!(&tokens[and_idx + 1].token_type, lexing::TokenType::Break(_)) {
    return Err(ParseError::unexpected_token(
      "'break'",
      &format!("{:?}", tokens[and_idx + 1].token_type),
      tokens[and_idx + 1].start
    ));
  }

  // Parse the select expression
  let select_tokens = &tokens[0..and_idx];
  let select_expr = expression_parser::parse_expression(select_tokens)?;

  // The rest is similar to array comprehension
  // Find the 'for' keyword after the 'break'
  let for_tokens = &tokens[(and_idx + 2)..];

  // We'll reuse the array comprehension logic for the rest of the parsing
  // and convert it to a find comprehension at the end
  if let Ok(Expr::ArrayComp(array_comp)) = array_parser::parse_array_comprehension(for_tokens) {
    return Ok(Expr::FindComp(FindComp {
      select: Box::new(select_expr),
      var: array_comp.var,
      mode: array_comp.mode,
      iter: array_comp.iter,
      filter: array_comp.filter,
    }));
  }

  Err(ParseError::invalid_expression(
    "Invalid find comprehension syntax",
    tokens[and_idx].start
  ))
}
