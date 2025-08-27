
use crate::lexing;
use crate::grama::gramma_rules::{Program, Stmt, Expr};


pub fn build_statements(lex_vec: &[lexing::Token]) -> Option<Program> {
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

  // Create a program from the parsed statements
  // let program: Option<Program> = build_statements(lex_vec);
  // println!("Program: {:#?}", program);
  println!("Statements: {:#?}", stmts);

  // Parse statements and build a program
  if stmts.is_empty() {
    return None;
  }

  // Try to parse all statements
  let parsed_stmts = parse_statements(&stmts);

  // Return a program from parsed statements
  Some(Program::new(parsed_stmts))
}

fn parse_statements(stmts: &[Vec<lexing::Token>]) -> Vec<Stmt> {
  let mut parsed_stmts: Vec<Stmt> = Vec::new();

  for stmt in stmts {
    if let Some(parsed_stmt) = parse_statement(stmt) {
      parsed_stmts.push(parsed_stmt);
    }
  }

  parsed_stmts
}

fn parse_statement(tokens: &[lexing::Token]) -> Option<Stmt> {
  // Check for variable declaration: 'private' keyword
  if !tokens.is_empty() {
    match &tokens[0].token_type {
      lexing::TokenType::Private(_) => {
        return parse_var_declaration(tokens, true);
      }
      _ => {
        // If not a variable declaration, try to parse as expression statement
        if let Some(expr) = parse_expression(tokens) {
          return Some(Stmt::ExprStmt(expr));
        }
      }
    }
  }

  None
}

fn parse_var_declaration(tokens: &[lexing::Token], is_private: bool) -> Option<Stmt> {
  let start_idx = if is_private { 1 } else { 0 };

  if tokens.len() > start_idx + 2 {
    // Check if we have an identifier
    if let lexing::TokenType::Identifier(name) = &tokens[start_idx].token_type {
      // Check for equals sign
      if tokens.len() > start_idx + 1 {
        if let lexing::TokenType::Assign(_) = &tokens[start_idx + 1].token_type {
          // Parse the expression after the equals sign
          let expr_tokens = &tokens[(start_idx + 2)..];
          if let Some(expr) = parse_expression(expr_tokens) {
            return Some(Stmt::VarDecl {
              private_: is_private,
              name: name.clone(),
              value: expr
            });
          }
        }
      }
    }
  }

  None
}

fn parse_expression(tokens: &[lexing::Token]) -> Option<Expr> {
  if tokens.is_empty() {
    return None;
  }

  // Try to parse binary expressions first
  if let Some(expr) = parse_binary_expression(tokens, 0) {
    return Some(expr);
  }

  // If not a binary expression, try to parse other expressions
  parse_primary_expression(tokens)
}

