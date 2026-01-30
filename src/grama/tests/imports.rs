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
        let program = build_statements(&tokens);

        // Should parse successfully (import is now a function)
        assert!(program.is_ok(), "Import with number should parse");

        // But should fail at runtime
        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program.unwrap());
        assert!(result.is_err(), "Import with non-string should fail at runtime");
        let err = result.unwrap_err();
        assert!(err.message.contains("string") || err.message.contains("expects"));
    }

    #[test]
    fn test_import_multiple_arguments() {
        let code = r#"import("file1.cenv", "file2.cenv")"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens);

        // Should parse successfully (import is now a function)
        assert!(program.is_ok(), "Import with multiple args should parse");

        // But should fail at runtime (wrong arg count)
        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program.unwrap());
        assert!(result.is_err(), "Import with wrong arg count should fail at runtime");
        let err = result.unwrap_err();
        assert!(err.message.contains("1") || err.message.contains("argument"));
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
        // Create a temporary file for testing - using PUBLIC variable
        std::fs::create_dir_all("test_imports_vars").ok();
        std::fs::write("test_imports_vars/temp_config.cenv", "X_VALUE = 42\n").unwrap();

        let code = r#"
import("test_imports_vars/temp_config.cenv")
print(X_VALUE)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
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
        // Create two files with PUBLIC variables
        std::fs::create_dir_all("test_imports_multi").ok();
        std::fs::write("test_imports_multi/file1.cenv", "A_VALUE = 10\n").unwrap();
        std::fs::write("test_imports_multi/file2.cenv", "B_VALUE = 20\n").unwrap();

        let code = r#"
import("test_imports_multi/file1.cenv")
import("test_imports_multi/file2.cenv")
private c = A_VALUE + B_VALUE
print(c)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
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
        std::fs::write("test_imports_expr/math.cenv", "PI = 3.14\nE = 2.71\n").unwrap();

        let code = r#"
import("test_imports_expr/math.cenv")
private circle_area = PI * 5 * 5
print(type(PI))
print(circle_area > 70)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
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
        std::fs::write("test_imports_shadow/shadowing.cenv", "X_VAR = 100\n").unwrap();

        let code = r#"
private x = 1
print("Before import:", x)
import("test_imports_shadow/shadowing.cenv")
print("After import:", X_VAR)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program);

        std::fs::remove_dir_all("test_imports_shadow").ok();

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "Before import: 1");
        assert_eq!(output[1], "After import: 100");
    }
}

#[cfg(test)]
mod import_object_tests {
    use super::*;

    #[test]
    fn test_import_returns_object() {
        std::fs::create_dir_all("test_import_obj").ok();
        std::fs::write("test_import_obj/config.cenv", "API_KEY = \"secret123\"\nAPI_URL = \"https://api.example.com\"\n").unwrap();

        let code = r#"
private config = import("test_import_obj/config.cenv")
print(type(config))
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program);

        std::fs::remove_dir_all("test_import_obj").ok();

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "object");
    }

    #[test]
    fn test_member_access_on_import() {
        std::fs::create_dir_all("test_member_access").ok();
        std::fs::write("test_member_access/db.cenv", "DATABASE_URL = \"postgresql://localhost/db\"\nDATABASE_PORT = 5432\n").unwrap();

        let code = r#"
private db = import("test_member_access/db.cenv")
DATABASE_URL = db.DATABASE_URL
print("URL:", db.DATABASE_URL)
print("Port:", db.DATABASE_PORT)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program);

        std::fs::remove_dir_all("test_member_access").ok();

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "URL: postgresql://localhost/db");
        assert_eq!(output[1], "Port: 5432");

        // Check that DATABASE_URL was added to public variables
        let env_output = evaluator.get_env_output();
        assert!(env_output.iter().any(|line| line.starts_with("DATABASE_URL=")));
    }

    #[test]
    fn test_import_only_public_vars() {
        std::fs::create_dir_all("test_public_only").ok();
        std::fs::write("test_public_only/vars.cenv",
            "PUBLIC_VAR = \"visible\"\nprivate PRIVATE_VAR = \"hidden\"\n").unwrap();

        let code = r#"
private vars = import("test_public_only/vars.cenv")
print("Length:", len(vars))
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program);

        std::fs::remove_dir_all("test_public_only").ok();

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "Length: 1"); // Only PUBLIC_VAR should be in the object
    }

    #[test]
    fn test_aws_secret_returns_object() {
        let code = r#"
private aws_vars = import_aws_secret("my-secret")
print("Type:", type(aws_vars))
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.len(), 2); // Note message + Type print
        assert!(output[0].contains("import_aws_secret"));
        assert_eq!(output[1], "Type: object");
    }

    #[test]
    fn test_member_access_error_on_non_object() {
        let code = r#"
private num = 42
print(num.field)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Cannot access field") || err.message.contains("non-object"));
    }

    #[test]
    fn test_member_access_nonexistent_field() {
        std::fs::create_dir_all("test_no_field").ok();
        std::fs::write("test_no_field/config.cenv", "KEY = \"value\"\n").unwrap();

        let code = r#"
private config = import("test_no_field/config.cenv")
print(config.NONEXISTENT)
"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::with_module(None);
        let result = evaluator.eval_program(&program);

        std::fs::remove_dir_all("test_no_field").ok();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("no field"));
    }
}
