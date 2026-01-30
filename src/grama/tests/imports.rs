// Tests for import statement functionality

use crate::grama::{Evaluator, build_statements};
use crate::lexing;

#[cfg(test)]
mod import_statement_tests {
    use super::*;

    #[test]
    fn test_import_statement_parsing() {
        let code = r#"import("file.cenv")"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens);

        assert!(program.is_ok(), "Import statement should parse successfully");
        let program = program.unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_import_aws_secret_parsing() {
        let code = r#"import_aws_secret("my-secret")"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens);

        assert!(program.is_ok(), "import_aws_secret should parse successfully");
        let program = program.unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_import_missing_parentheses() {
        let code = r#"import "file.cenv""#;
        let tokens = lexing::analyze_code(code);
        let result = build_statements(&tokens);

        assert!(result.is_err(), "Import without parentheses should fail");
    }

    #[test]
    fn test_import_missing_argument() {
        let code = r#"import()"#;
        let tokens = lexing::analyze_code(code);
        let result = build_statements(&tokens);

        assert!(result.is_err(), "Import without argument should fail");
    }

    #[test]
    fn test_import_non_string_argument() {
        let code = r#"import(123)"#;
        let tokens = lexing::analyze_code(code);
        let result = build_statements(&tokens);

        assert!(result.is_err(), "Import with non-string argument should fail");
    }

    #[test]
    fn test_import_multiple_arguments() {
        let code = r#"import("file1.cenv", "file2.cenv")"#;
        let tokens = lexing::analyze_code(code);
        let result = build_statements(&tokens);

        assert!(result.is_err(), "Import with multiple arguments should fail");
    }
}

#[cfg(test)]
mod import_execution_tests {
    use super::*;

    #[test]
    fn test_import_file_not_found() {
        let code = r#"import("nonexistent.cenv")"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_err(), "Import of nonexistent file should fail");
        let err = result.unwrap_err();
        assert!(err.message.contains("Failed to resolve") || err.message.contains("Failed to read"),
                "Error should mention file failure, got: {}", err.message);
    }

    #[test]
    fn test_import_aws_secret_placeholder() {
        let code = r#"import_aws_secret("my-secret")"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok(), "AWS secret import should succeed with placeholder");
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("import_aws_secret"), "Output should mention AWS secret");
    }

    #[test]
    fn test_import_variables_accessible() {
        // Create a temporary file for testing
        std::fs::create_dir_all("test_imports_vars").ok();
        std::fs::write("test_imports_vars/temp_config.cenv", "private x = 42\n").unwrap();

        let code = r#"
import("test_imports_vars/temp_config.cenv")
print(x)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        // Clean up
        std::fs::remove_dir_all("test_imports_vars").ok();

        assert!(result.is_ok(), "Import should work and make variables accessible");
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "42");
    }

    #[test]
    fn test_import_executes_code() {
        // Create a temporary file that prints something
        std::fs::create_dir_all("test_imports_exec").ok();
        std::fs::write("test_imports_exec/temp_print.cenv", "print(\"Hello from import\")\n").unwrap();

        let code = r#"import("test_imports_exec/temp_print.cenv")"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        // Clean up
        std::fs::remove_dir_all("test_imports_exec").ok();

        assert!(result.is_ok(), "Import should execute imported file's code");
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "Hello from import");
    }

    #[test]
    fn test_circular_import_detection() {
        // Create two files that import each other
        std::fs::create_dir_all("test_imports_circular").ok();
        std::fs::write("test_imports_circular/file_a.cenv", "import(\"file_b.cenv\")\n").unwrap();
        std::fs::write("test_imports_circular/file_b.cenv", "import(\"file_a.cenv\")\n").unwrap();

        let code = r#"import("test_imports_circular/file_a.cenv")"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        let is_err = result.is_err();
        let err_msg = if let Err(ref err) = result {
            err.message.clone()
        } else {
            String::new()
        };

        // Clean up
        std::fs::remove_dir_all("test_imports_circular").ok();

        assert!(is_err, "Circular imports should be detected, got: {:?}", result);
        assert!(err_msg.contains("Circular import"),
                "Error should mention circular import, got: {}", err_msg);
    }

    #[test]
    fn test_multiple_imports() {
        // Create two files
        std::fs::create_dir_all("test_imports_multi").ok();
        std::fs::write("test_imports_multi/file1.cenv", "private a = 10\n").unwrap();
        std::fs::write("test_imports_multi/file2.cenv", "private b = 20\n").unwrap();

        let code = r#"
import("test_imports_multi/file1.cenv")
import("test_imports_multi/file2.cenv")
private c = a + b
print(c)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        let is_ok = result.is_ok();
        let err_msg = if let Err(ref e) = result {
            e.message.clone()
        } else {
            String::new()
        };

        // Clean up
        std::fs::remove_dir_all("test_imports_multi").ok();

        assert!(is_ok, "Multiple imports should work, got error: {}", err_msg);
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "30");
    }
}

#[cfg(test)]
mod import_integration_tests {
    use super::*;

    #[test]
    fn test_import_with_expressions() {
        std::fs::create_dir_all("test_imports_expr").ok();
        std::fs::write("test_imports_expr/math.cenv", "private PI = 3.14\nprivate E = 2.71\n").unwrap();

        let code = r#"
import("test_imports_expr/math.cenv")
private circle_area = PI * 5 * 5
print(type(PI))
print(circle_area > 70)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        std::fs::remove_dir_all("test_imports_expr").ok();

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "number");
        assert_eq!(output[1], "true"); // 3.14 * 5 * 5 = 78.5 > 70
    }

    #[test]
    fn test_import_shadowing() {
        std::fs::create_dir_all("test_imports_shadow").ok();
        std::fs::write("test_imports_shadow/shadowing.cenv", "private x = 100\n").unwrap();

        let code = r#"
private x = 1
print("Before import:", x)
import("test_imports_shadow/shadowing.cenv")
print("After import:", x)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        std::fs::remove_dir_all("test_imports_shadow").ok();

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "Before import: 1");
        assert_eq!(output[1], "After import: 100"); // Import overwrites
    }
}
