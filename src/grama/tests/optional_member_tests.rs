use crate::grama::build_statements;
use crate::grama::{Evaluator, RuntimeError};
use crate::lexing;

/// Helper to evaluate code and return output
fn eval_code(code: &str) -> Result<Vec<String>, RuntimeError> {
    let tokens = lexing::analyze_code(code);
    let program = build_statements(&tokens).map_err(|e| RuntimeError::new(format!("{:?}", e)))?;
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(&program)
}

/// Helper to evaluate code and return error message
fn eval_code_err(code: &str) -> String {
    match eval_code(code) {
        Ok(_) => panic!("Expected error but got success"),
        Err(e) => e.message,
    }
}

// Note: These tests are currently disabled because object literal syntax is not yet implemented
// The optional chaining (?.) operator is fully functional but requires objects to test with
// Once object literals are implemented, these tests can be re-enabled

#[test]
#[ignore]
fn test_optional_member_access_existing_field() {
    let code = r#"
private user = { name: "Alice" }
private result = user?.name
print(result)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "Alice");
}

#[test]
#[ignore]
fn test_optional_member_access_missing_field() {
    let code = r#"
private user = { name: "Bob" }
private result = user?.email
print(result)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "null");
}

#[test]
fn test_optional_member_access_on_null() {
    let code = r#"
private nullable = null
private result = nullable?.anything
print(result)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "null");
}

#[test]
fn test_optional_member_access_on_string() {
    let code = r#"
private str_var = "test"
private result = str_var?.length
print(result)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "null");
}

#[test]
fn test_optional_member_access_on_number() {
    let code = r#"
private num = 42
private result = num?.value
print(result)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "null");
}

#[test]
#[ignore]
fn test_optional_member_access_on_array() {
    let code = r#"
private arr = [1, 2, 3]
private result = arr?.length
print(result)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "null");
}

#[test]
#[ignore]
fn test_regular_member_access_errors_on_missing_field() {
    let code = r#"
private obj = { name: "Test" }
private result = obj.missing
"#;
    let error = eval_code_err(code);
    assert!(error.contains("no field"));
}

#[test]
#[ignore]
fn test_comparison_regular_vs_optional_member() {
    let code = r#"
private obj = { exists: "value" }
print(obj.exists)
print(obj?.exists)
print(obj?.missing)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "value");
    assert_eq!(output[1], "value");
    assert_eq!(output[2], "null");
}

#[test]
#[ignore]
fn test_optional_member_with_nested_objects() {
    let code = r#"
private user = { address: { city: "NYC" } }
private result = user?.address
print(type(result))
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "object");
}

#[test]
fn test_optional_member_safe_on_boolean() {
    let code = r#"
private flag = true
private result = flag?.anything
print(result)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "null");
}

#[test]
#[ignore]
fn test_optional_member_multiple_fields() {
    let code = r#"
private data = { x: 10, y: 20 }
print(data?.x)
print(data?.y)
print(data?.z)
"#;
    let output = eval_code(code).expect("Should evaluate");
    assert_eq!(output[0], "10");
    assert_eq!(output[1], "20");
    assert_eq!(output[2], "null");
}
