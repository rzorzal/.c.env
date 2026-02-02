// Main parser orchestration - coordinates the parsing process

use crate::lexing;
use crate::grama::gramma_rules::{Program, Stmt, Expr};
use crate::grama::error::{ParseError, ParseResult};
use super::statement_parser;
use super::expression_parser;

/// Build a complete program from a list of tokens
/// This is the main entry point for the parser
pub fn build_statements(lex_vec: &[lexing::Token]) -> ParseResult<Program> {
  let mut stmts: Vec<Vec<lexing::Token>> = Vec::new();
  let mut stmt: Vec<lexing::Token> = Vec::new();
  let mut brace_depth: i32 = 0; // Track nesting depth of braces

  for token in lex_vec {
    // Track brace depth to know when we're inside a block
    match &token.token_type {
      lexing::TokenType::LeftBrace(_) => brace_depth += 1,
      lexing::TokenType::RightBrace(_) => brace_depth = brace_depth.saturating_sub(1),
      _ => {}
    }

    match &token.token_type {
      lexing::TokenType::Eol(_) if brace_depth == 0 => {
        // End of line (only split if not inside braces)
        if !stmt.is_empty() {
          stmts.push(stmt);
          stmt = Vec::new(); // Start a new statement
        }
      }
      _ => {
        // Add token to current statement
        stmt.push(token.clone());
      }
    }
  }

  // Don't forget to add the last statement if it's not empty
  if !stmt.is_empty() {
    stmts.push(stmt);
  }

  // Parse statements and build a program
  if stmts.is_empty() {
    return Ok(Program::empty());
  }

  // Try to parse all statements
  let parsed_stmts = parse_statements(&stmts)?;

  // Return a program from parsed statements
  Ok(Program::new(parsed_stmts))
}

fn parse_statements(stmts: &[Vec<lexing::Token>]) -> ParseResult<Vec<Stmt>> {
  let mut parsed_stmts: Vec<Stmt> = Vec::new();
  let mut errors: Vec<ParseError> = Vec::new();

  for stmt in stmts {
    match statement_parser::parse_statement(stmt) {
      Ok(parsed_stmt) => parsed_stmts.push(parsed_stmt),
      Err(err) => {
        // Collect errors but continue parsing (error recovery)
        errors.push(err);
      }
    }
  }

  // If we collected any errors, return the first one
  // In the future, we could return all errors
  if let Some(first_error) = errors.first() {
    return Err(first_error.clone());
  }

  Ok(parsed_stmts)
}

/// Parse an expression from a token sequence
/// This is exposed for internal use and testing
#[allow(dead_code)]
pub fn parse_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  expression_parser::parse_expression(tokens)
}
