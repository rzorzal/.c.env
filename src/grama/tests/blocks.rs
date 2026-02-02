// Tests for block statements

#[cfg(test)]
mod block_statement_tests {
    use crate::grama::evaluator::Evaluator;
    use crate::grama::gramma_rules::Stmt;
    use crate::grama::parser::build_statements;
    use crate::lexing;

    #[test]
    fn test_empty_block() {
        let code = "{}";
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        assert_eq!(program.items.len(), 1);
        match &program.items[0] {
            Stmt::Block(stmts) => {
                assert_eq!(stmts.len(), 0, "Empty block should have no statements");
            }
            _ => panic!("Expected Block statement"),
        }
    }

    #[test]
    fn test_block_with_single_statement() {
        let code = r#"{
    x = 10
}"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        assert_eq!(program.items.len(), 1);
        match &program.items[0] {
            Stmt::Block(stmts) => {
                assert_eq!(stmts.len(), 1, "Block should have one statement");
            }
            _ => panic!("Expected Block statement"),
        }
    }

    #[test]
    fn test_block_with_multiple_statements() {
        let code = r#"{
    private x = 10
    y = x + 5
    print(y)
}"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        assert_eq!(program.items.len(), 1);
        match &program.items[0] {
            Stmt::Block(stmts) => {
                assert_eq!(stmts.len(), 3, "Block should have three statements");
            }
            _ => panic!("Expected Block statement"),
        }
    }

    #[test]
    fn test_nested_blocks() {
        let code = r#"{
    x = 1
    {
        y = 2
    }
}"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        assert_eq!(program.items.len(), 1);
        match &program.items[0] {
            Stmt::Block(outer_stmts) => {
                assert_eq!(
                    outer_stmts.len(),
                    2,
                    "Outer block should have two statements"
                );
                match &outer_stmts[1] {
                    Stmt::Block(inner_stmts) => {
                        assert_eq!(
                            inner_stmts.len(),
                            1,
                            "Inner block should have one statement"
                        );
                    }
                    _ => panic!("Expected nested Block statement"),
                }
            }
            _ => panic!("Expected Block statement"),
        }
    }

    #[test]
    fn test_block_execution_empty() {
        let code = "{}";
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok(), "Empty block should execute successfully");
    }

    #[test]
    fn test_block_execution_with_statements() {
        let code = r#"{
    private x = 10
    y = x + 5
}
print(y)"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(
            result.is_ok(),
            "Block with statements should execute successfully"
        );
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "15");
    }

    #[test]
    fn test_block_shares_scope() {
        // Variables defined inside block should be accessible outside
        let code = r#"{
    private x = 10
}
y = x + 5
print(y)"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(
            result.is_ok(),
            "Variables from block should be accessible outside"
        );
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "15");
    }

    #[test]
    fn test_block_modifies_outer_variables() {
        // Block should be able to modify variables from outer scope
        let code = r#"private x = 10
{
    x = 20
}
print(x)"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(
            result.is_ok(),
            "Block should be able to modify outer variables"
        );
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "20");
    }

    #[test]
    fn test_nested_blocks_share_scope() {
        let code = r#"{
    private x = 10
    {
        y = x + 5
    }
}
print(y)"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok(), "Nested blocks should share scope");
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "15");
    }

    #[test]
    fn test_block_with_expressions() {
        let code = r#"{
    private a = 2 * 3
    private b = a + 4
    result = b * 2
}
print(result)"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok(), "Block with expressions should execute");
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "20"); // (2*3 + 4) * 2 = 10 * 2 = 20
    }

    #[test]
    fn test_unmatched_brace_error() {
        let code = r#"{
    x = 10"#;
        let tokens = lexing::analyze_code(code);
        let result = build_statements(&tokens);

        assert!(result.is_err(), "Unmatched brace should cause error");
        let err = result.unwrap_err();
        let err_msg = format!("{}", err);
        assert!(
            err_msg.contains("Unmatched") || err_msg.contains("brace"),
            "Error should mention unmatched brace, got: {}",
            err_msg
        );
    }

    #[test]
    fn test_block_with_function_calls() {
        let code = r#"{
    private x = 10
    private y = 20
    print(x)
    print(y)
}"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok(), "Block with function calls should execute");
        let output = result.unwrap();
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "10");
        assert_eq!(output[1], "20");
    }

    #[test]
    fn test_deeply_nested_blocks() {
        let code = r#"{
    private a = 1
    {
        private b = 2
        {
            private c = 3
            result = a + b + c
        }
    }
}
print(result)"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok(), "Deeply nested blocks should execute");
        let output = result.unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "6"); // 1 + 2 + 3 = 6
    }

    #[test]
    fn test_block_preserves_public_variables() {
        // Public variables defined in blocks should be in .env output
        let code = r#"{
    DATABASE_URL = "postgres://localhost/db"
    API_KEY = "secret123"
}"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok(), "Block should preserve public variables");

        // Check that public variables were tracked
        let public_vars = evaluator.get_public_vars();
        assert_eq!(public_vars.len(), 2);
        assert!(public_vars.contains_key("DATABASE_URL"));
        assert!(public_vars.contains_key("API_KEY"));
    }

    #[test]
    fn test_block_with_private_and_public_vars() {
        let code = r#"{
    private temp = 100
    FINAL_VALUE = temp * 2
}"#;
        let tokens = lexing::analyze_code(code);
        let program = build_statements(&tokens).unwrap();

        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program);

        assert!(result.is_ok(), "Block with mixed visibility should execute");

        // Only public variable should be tracked
        let public_vars = evaluator.get_public_vars();
        assert_eq!(public_vars.len(), 1);
        assert!(public_vars.contains_key("FINAL_VALUE"));
        assert!(!public_vars.contains_key("temp"));
    }
}
