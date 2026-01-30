// Statement parsing - handles variable declarations and assignments

use crate::lexing;
use crate::grama::gramma_rules::Stmt;
use crate::grama::error::{ParseError, ParseResult};
use super::expression_parser;

/// Parse a single statement from tokens
pub(super) fn parse_statement(tokens: &[lexing::Token]) -> ParseResult<Stmt> {
  // Check for variable declaration: 'private' keyword
  if tokens.is_empty() {
    return Err(ParseError::invalid_statement("Empty statement", 0));
  }

  match &tokens[0].token_type {
    lexing::TokenType::Private(_) => {
      parse_var_declaration(tokens, true)
    }
    lexing::TokenType::Import(_) | lexing::TokenType::ImportAwsSecret(_) => {
      parse_import_statement(tokens)
    }
    lexing::TokenType::LeftBrace(_) => {
      parse_block_statement(tokens)
    }
    _ => {
      // Check if it's an assignment or public variable declaration: identifier followed by '='
      if tokens.len() >= 2 {
        if let lexing::TokenType::Identifier(_name) = &tokens[0].token_type {
          if let lexing::TokenType::Assign(_) = &tokens[1].token_type {
            // In C.env, 'identifier = value' without 'private' is a public variable declaration
            // This handles both first-time declarations and reassignments
            return parse_var_declaration(tokens, false);
          }
        }
      }

      // If not a variable declaration or assignment, try to parse as expression statement
      let expr = expression_parser::parse_expression(tokens)?;
      Ok(Stmt::ExprStmt(expr))
    }
  }
}

fn parse_var_declaration(tokens: &[lexing::Token], is_private: bool) -> ParseResult<Stmt> {
  let start_idx = if is_private { 1 } else { 0 };

  if tokens.len() <= start_idx {
    let pos = if tokens.is_empty() { 0 } else { tokens[0].start };
    return Err(ParseError::invalid_statement(
      "Expected identifier after 'private'",
      pos
    ));
  }

  // Check if we have an identifier
  if let lexing::TokenType::Identifier(name) = &tokens[start_idx].token_type {
    // Check for equals sign
    if tokens.len() <= start_idx + 1 {
      return Err(ParseError::unexpected_token(
        "'=' (assignment)",
        "end of statement",
        tokens[start_idx].end
      ).with_context("Variable declarations must have a value: 'private name = value'".to_string()));
    }

    if let lexing::TokenType::Assign(_) = &tokens[start_idx + 1].token_type {
      // Parse the expression after the equals sign
      if tokens.len() <= start_idx + 2 {
        return Err(ParseError::invalid_expression(
          "Expected value after '='",
          tokens[start_idx + 1].end
        ));
      }

      let expr_tokens = &tokens[(start_idx + 2)..];
      let expr = expression_parser::parse_expression(expr_tokens)?;

      return Ok(Stmt::VarDecl {
        private_: is_private,
        name: name.clone(),
        value: expr
      });
    } else {
      return Err(ParseError::unexpected_token(
        "'='",
        &format!("{:?}", tokens[start_idx + 1].token_type),
        tokens[start_idx + 1].start
      ));
    }
  }

  Err(ParseError::unexpected_token(
    "identifier",
    &format!("{:?}", tokens[start_idx].token_type),
    tokens[start_idx].start
  ))
}

fn parse_assignment(tokens: &[lexing::Token]) -> ParseResult<Stmt> {
  // tokens[0] should be an identifier, tokens[1] should be '='
  if tokens.is_empty() {
    return Err(ParseError::invalid_statement("Empty assignment", 0));
  }

  // Extract the target identifier
  let target = if let lexing::TokenType::Identifier(name) = &tokens[0].token_type {
    name.clone()
  } else {
    return Err(ParseError::unexpected_token(
      "identifier",
      &format!("{:?}", tokens[0].token_type),
      tokens[0].start
    ));
  };

  // Check for equals sign
  if tokens.len() < 2 {
    return Err(ParseError::unexpected_token(
      "'='",
      "end of statement",
      tokens[0].end
    ));
  }

  if !matches!(tokens[1].token_type, lexing::TokenType::Assign(_)) {
    return Err(ParseError::unexpected_token(
      "'='",
      &format!("{:?}", tokens[1].token_type),
      tokens[1].start
    ));
  }

  // Parse the value expression
  if tokens.len() < 3 {
    return Err(ParseError::invalid_expression(
      "Expected value after '='",
      tokens[1].end
    ));
  }

  let expr_tokens = &tokens[2..];
  let value = expression_parser::parse_expression(expr_tokens)?;

  Ok(Stmt::Assignment { target, value })
}

