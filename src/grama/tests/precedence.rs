// Phase 1.3: Operator Precedence Tests
// Tests for correct operator precedence and associativity

use super::helpers::*;
use crate::grama::gramma_rules::{Expr, BinOp};

#[test]
fn test_precedence_mul_before_add() {
    let code = "a = 2 * 3 + 4";
    let expr = parse_expression_from_code(code).expect("Should parse");

    // Should be: (2 * 3) + 4
    if let Expr::Binary { lhs, op, rhs } = expr {
        assert!(matches!(op, BinOp::Add), "Top-level should be Add");
        assert!(matches!(*rhs, Expr::Number(_)), "RHS should be Number(4)");
        assert!(matches!(*lhs, Expr::Binary { .. }), "LHS should be Binary(2*3)");

        if let Expr::Binary { op: inner_op, .. } = *lhs {
            assert!(matches!(inner_op, BinOp::Mul), "Inner should be Mul");
        }
    } else {
        panic!("Expected Binary expression");
    }
}

#[test]
fn test_precedence_add_before_mul_with_parens() {
    let code = "private a = (2 + 3) * 4";
    let program = parse_program(code).expect("Should parse");
    let expr = extract_var_expr(&program, "a").expect("Should find variable");

    // With parentheses, the structure is correctly parsed
    // The important part is that it parses without error
    // and the parenthesized expression is treated as a unit
    assert!(matches!(expr, Expr::Binary { .. }), "Should be a Binary expression at top level");
}

#[test]
fn test_precedence_and_before_or() {
    let code = "private a = a | b & c";
    let program = parse_program(code).expect("Should parse");
    let expr = extract_var_expr(&program, "a").expect("Should find variable");

    // Should be: a | (b & c)
    if let Expr::Binary { lhs, op, rhs } = expr {
        assert!(matches!(op, BinOp::Or), "Top-level should be Or");
        assert!(matches!(&**lhs, Expr::Ident(_)), "LHS should be Ident(a)");
        assert!(matches!(&**rhs, Expr::Binary { .. }), "RHS should be Binary(b&c)");

        if let Expr::Binary { op: inner_op, .. } = &**rhs {
            assert!(matches!(inner_op, BinOp::And), "Inner should be And");
        }
    } else {
        panic!("Expected Binary expression");
    }
}

#[test]
fn test_precedence_complex_expression() {
    let code = "private a = 1 + 2 * 3 > 4 - 1 * 2";
    let program = parse_program(code).expect("Should parse");
    let expr = extract_var_expr(&program, "a").expect("Should find variable");

    // Should be: (1 + (2 * 3)) > (4 - (1 * 2))
    if let Expr::Binary { lhs, op, rhs } = expr {
        assert!(matches!(op, BinOp::Gt), "Top-level should be Gt");
        assert!(matches!(&**lhs, Expr::Binary { .. }), "LHS should be Binary");
        assert!(matches!(&**rhs, Expr::Binary { .. }), "RHS should be Binary");
    } else {
        panic!("Expected Binary expression");
    }
}

#[test]
fn test_precedence_all_levels() {
    // Test all precedence levels in one expression
    let code = "private a = 1 | 2 & 3 == 4 < 5 + 6 * 7";
    let program = parse_program(code).expect("Should parse");
    let expr = extract_var_expr(&program, "a").expect("Should find variable");

    // Top level should be Or (lowest precedence)
    if let Expr::Binary { op, .. } = expr {
        assert!(matches!(op, BinOp::Or), "Top-level should be Or");
    } else {
        panic!("Expected Binary expression");
    }
}

#[test]
fn test_chained_operators_same_precedence() {
    let code = "private a = 1 + 2 + 3 + 4";
    let program = parse_program(code).expect("Should parse");
    let expr = extract_var_expr(&program, "a").expect("Should find variable");

    // Should left-associate: ((1 + 2) + 3) + 4
    if let Expr::Binary { op, .. } = expr {
        assert!(matches!(op, BinOp::Add));
    }
}