fn parse_primary_expression(tokens: &[lexing::Token]) -> Option<Expr> {
  if tokens.is_empty() {
    return None;
  }

  // First try to parse if expressions (ternary operator)
  if let Some(if_expr) = parse_if_expression(tokens) {
    return Some(if_expr);
  }

  // Then try to parse find comprehensions
  if let Some(find_expr) = parse_find_comprehension(tokens) {
    return Some(find_expr);
  }

  match &tokens[0].token_type {
    // Literals
    lexing::TokenType::IntegerLiteral(num) => {
      return Some(Expr::Number(*num as f64))
    },
    lexing::TokenType::StringLiteral(value) => {
      return Some(Expr::StringLiteral(value.clone()))
    },
    lexing::TokenType::StringTemplate(_) => {
      return parse_string_template(tokens)
    },
    lexing::TokenType::TrueLiteral(_) => {
      return Some(Expr::Bool(true))
    },
    lexing::TokenType::FalseLiteral(_) => {
      return Some(Expr::Bool(false))
    },
    // Add support for null literal if it exists in your token types
    // If there's no specific null token, you might handle identifiers like "null" here
    _ if tokens.len() == 1 && matches!(&tokens[0].token_type, lexing::TokenType::Identifier(id) if id.to_lowercase() == "null") => {
      return Some(Expr::Null)
    },

    // Identifiers
    lexing::TokenType::Identifier(value) => {
      // Check if this is a function call
      if tokens.len() > 1 && matches!(&tokens[1].token_type, lexing::TokenType::LeftParen(_)) {
        return parse_function_call(tokens);
      }

      // Check if this is an array/object access (identifier followed by brackets)
      if tokens.len() > 1 && matches!(&tokens[1].token_type, lexing::TokenType::LeftBracket(_)) {
        return parse_index_access(tokens);
      }

      // Check if this is a property access (identifier followed by dot)
      if tokens.len() > 2 && matches!(&tokens[1].token_type, lexing::TokenType::Dot(_)) {
        if let lexing::TokenType::Identifier(prop_name) = &tokens[2].token_type {
          // Convert obj.prop to obj["prop"] for consistency
          let target = Expr::Ident(value.clone());
          let index = Expr::StringLiteral(prop_name.clone());

          return Some(Expr::Index {
            target: Box::new(target),
            index: Box::new(index),
          });
        }
      }

      // Regular identifier
      return Some(Expr::Ident(value.clone()))
    },

    // Parenthesized expressions
    lexing::TokenType::LeftParen(_) => {
      // Find the matching right parenthesis
      let mut paren_count = 1;
      let mut end_idx = 1;

      while end_idx < tokens.len() && paren_count > 0 {
        match &tokens[end_idx].token_type {
          lexing::TokenType::LeftParen(_) => paren_count += 1,
          lexing::TokenType::RightParen(_) => paren_count -= 1,
          _ => {}
        }
        end_idx += 1;
      }

      if paren_count == 0 && end_idx > 1 {
        // Parse the expression inside the parentheses
        return parse_expression(&tokens[1..end_idx-1]);
      }
    },

    // Unary expressions
    lexing::TokenType::Plus(_) | lexing::TokenType::Minus(_) => {
      let op = match &tokens[0].token_type {
        lexing::TokenType::Plus(_) => crate::grama::gramma_rules::UnaryOp::Plus,
        lexing::TokenType::Minus(_) => crate::grama::gramma_rules::UnaryOp::Minus,
        _ => unreachable!(),
      };

      if tokens.len() > 1 {
        if let Some(expr) = parse_expression(&tokens[1..]) {
          return Some(Expr::Unary {
            op,
            rhs: Box::new(expr),
          });
        }
      }
    },

    // Array literals
    lexing::TokenType::LeftBracket(_) => {
      return parse_array_expression(tokens);
    },

    _ => {},
  }

  None
}

fn parse_array_expression(tokens: &[lexing::Token]) -> Option<Expr> {
  // First token must be left bracket
  if tokens.is_empty() || !matches!(&tokens[0].token_type, lexing::TokenType::LeftBracket(_)) {
    return None;
  }

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
    return None; // Unmatched brackets
  }

  // Check if this is an array comprehension
  let inner_tokens = &tokens[1..end_idx-1];
  if let Some(array_comp) = parse_array_comprehension(inner_tokens) {
    return Some(array_comp);
  }

  // Parse array elements
  let elements = parse_comma_separated_expressions(inner_tokens);

  // Convert to array expression
  // Note: The Expr enum doesn't have a dedicated Array variant in your grammar,
  // but we can create a template with elements as parts
  let template_parts: Vec<_> = elements.into_iter()
    .map(|expr| crate::grama::gramma_rules::TemplatePart::Expr(expr))
    .collect();

  Some(Expr::Template(template_parts))
}

fn parse_function_call(tokens: &[lexing::Token]) -> Option<Expr> {
  if tokens.len() < 4 || !matches!(&tokens[1].token_type, lexing::TokenType::LeftParen(_)) {
    return None;
  }

  // Parse the callee
  let callee = Expr::Ident(match &tokens[0].token_type {
    lexing::TokenType::Identifier(name) => name.clone(),
    _ => return None,
  });

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

  if paren_count != 0 || end_idx <= 2 {
    return None; // Unmatched parentheses
  }

  // Parse arguments
  let args_tokens = &tokens[2..end_idx-1];
  let args = parse_comma_separated_expressions(args_tokens);

  Some(Expr::Call {
    callee: Box::new(callee),
    args,
  })
}

