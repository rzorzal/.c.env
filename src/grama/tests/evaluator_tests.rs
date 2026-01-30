use crate::grama::{Evaluator, RuntimeError};
use crate::lexing;
use crate::grama::build_statements;

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

#[cfg(test)]
mod print_tests {
    use super::*;

    #[test]
    fn test_print_number() {
        let code = r#"
private x = 42
print(x)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "42");
    }

    #[test]
    fn test_print_string() {
        let code = r#"
private msg = "Hello, World!"
print(msg)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "Hello, World!");
    }

    #[test]
    fn test_print_boolean() {
        let code = r#"
private flag = true
print(flag)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "true");
    }

    #[test]
    fn test_print_null() {
        let code = r#"
private nothing = null
print(nothing)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "null");
    }

    #[test]
    fn test_print_expression() {
        let code = r#"
private a = 10
private b = 5
print(a + b)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "15");
    }

    #[test]
    fn test_print_multiple_args() {
        let code = r#"
private x = 10
private y = 20
print(x, y, 30)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "10 20 30");
    }

    #[test]
    fn test_print_no_args() {
        let code = "print()";
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "");
    }

    #[test]
    fn test_multiple_prints() {
        let code = r#"
print(1)
print(2)
print(3)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 3);
        assert_eq!(output[0], "1");
        assert_eq!(output[1], "2");
        assert_eq!(output[2], "3");
    }

    #[test]
    fn test_print_mixed_types() {
        let code = r#"
print(42, "hello", true, null)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "42 hello true null");
    }

    #[test]
    fn test_print_complex_expression() {
        let code = r#"
private x = 5
print((x + 3) * 2)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "16");
    }
}

#[cfg(test)]
mod len_tests {
    use super::*;

    #[test]
    fn test_len_string() {
        let code = r#"
private text = "hello"
private length = len(text)
print(length)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "5");
    }

    #[test]
    fn test_len_empty_string() {
        let code = r#"
private empty = ""
print(len(empty))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "0");
    }

    #[test]
    fn test_len_wrong_type() {
        let code = r#"
private num = 42
len(num)
"#;
        let err = eval_code_err(code);
        assert!(err.contains("Type error"));
        assert!(err.contains("string, array, or object"));
    }

    #[test]
    fn test_len_wrong_arg_count() {
        let code = r#"
len("a", "b")
"#;
        let err = eval_code_err(code);
        assert!(err.contains("expects 1 argument"));
    }
}

#[cfg(test)]
mod type_tests {
    use super::*;

    #[test]
    fn test_type_number() {
        let code = r#"
private x = 42
print(type(x))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "number");
    }

    #[test]
    fn test_type_string() {
        let code = r#"
private x = "hello"
print(type(x))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "string");
    }

    #[test]
    fn test_type_boolean() {
        let code = r#"
private x = true
print(type(x))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "boolean");
    }

    #[test]
    fn test_type_null() {
        let code = r#"
private x = null
print(type(x))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "null");
    }
}

#[cfg(test)]
mod str_tests {
    use super::*;

    #[test]
    fn test_str_number() {
        let code = r#"
private x = 42
print(str(x))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "42");
    }

    #[test]
    fn test_str_boolean() {
        let code = r#"
print(str(true))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "true");
    }

    #[test]
    fn test_str_null() {
        let code = r#"
print(str(null))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "null");
    }
}

#[cfg(test)]
mod num_tests {
    use super::*;

    #[test]
    fn test_num_string() {
        let code = r#"
private x = "42"
print(num(x))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "42");
    }

    #[test]
    fn test_num_string_float() {
        let code = r#"
print(num("3.14"))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "3.14");
    }

    #[test]
    fn test_num_boolean_true() {
        let code = r#"
print(num(true))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "1");
    }

    #[test]
    fn test_num_boolean_false() {
        let code = r#"
print(num(false))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "0");
    }

    #[test]
    fn test_num_null() {
        let code = r#"
print(num(null))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "0");
    }

    #[test]
    fn test_num_invalid_string() {
        let code = r#"
num("hello")
"#;
        let err = eval_code_err(code);
        assert!(err.contains("Cannot convert"));
    }
}

