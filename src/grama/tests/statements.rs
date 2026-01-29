// Basic Statement Tests
// Tests for variable declarations and general statement parsing

use super::helpers::*;
use crate::grama::gramma_rules::{Expr, Stmt};

#[test]
fn test_simple_var_declaration() {
    let code = "private x = 42";
    let program = parse_program(code).expect("Should parse");
    assert_eq!(program.items.len(), 1);

    if let Stmt::VarDecl { private_, name, value } = &program.items[0] {
        assert_eq!(*private_, true);
        assert_eq!(name, "x");
        assert!(matches!(value, Expr::Number(42.0)));
    } else {
        panic!("Expected VarDecl");
    }
}

#[test]
fn test_string_literal() {
    let code = r#"private s = "hello world""#;
    let program = parse_program(code).expect("Should parse");
    let expr = extract_var_expr(&program, "s").expect("Should find variable");

    if let Expr::StringLiteral(s) = expr {
        assert_eq!(s, "hello world");
    } else {
        panic!("Expected StringLiteral");
    }
}

#[test]
fn test_boolean_literals() {
    let code = "private t = true\nprivate f = false";
    let program = parse_program(code).expect("Should parse");

    let t_expr = extract_var_expr(&program, "t").expect("Should find 't'");
    let f_expr = extract_var_expr(&program, "f").expect("Should find 'f'");

    assert!(matches!(t_expr, Expr::Bool(true)));
    assert!(matches!(f_expr, Expr::Bool(false)));
}

#[test]
fn test_multiple_statements() {
    let code = r#"
private a = 1
private b = 2
private c = 3
"#;
    let program = parse_program(code).expect("Should parse");
    assert_eq!(program.items.len(), 3, "Should parse 3 statements");
}