fn parse_import_statement(tokens: &[lexing::Token]) -> ParseResult<Stmt> {
  // Import syntax: import(path) or import_aws_secret(path)
  // The path should be a string literal inside parentheses

  if tokens.is_empty() {
    return Err(ParseError::invalid_statement("Empty import statement", 0));
  }

  let is_aws_secret = matches!(&tokens[0].token_type, lexing::TokenType::ImportAwsSecret(_));

  // Expect opening parenthesis
  if tokens.len() < 2 || !matches!(&tokens[1].token_type, lexing::TokenType::LeftParen(_)) {
    let got = if tokens.len() > 1 {
      format!("{:?}", tokens[1].token_type)
    } else {
      "end of statement".to_string()
    };
    return Err(ParseError::unexpected_token(
      "'(' after import keyword",
      &got,
      if tokens.len() > 1 { tokens[1].start } else { tokens[0].end }
    ));
  }

  // Find closing parenthesis
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
    return Err(ParseError::unmatched_delimiter(')', tokens[1].start, Some(tokens[1].start)));
  }

  // Parse the argument (can be a string literal or expression)
  if end_idx <= 3 {
    return Err(ParseError::invalid_expression(
      "Expected file path in import",
      tokens[1].end
    ));
  }

  let arg_tokens = &tokens[2..end_idx-1];
  if arg_tokens.is_empty() {
    return Err(ParseError::invalid_expression(
      "Import requires a path argument",
      tokens[2].start
    ));
  }

  // Parse the path as an expression
  let path_expr = expression_parser::parse_expression(arg_tokens)?;

  Ok(Stmt::Import {
    path: path_expr,
    is_aws_secret,
    alias: None, // For now, we don't support aliases
  })
}

fn parse_block_statement(tokens: &[lexing::Token]) -> ParseResult<Stmt> {
  // Block syntax: { stmt1 stmt2 ... }
  if tokens.is_empty() {
    return Err(ParseError::invalid_statement("Empty block statement", 0));
  }

  // Verify we start with a left brace
  if !matches!(&tokens[0].token_type, lexing::TokenType::LeftBrace(_)) {
    return Err(ParseError::unexpected_token(
      "'{'",
      &format!("{:?}", tokens[0].token_type),
      tokens[0].start
    ));
  }

  // Find the matching closing brace
  let mut brace_count = 1;
  let mut end_idx = 1;
  while end_idx < tokens.len() && brace_count > 0 {
    match &tokens[end_idx].token_type {
      lexing::TokenType::LeftBrace(_) => brace_count += 1,
      lexing::TokenType::RightBrace(_) => brace_count -= 1,
      _ => {}
    }
    end_idx += 1;
  }

  if brace_count != 0 {
    return Err(ParseError::unmatched_delimiter('}', tokens[0].start, Some(tokens[0].start)));
  }

  // Parse statements inside the block
  let inner_tokens = &tokens[1..end_idx-1];

  // Empty block is valid
  if inner_tokens.is_empty() {
    return Ok(Stmt::Block(vec![]));
  }

  // Split the inner tokens into individual statements (similar to build_statements)
  let mut statements = Vec::new();
  let mut current_start = 0;
  let mut brace_depth: i32 = 0;

  for (i, token) in inner_tokens.iter().enumerate() {
    // Track brace depth for nested blocks
    match &token.token_type {
      lexing::TokenType::LeftBrace(_) => brace_depth += 1,
      lexing::TokenType::RightBrace(_) => brace_depth = brace_depth.saturating_sub(1),
      _ => {}
    }

    // Only split on Eol when not inside nested braces
    if matches!(&token.token_type, lexing::TokenType::Eol(_)) && brace_depth == 0 {
      // Parse the statement up to this point
      if i > current_start {
        let stmt_tokens = &inner_tokens[current_start..i];
        if !stmt_tokens.is_empty() {
          let stmt = parse_statement(stmt_tokens)?;
          statements.push(stmt);
        }
      }
      current_start = i + 1;
    }
  }

  // Parse any remaining statement after the last newline
  if current_start < inner_tokens.len() {
    let stmt_tokens = &inner_tokens[current_start..];
    if !stmt_tokens.is_empty() {
      let stmt = parse_statement(stmt_tokens)?;
      statements.push(stmt);
    }
  }

  Ok(Stmt::Block(statements))
}
