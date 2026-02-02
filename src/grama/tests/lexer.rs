// Phase 1.2: Lexer Tests
// Tests for tokenization, comments, and operators

use super::helpers::*;
use crate::grama::gramma_rules::{BinOp, Expr};

#[test]
fn test_single_line_comments() {
    let code = r#"
// This is a comment
private x = 5 // End of line comment
"#;
    let result = parse_program(code);
    assert!(result.is_ok(), "Should parse with single-line comments");

    let program = result.unwrap();
    assert_eq!(
        program.items.len(),
        1,
        "Should have 1 statement (comment removed)"
    );
}

#[test]
fn test_multi_line_comments() {
    let code = r#"
/* Multi-line
   comment here */
private x = 10
/* Another block comment */
"#;
    let result = parse_program(code);
    assert!(result.is_ok(), "Should parse with multi-line comments");

    let program = result.unwrap();
    assert_eq!(program.items.len(), 1, "Should have 1 statement");
}

#[test]
fn test_comparison_operators() {
    let code = "private a = 1 <= 2";
    let result = parse_program(code);
    assert!(result.is_ok(), "Should parse <= operator");

    let program = result.unwrap();
    let expr = extract_var_expr(&program, "a").expect("Should find variable 'a'");
    if let Expr::Binary { op, .. } = expr {
        assert!(matches!(op, BinOp::Le), "Should be Le operator");
    } else {
        panic!("Expected Binary expression");
    }
}

#[test]
fn test_equality_operator() {
    let code = "private a = 1 == 2";
    let result = parse_program(code);
    assert!(result.is_ok(), "Should parse == operator");

    let program = result.unwrap();
    let expr = extract_var_expr(&program, "a").expect("Should find variable 'a'");
    if let Expr::Binary { op, .. } = expr {
        assert!(matches!(op, BinOp::Eq), "Should be Eq operator");
    } else {
        panic!("Expected Binary expression");
    }
}