#[cfg(test)]
mod bool_tests {
    use super::*;

    #[test]
    fn test_bool_number_nonzero() {
        let code = r#"
print(bool(42))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "true");
    }

    #[test]
    fn test_bool_number_zero() {
        let code = r#"
print(bool(0))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "false");
    }

    #[test]
    fn test_bool_string_nonempty() {
        let code = r#"
print(bool("hello"))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "true");
    }

    #[test]
    fn test_bool_string_empty() {
        let code = r#"
print(bool(""))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "false");
    }

    #[test]
    fn test_bool_null() {
        let code = r#"
print(bool(null))
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "false");
    }
}

#[cfg(test)]
mod evaluator_integration_tests {
    use super::*;

    #[test]
    fn test_variable_declaration_and_use() {
        let code = r#"
private x = 10
private y = 20
private z = x + y
print(z)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "30");
    }

    #[test]
    fn test_assignment_and_use() {
        let code = r#"
private x = 10
x = 20
print(x)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "20");
    }

    #[test]
    fn test_undefined_variable() {
        let code = r#"
print(x)
"#;
        let err = eval_code_err(code);
        assert!(err.contains("Undefined variable"));
        assert!(err.contains("'x'"));
    }

    #[test]
    fn test_assign_creates_public_variable() {
        // In .env context, `name = value` creates a public variable
        let code = r#"
X_VAR = 10
"#;
        let mut evaluator = Evaluator::with_module(None);
        let tokens = crate::lexing::analyze_code(code);
        let program = crate::grama::build_statements(&tokens).expect("Should parse");
        let result = evaluator.eval_program(&program);
        assert!(result.is_ok(), "Should succeed");

        // Check that it's in the public variables
        let env_output = evaluator.get_env_output();
        assert!(env_output.iter().any(|line| line.starts_with("X_VAR=")));
    }

    #[test]
    fn test_arithmetic_operations() {
        let code = r#"
print(10 + 5)
print(10 - 5)
print(10 * 5)
print(10 / 5)
print(10 % 3)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 5);
        assert_eq!(output[0], "15");
        assert_eq!(output[1], "5");
        assert_eq!(output[2], "50");
        assert_eq!(output[3], "2");
        assert_eq!(output[4], "1");
    }

    #[test]
    fn test_comparison_operations() {
        let code = r#"
print(10 > 5)
print(10 < 5)
print(10 >= 10)
print(10 <= 5)
print(10 == 10)
print(10 != 5)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 6);
        assert_eq!(output[0], "true");
        assert_eq!(output[1], "false");
        assert_eq!(output[2], "true");
        assert_eq!(output[3], "false");
        assert_eq!(output[4], "true");
        assert_eq!(output[5], "true");
    }

    #[test]
    fn test_logical_operations() {
        let code = r#"
print(true & true)
print(true & false)
print(false | true)
print(false | false)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output.len(), 4);
        assert_eq!(output[0], "true");
        assert_eq!(output[1], "false");
        assert_eq!(output[2], "true");
        assert_eq!(output[3], "false");
    }

    #[test]
    fn test_complex_expression() {
        let code = r#"
private a = 10
private b = 5
private c = 2
print((a + b) * c - 3)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "27");
    }

    #[test]
    fn test_string_concatenation() {
        let code = r#"
private first = "Hello"
private second = "World"
print(first + " " + second)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "Hello World");
    }

    #[test]
    fn test_type_conversion_workflow() {
        let code = r#"
private x = "42"
private num_x = num(x)
private doubled = num_x * 2
print(doubled)
"#;
        let output = eval_code(code).expect("Should evaluate");
        assert_eq!(output[0], "84");
    }
}
