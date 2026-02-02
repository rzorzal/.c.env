// Test helper functions shared across all test modules

use crate::grama::build_statements;
use crate::grama::gramma_rules::{Expr, Program, Stmt};
use crate::grama::parser::operator::parse_binary_expression;
use crate::lexing;

/// Parse a complete program from source code
pub fn parse_program(code: &str) -> Result<Program, String> {
    let tokens = lexing::analyze_code(code);
    build_statements(&tokens).map_err(|e| format!("{:?}", e))
}

/// Extract the expression from a variable declaration
pub fn extract_var_expr<'a>(program: &'a Program, var_name: &str) -> Option<&'a Expr> {
    program.items.iter().find_map(|stmt| {
        if let Stmt::VarDecl { name, value, .. } = stmt {
            if name == var_name { Some(value) } else { None }
        } else {
            None
        }
    })
}

/// Extract the expression from an assignment statement or variable declaration
/// Returns the LAST occurrence (useful for finding reassignments)
pub fn extract_assignment_expr<'a>(program: &'a Program, var_name: &str) -> Option<&'a Expr> {
    program.items.iter().rev().find_map(|stmt| match stmt {
        Stmt::Assignment { target, value } => {
            if target == var_name {
                Some(value)
            } else {
                None
            }
        }
        Stmt::VarDecl { name, value, .. } => {
            if name == var_name {
                Some(value)
            } else {
                None
            }
        }
        _ => None,
    })
}

/// Parse just an expression from tokens (skipping variable declaration)
pub fn parse_expression_from_code(code: &str) -> Result<Expr, String> {
    let tokens = lexing::analyze_code(code);

    // Find the tokens after "=" sign
    let expr_tokens: Vec<_> = tokens
        .iter()
        .skip_while(|t| !matches!(t.token_type, lexing::TokenType::Assign(_)))
        .skip(1)
        .take_while(|t| !matches!(t.token_type, lexing::TokenType::Eol(_)))
        .cloned()
        .collect();

    parse_binary_expression(&expr_tokens, 0).map_err(|e| format!("{:?}", e))
}
