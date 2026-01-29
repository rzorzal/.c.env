// Phase 1.1: Error Handling Tests
// Tests for parser error detection and reporting

use super::helpers::*;

#[test]
fn test_error_unmatched_parenthesis() {
    let code = "private x = (1 + 2";
    let result = parse_program(code);
    assert!(result.is_err(), "Should fail with unmatched parenthesis");
    let err = result.unwrap_err();
    assert!(err.contains("')'")); // Error should mention missing closing paren
}

#[test]
fn test_error_unmatched_bracket() {
    let code = "private x = [1, 2, 3";
    let result = parse_program(code);
    assert!(result.is_err(), "Should fail with unmatched bracket");
    let err = result.unwrap_err();
    assert!(err.contains("']'")); // Error should mention missing closing bracket
}

#[test]
fn test_successful_parse_with_parens() {
    let code = "private x = (1 + 2)";
    let result = parse_program(code);
    assert!(result.is_ok(), "Should parse correctly with matched parens");
}
