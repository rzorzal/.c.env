
use crate::lexing;
use crate::grama::gramma_rules::{Program, Stmt, Expr};
use crate::grama::error::{ParseError, ParseResult};


pub fn build_statements(lex_vec: &[lexing::Token]) -> ParseResult<Program> {
  let mut stmts: Vec<Vec<lexing::Token>> = Vec::new();
  let mut stmt: Vec<lexing::Token> = Vec::new();

  for token in lex_vec {
    match &token.token_type {
      lexing::TokenType::Eol(_) => {
        // End of line, push current statement to stmts
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
    match parse_statement(stmt) {
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

fn parse_statement(tokens: &[lexing::Token]) -> ParseResult<Stmt> {
  // Check for variable declaration: 'private' keyword
  if tokens.is_empty() {
    return Err(ParseError::invalid_statement("Empty statement", 0));
  }

  match &tokens[0].token_type {
    lexing::TokenType::Private(_) => {
      parse_var_declaration(tokens, true)
    }
    _ => {
      // Check if it's an assignment: identifier followed by '='
      if tokens.len() >= 2 {
        if let lexing::TokenType::Identifier(_name) = &tokens[0].token_type {
          if let lexing::TokenType::Assign(_) = &tokens[1].token_type {
            // This is an assignment (or attempted assignment)
            return parse_assignment(tokens);
          }
        }
      }

      // If not a variable declaration or assignment, try to parse as expression statement
      let expr = parse_expression(tokens)?;
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
      let expr = parse_expression(expr_tokens)?;

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
  let value = parse_expression(expr_tokens)?;

  Ok(Stmt::Assignment { target, value })
}

fn parse_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  if tokens.is_empty() {
    return Err(ParseError::invalid_expression("Empty expression", 0));
  }

  // Try to parse binary expressions first
  if let Ok(expr) = parse_binary_expression(tokens, 0) {
    return Ok(expr);
  }

  // If not a binary expression, try to parse other expressions
  parse_primary_expression(tokens)
}

fn parse_primary_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  let (expr, _) = parse_primary_expression_with_count(tokens)?;
  Ok(expr)
}

/// Parse a primary expression and return the expression plus the number of tokens consumed
fn parse_primary_expression_with_count(tokens: &[lexing::Token]) -> ParseResult<(Expr, usize)> {
  if tokens.is_empty() {
    return Err(ParseError::invalid_expression("Empty expression", 0));
  }

  // First try to parse if expressions (ternary operator)
  if let Ok(if_expr) = parse_if_expression(tokens) {
    // If expressions consume all tokens they need, but we don't have an easy way
    // to know how many. For now, return 1 (this is a limitation)
    return Ok((if_expr, 1));
  }

  // Then try to parse find comprehensions
  if let Ok(find_expr) = parse_find_comprehension(tokens) {
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
      let expr = parse_string_template(tokens)?;
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
        return parse_function_call_with_count(tokens);
      }

      // Check if this is an array/object access
      if tokens.len() > 1 && matches!(&tokens[1].token_type, lexing::TokenType::LeftBracket(_)) {
        let expr = parse_index_access(tokens)?;
        return Ok((expr, 1));
      }

      // Check if this is a property access
      if tokens.len() > 2 && matches!(&tokens[1].token_type, lexing::TokenType::Dot(_)) {
        if let lexing::TokenType::Identifier(prop_name) = &tokens[2].token_type {
          let target = Expr::Ident(value.clone());
          let index = Expr::StringLiteral(prop_name.clone());
          return Ok((Expr::Index {
            target: Box::new(target),
            index: Box::new(index),
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
        return parse_function_call_with_count(tokens);
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
        return Err(ParseError::unmatched_delimiter(')', token_pos, Some(opening_pos)));
      }

      if end_idx > 1 {
        // Parse the expression inside the parentheses
        let expr = parse_expression(&tokens[1..end_idx-1])?;
        // Consumed: opening paren + inner tokens + closing paren = end_idx tokens
        return Ok((expr, end_idx));
      }

      Err(ParseError::invalid_expression("Empty parentheses", token_pos))
    },

    // Unary expressions
    lexing::TokenType::Plus(_) | lexing::TokenType::Minus(_) => {
      let op = match &tokens[0].token_type {
        lexing::TokenType::Plus(_) => crate::grama::gramma_rules::UnaryOp::Plus,
        lexing::TokenType::Minus(_) => crate::grama::gramma_rules::UnaryOp::Minus,
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
    },

    // Array literals
    lexing::TokenType::LeftBracket(_) => {
      let expr = parse_array_expression(tokens)?;
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

fn parse_array_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  // First token must be left bracket
  if tokens.is_empty() || !matches!(&tokens[0].token_type, lexing::TokenType::LeftBracket(_)) {
    let pos = if tokens.is_empty() { 0 } else { tokens[0].start };
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
    return Err(ParseError::unmatched_delimiter(']', opening_pos, Some(opening_pos)));
  }

  // Check if this is an array comprehension
  let inner_tokens = &tokens[1..end_idx-1];
  if let Ok(array_comp) = parse_array_comprehension(inner_tokens) {
    return Ok(array_comp);
  }

  // Parse array elements
  let elements = parse_comma_separated_expressions(inner_tokens)?;

  // Convert to array expression
  // Note: The Expr enum doesn't have a dedicated Array variant in your grammar,
  // but we can create a template with elements as parts
  let template_parts: Vec<_> = elements.into_iter()
    .map(|expr| crate::grama::gramma_rules::TemplatePart::Expr(expr))
    .collect();

  Ok(Expr::Template(template_parts))
}

fn parse_function_call(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  let (expr, _) = parse_function_call_with_count(tokens)?;
  Ok(expr)
}

fn parse_function_call_with_count(tokens: &[lexing::Token]) -> ParseResult<(Expr, usize)> {
  if tokens.len() < 3 || !matches!(&tokens[1].token_type, lexing::TokenType::LeftParen(_)) {
    let pos = if tokens.is_empty() { 0 } else { tokens[0].start };
    return Err(ParseError::invalid_expression("Expected '(' after function name", pos));
  }

  // Parse the callee - extract function name from various token types
  let callee_name = match &tokens[0].token_type {
    lexing::TokenType::Identifier(name) => name.clone(),
    lexing::TokenType::Print(name) => name.clone(),
    lexing::TokenType::Import(name) => name.clone(),
    lexing::TokenType::Range(name) => name.clone(),
    lexing::TokenType::ImportAwsSecret(name) => name.clone(),
    _ => return Err(ParseError::invalid_expression("Expected identifier or function name", tokens[0].start)),
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
    return Err(ParseError::unmatched_delimiter(')', opening_pos, Some(opening_pos)));
  }

  // Parse arguments
  let args_tokens = &tokens[2..end_idx-1];
  let args = if args_tokens.is_empty() {
    Vec::new()
  } else {
    parse_comma_separated_expressions(args_tokens)?
  };

  let expr = Expr::Call {
    callee: Box::new(callee),
    args,
  };

  // Return expression and number of tokens consumed
  // Consumed: function_name + ( + args + ) = end_idx tokens
  Ok((expr, end_idx))
}

fn parse_comma_separated_expressions(tokens: &[lexing::Token]) -> ParseResult<Vec<Expr>> {
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
fn get_operator_precedence(token_type: &lexing::TokenType) -> Option<(crate::grama::gramma_rules::BinOp, u8)> {
  match token_type {
    // Level 0: Logical OR (lowest precedence)
    lexing::TokenType::Or(_) => Some((crate::grama::gramma_rules::BinOp::Or, 0)),

    // Level 1: Logical AND
    lexing::TokenType::And(_) => Some((crate::grama::gramma_rules::BinOp::And, 1)),

    // Level 2: Equality operators
    lexing::TokenType::Equal(_) => Some((crate::grama::gramma_rules::BinOp::Eq, 2)),
    lexing::TokenType::NotEqual(_) => Some((crate::grama::gramma_rules::BinOp::Ne, 2)),

    // Level 3: Comparison operators
    lexing::TokenType::LessThan(_) => Some((crate::grama::gramma_rules::BinOp::Lt, 3)),
    lexing::TokenType::LessThanOrEqual(_) => Some((crate::grama::gramma_rules::BinOp::Le, 3)),
    lexing::TokenType::GreaterThan(_) => Some((crate::grama::gramma_rules::BinOp::Gt, 3)),
    lexing::TokenType::GreaterThanOrEqual(_) => Some((crate::grama::gramma_rules::BinOp::Ge, 3)),

    // Level 4: Addition and Subtraction
    lexing::TokenType::Plus(_) => Some((crate::grama::gramma_rules::BinOp::Add, 4)),
    lexing::TokenType::Minus(_) => Some((crate::grama::gramma_rules::BinOp::Sub, 4)),

    // Level 5: Multiplication, Division, and Modulo (highest precedence)
    lexing::TokenType::Multiply(_) => Some((crate::grama::gramma_rules::BinOp::Mul, 5)),
    lexing::TokenType::Divider(_) => Some((crate::grama::gramma_rules::BinOp::Div, 5)),
    lexing::TokenType::Mod(_) => Some((crate::grama::gramma_rules::BinOp::Mod, 5)),

    // Not a binary operator
    _ => None,
  }
}

pub fn parse_binary_expression(tokens: &[lexing::Token], min_precedence: u8) -> ParseResult<Expr> {
  let (expr, _) = parse_binary_expression_impl(tokens, 0, min_precedence)?;
  Ok(expr)
}

fn parse_binary_expression_impl(tokens: &[lexing::Token], mut pos: usize, min_precedence: u8) -> ParseResult<(Expr, usize)> {
  if pos >= tokens.len() {
    return Err(ParseError::invalid_expression("Empty expression in binary operation", 0));
  }

  // First parse the left-hand side as a primary expression
  let (mut lhs, tokens_consumed) = parse_primary_expression_with_count(&tokens[pos..])?;
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
        op_token.start
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

fn parse_array_comprehension(tokens: &[lexing::Token]) -> ParseResult<Expr> {
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
    let pos = if tokens.is_empty() { 0 } else { tokens[0].start };
    return Err(ParseError::invalid_expression(
      "Array comprehension requires 'for' keyword",
      pos
    ));
  }

  // Parse the expression to include in the array
  let expr_tokens = &tokens[0..for_idx];
  let expr = parse_expression(expr_tokens)?;

  // Parse variable name
  if for_idx + 1 >= tokens.len() {
    return Err(ParseError::invalid_expression(
      "Expected variable name after 'for'",
      tokens[for_idx].end
    ));
  }

  let var_token = &tokens[for_idx + 1];
  let var = match &var_token.token_type {
    lexing::TokenType::Identifier(name) => name.clone(),
    _ => return Err(ParseError::unexpected_token(
      "variable name",
      &format!("{:?}", var_token.token_type),
      var_token.start
    )),
  };

  // Check for 'of' or 'in'
  if for_idx + 2 >= tokens.len() {
    return Err(ParseError::invalid_expression(
      "Expected 'of' or 'in' after variable name",
      tokens[for_idx + 1].end
    ));
  }

  let mode_token = &tokens[for_idx + 2];
  let mode = match &mode_token.token_type {
    lexing::TokenType::Of(_) => crate::grama::gramma_rules::IterMode::Of,
    lexing::TokenType::In(_) => crate::grama::gramma_rules::IterMode::In,
    _ => return Err(ParseError::unexpected_token(
      "'of' or 'in'",
      &format!("{:?}", mode_token.token_type),
      mode_token.start
    )),
  };

  // Parse iterator expression
  if for_idx + 3 >= tokens.len() {
    return Err(ParseError::invalid_expression(
      "Expected iterator expression",
      tokens[for_idx + 2].end
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
  let iter = parse_expression(iter_tokens)?;

  // Parse optional filter condition
  let filter = if iter_end_idx < tokens.len() && matches!(&tokens[iter_end_idx].token_type, lexing::TokenType::If(_)) {
    let filter_tokens = &tokens[(iter_end_idx + 1)..];
    Some(Box::new(parse_expression(filter_tokens)?))
  } else {
    None
  };

  // Create array comprehension
  Ok(Expr::ArrayComp(crate::grama::gramma_rules::ArrayComp {
    expr: Box::new(expr),
    var,
    mode,
    iter: Box::new(iter),
    filter,
  }))
}

fn parse_if_expression(tokens: &[lexing::Token]) -> ParseResult<Expr> {
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
        if paren_count > 0 {
          paren_count -= 1;
        }
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
  let cond = parse_expression(cond_tokens)?;

  // Find the 'else' keyword
  let mut else_idx = question_idx + 1;
  paren_count = 0;

  while else_idx < tokens.len() {
    match &tokens[else_idx].token_type {
      lexing::TokenType::LeftParen(_) => paren_count += 1,
      lexing::TokenType::RightParen(_) => {
        if paren_count > 0 {
          paren_count -= 1;
        }
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
  let then_expr = parse_expression(then_tokens)?;

  // Parse the else expression
  let else_tokens = &tokens[(else_idx + 1)..];
  let else_expr = parse_expression(else_tokens)?;

  Ok(Expr::IfExpr {
    cond: Box::new(cond),
    then_: Box::new(then_expr),
    else_: Box::new(else_expr),
  })
}

fn parse_string_template(tokens: &[lexing::Token]) -> ParseResult<Expr> {
  // String template parsing
  // This is a simplified implementation - in a real parser you'd need to handle
  // the complex parsing of the template with expressions inside

  if tokens.is_empty() {
    return Err(ParseError::invalid_expression("Empty template", 0));
  }

  // For now, we'll just handle a basic template
  if let lexing::TokenType::StringTemplate(content) = &tokens[0].token_type {
    // Here we would parse the content to find expressions inside the template
    // For now, we'll just create a simple template with text parts

    let parts = vec![
      crate::grama::gramma_rules::TemplatePart::Text(content.clone())
    ];

    return Ok(Expr::Template(parts));
  }

  Err(ParseError::unexpected_token(
    "string template",
    &format!("{:?}", tokens[0].token_type),
    tokens[0].start
  ))
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

fn parse_find_comprehension(tokens: &[lexing::Token]) -> ParseResult<Expr> {
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
  let select_expr = parse_expression(select_tokens)?;

  // The rest is similar to array comprehension
  // Find the 'for' keyword after the 'break'
  let for_tokens = &tokens[(and_idx + 2)..];

  // We'll reuse the array comprehension logic for the rest of the parsing
  // and convert it to a find comprehension at the end
  if let Ok(Expr::ArrayComp(array_comp)) = parse_array_comprehension(for_tokens) {
    return Ok(Expr::FindComp(crate::grama::gramma_rules::FindComp {
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