fn parse_comma_separated_expressions(tokens: &[lexing::Token]) -> Vec<Expr> {
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
        if let Some(expr) = parse_expression(&tokens[start_idx..i]) {
          expressions.push(expr);
        }
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

  expressions
}

fn parse_binary_expression(tokens: &[lexing::Token], min_precedence: u8) -> Option<Expr> {
  if tokens.is_empty() {
    return None;
  }

  // First parse the left-hand side as a primary expression
  let mut lhs = parse_primary_expression(tokens)?;

  let mut position = 1;
  while position < tokens.len() {
    // Lookahead to find a binary operator
    let op_token = &tokens[position];
    let (op, precedence) = match &op_token.token_type {
      lexing::TokenType::Plus(_) => (crate::grama::gramma_rules::BinOp::Add, 1),
      lexing::TokenType::Minus(_) => (crate::grama::gramma_rules::BinOp::Sub, 1),
      lexing::TokenType::Multiply(_) => (crate::grama::gramma_rules::BinOp::Mul, 2),
      lexing::TokenType::Divider(_) => (crate::grama::gramma_rules::BinOp::Div, 2),
      lexing::TokenType::Mod(_) => (crate::grama::gramma_rules::BinOp::Mod, 2),
      lexing::TokenType::Equal(_) => (crate::grama::gramma_rules::BinOp::Eq, 0),
      lexing::TokenType::NotEqual(_) => (crate::grama::gramma_rules::BinOp::Ne, 0),
      lexing::TokenType::LessThan(_) => (crate::grama::gramma_rules::BinOp::Lt, 0),
      lexing::TokenType::LessThanOrEqual(_) => (crate::grama::gramma_rules::BinOp::Le, 0),
      lexing::TokenType::GreaterThan(_) => (crate::grama::gramma_rules::BinOp::Gt, 0),
      lexing::TokenType::GreaterThanOrEqual(_) => (crate::grama::gramma_rules::BinOp::Ge, 0),
      lexing::TokenType::And(_) => (crate::grama::gramma_rules::BinOp::And, 0),
      lexing::TokenType::Or(_) => (crate::grama::gramma_rules::BinOp::Or, 0),
      _ => break,
    };

    // If this operator's precedence is too low, break
    if precedence < min_precedence {
      break;
    }

    // Look past the operator to parse the RHS
    position += 1;
    if position >= tokens.len() {
      break;
    }

    // Recursively parse the right-hand side with higher precedence
    if let Some(rhs) = parse_binary_expression(&tokens[position..], precedence + 1) {
      lhs = Expr::Binary {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
      };
    } else {
      // If we couldn't parse a valid RHS, try to parse a primary expression
      if let Some(rhs) = parse_primary_expression(&tokens[position..]) {
        lhs = Expr::Binary {
          lhs: Box::new(lhs),
          op,
          rhs: Box::new(rhs),
        };
      }
    }

    // Move past the RHS
    position = tokens.len();
  }

  Some(lhs)
}

fn parse_array_comprehension(tokens: &[lexing::Token]) -> Option<Expr> {
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
    return None; // No 'for' found or it's at the beginning
  }

  // Parse the expression to include in the array
  let expr_tokens = &tokens[0..for_idx];
  let expr = parse_expression(expr_tokens)?;

  // Parse variable name
  if for_idx + 1 >= tokens.len() {
    return None;
  }

  let var_token = &tokens[for_idx + 1];
  let var = match &var_token.token_type {
    lexing::TokenType::Identifier(name) => name.clone(),
    _ => return None,
  };

  // Check for 'of' or 'in'
  if for_idx + 2 >= tokens.len() {
    return None;
  }

  let mode_token = &tokens[for_idx + 2];
  let mode = match &mode_token.token_type {
    lexing::TokenType::Of(_) => crate::grama::gramma_rules::IterMode::Of,
    lexing::TokenType::In(_) => crate::grama::gramma_rules::IterMode::In,
    _ => return None,
  };

  // Parse iterator expression
  if for_idx + 3 >= tokens.len() {
    return None;
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
    parse_expression(filter_tokens).map(Box::new)
  } else {
    None
  };

  // Create array comprehension
  Some(Expr::ArrayComp(crate::grama::gramma_rules::ArrayComp {
    expr: Box::new(expr),
    var,
    mode,
    iter: Box::new(iter),
    filter,
  }))
}

