// Phase 2.1: Assignment Statement Tests
// Tests for variable reassignment functionality

use super::helpers::*;
use crate::grama::gramma_rules::{Expr, BinOp, Stmt};

#[test]
fn test_simple_assignment() {
    let code = r#"
private x = 5
x = 10
"#;
    let program = parse_program(code).expect("Should parse assignment");
    assert_eq!(program.items.len(), 2, "Should have declaration and assignment");

    // Check that first is a declaration
    assert!(matches!(program.items[0], Stmt::VarDecl { .. }));

    // Check that second is an assignment
    if let Stmt::Assignment { target, value } = &program.items[1] {
        assert_eq!(target, "x");
        assert!(matches!(value, Expr::Number(10.0)));
    } else {
        panic!("Expected assignment statement");
    }
}

#[test]
fn test_assignment_with_expression() {
    let code = r#"
private a = 1
a = 2 + 3 * 4
"#;
    let program = parse_program(code).expect("Should parse");
    let expr = extract_assignment_expr(&program, "a").expect("Should find assignment");

    // Should respect precedence: 2 + (3 * 4)
    if let Expr::Binary { lhs, op, rhs } = expr {
        assert!(matches!(op, BinOp::Add));
        assert!(matches!(**lhs, Expr::Number(2.0)));
        assert!(matches!(**rhs, Expr::Binary { op: BinOp::Mul, .. }));
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_assignment_with_variable_reference() {
    let code = r#"
private x = 5
private y = 10
x = y
"#;
    let program = parse_program(code).expect("Should parse");
    let expr = extract_assignment_expr(&program, "x").expect("Should find assignment");

    assert!(matches!(expr, Expr::Ident(name) if name == "y"));
}

#[test]
fn test_assignment_chain() {
    let code = r#"
private a = 1
private b = 2
a = 10
b = 20
a = 30
"#;
    let program = parse_program(code).expect("Should parse multiple assignments");
    assert_eq!(program.items.len(), 5, "Should have 2 declarations and 3 assignments");

    // Count assignments
    let assignment_count = program.items.iter()
        .filter(|stmt| matches!(stmt, Stmt::Assignment { .. }))
        .count();
    assert_eq!(assignment_count, 3, "Should have 3 assignments");
}

#[test]
fn test_assignment_with_complex_expression() {
    let code = r#"
private result = 0
result = (10 + 5) * 2 - 3
"#;
    let program = parse_program(code).expect("Should parse");
    let expr = extract_assignment_expr(&program, "result").expect("Should find assignment");

    // Should parse as ((10 + 5) * 2) - 3
    if let Expr::Binary { op: BinOp::Sub, lhs, rhs } = expr {
        assert!(matches!(**lhs, Expr::Binary { op: BinOp::Mul, .. }));
        assert!(matches!(**rhs, Expr::Number(3.0)));
    } else {
        panic!("Expected subtraction at top level, got: {:?}", expr);
    }
}

#[test]
fn test_assignment_with_logical_operators() {
    let code = r#"
private flag = true
flag = true & false | true
"#;
    let program = parse_program(code).expect("Should parse");
    let expr = extract_assignment_expr(&program, "flag").expect("Should find assignment");

    // Should parse as (true & false) | true (& has higher precedence than |)
    if let Expr::Binary { op: BinOp::Or, lhs, rhs } = expr {
        assert!(matches!(**lhs, Expr::Binary { op: BinOp::And, .. }));
        assert!(matches!(**rhs, Expr::Bool(true)));
    } else {
        panic!("Expected OR at top level");
    }
}

#[test]
fn test_assignment_error_missing_value() {
    let code = "private x = 5\nx =";
    let result = parse_program(code);
    assert!(result.is_err(), "Should fail with missing value after '='");
}

#[test]
fn test_assignment_error_missing_equals() {
    let code = "private x = 5\nx 10";
    let result = parse_program(code);
    // This should parse as two separate expression statements, not an error
    assert!(result.is_ok(), "Should parse as separate statements");
}

#[test]
fn test_mixed_declarations_and_assignments() {
    let code = r#"
private a = 1
private b = 2
a = 3
private c = 4
b = 5
c = 6
"#;
    let program = parse_program(code).expect("Should parse mixed statements");
    assert_eq!(program.items.len(), 6);

    // Check the sequence
    assert!(matches!(program.items[0], Stmt::VarDecl { .. })); // a = 1
    assert!(matches!(program.items[1], Stmt::VarDecl { .. })); // b = 2
    assert!(matches!(program.items[2], Stmt::Assignment { .. })); // a = 3
    assert!(matches!(program.items[3], Stmt::VarDecl { .. })); // c = 4
    assert!(matches!(program.items[4], Stmt::Assignment { .. })); // b = 5
    assert!(matches!(program.items[5], Stmt::Assignment { .. })); // c = 6
}