fn parse_if_expression(tokens: &[lexing::Token]) -> Option<Expr> {
  // If expression pattern: if <cond> ? <then_expr> else <else_expr>

  // First find the 'if' keyword
  if tokens.is_empty() || !matches!(&tokens[0].token_type, lexing::TokenType::If(_)) {
    return None;
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
    return None; // No '?' found
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
    return None; // No 'else' found
  }

  // Parse the then expression
  let then_tokens = &tokens[(question_idx + 1)..else_idx];
  let then_expr = parse_expression(then_tokens)?;

  // Parse the else expression
  let else_tokens = &tokens[(else_idx + 1)..];
  let else_expr = parse_expression(else_tokens)?;

  Some(Expr::IfExpr {
    cond: Box::new(cond),
    then_: Box::new(then_expr),
    else_: Box::new(else_expr),
  })
}

fn parse_string_template(tokens: &[lexing::Token]) -> Option<Expr> {
  // String template parsing
  // This is a simplified implementation - in a real parser you'd need to handle
  // the complex parsing of the template with expressions inside

  if tokens.is_empty() {
    return None;
  }

  // For now, we'll just handle a basic template
  if let lexing::TokenType::StringTemplate(content) = &tokens[0].token_type {
    // Here we would parse the content to find expressions inside the template
    // For now, we'll just create a simple template with text parts

    let parts = vec![
      crate::grama::gramma_rules::TemplatePart::Text(content.clone())
    ];

    return Some(Expr::Template(parts));
  }

  None
}

fn parse_index_access(tokens: &[lexing::Token]) -> Option<Expr> {
  // Array/object index access pattern: target[index]
  if tokens.len() < 4 {
    return None;
  }

  // Parse the target expression (usually an identifier)
  let target = match &tokens[0].token_type {
    lexing::TokenType::Identifier(name) => Expr::Ident(name.clone()),
    _ => return None,
  };

  // Ensure the second token is a left bracket
  if !matches!(&tokens[1].token_type, lexing::TokenType::LeftBracket(_)) {
    return None;
  }

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

  if bracket_count != 0 || end_idx <= 3 {
    return None; // Unmatched brackets or empty brackets
  }

  // Parse the index expression
  let index_tokens = &tokens[2..end_idx-1];
  let index = parse_expression(index_tokens)?;

  // Create the index access expression
  Some(Expr::Index {
    target: Box::new(target),
    index: Box::new(index),
  })
}

fn parse_find_comprehension(tokens: &[lexing::Token]) -> Option<Expr> {
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
    return None;
  }

  // Check if the next token is 'break'
  if and_idx + 1 >= tokens.len() {
    return None;
  }

  if !matches!(&tokens[and_idx + 1].token_type, lexing::TokenType::Break(_)) {
    return None;
  }

  // Parse the select expression
  let select_tokens = &tokens[0..and_idx];
  let select_expr = parse_expression(select_tokens)?;

  // The rest is similar to array comprehension
  // Find the 'for' keyword after the 'break'
  let for_tokens = &tokens[(and_idx + 2)..];

  // We'll reuse the array comprehension logic for the rest of the parsing
  // and convert it to a find comprehension at the end
  if let Some(Expr::ArrayComp(array_comp)) = parse_array_comprehension(for_tokens) {
    return Some(Expr::FindComp(crate::grama::gramma_rules::FindComp {
      select: Box::new(select_expr),
      var: array_comp.var,
      mode: array_comp.mode,
      iter: array_comp.iter,
      filter: array_comp.filter,
    }));
  }

  None
}
