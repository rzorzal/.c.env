use crate::grama::Evaluator;
use crate::grama::build_statements;
use crate::lexing;

/// Helper to evaluate code and return .env output
fn eval_code_env(code: &str) -> Vec<String> {
    let tokens = lexing::analyze_code(code);
    let program = build_statements(&tokens).expect("Failed to parse");
    let mut evaluator = Evaluator::with_module(None);
    evaluator
        .eval_program(&program)
        .expect("Failed to evaluate");
    evaluator.get_env_output()
}

#[test]
fn test_hash_comments_preserved_in_env() {
    let source = r#"
# Database configuration
DB_HOST = "localhost"
DB_PORT = "5432"

// This line comment should NOT appear
DB_USER = "admin"
# User password
DB_PASS = "secret"
"#;

    let env_output = eval_code_env(source);

    // Verify that # comments are preserved
    assert!(env_output.contains(&"# Database configuration".to_string()));
    assert!(env_output.contains(&"# User password".to_string()));

    // Verify that // comments are NOT in the output
    assert!(
        !env_output
            .iter()
            .any(|line| line.contains("This line comment should NOT appear"))
    );

    // Verify variables are present
    assert!(env_output.iter().any(|line| line.starts_with("DB_HOST=")));
    assert!(env_output.iter().any(|line| line.starts_with("DB_PORT=")));
    assert!(env_output.iter().any(|line| line.starts_with("DB_USER=")));
    assert!(env_output.iter().any(|line| line.starts_with("DB_PASS=")));
}

#[test]
fn test_hash_comments_ordering() {
    let source = r#"
# First comment
VAR1 = "value1"
# Second comment
VAR2 = "value2"
# Third comment
"#;

    let env_output = eval_code_env(source);

    // The order should be: First comment, VAR1, Second comment, VAR2, Third comment
    let expected = vec![
        "# First comment",
        "VAR1=value1",
        "# Second comment",
        "VAR2=value2",
        "# Third comment",
    ];

    assert_eq!(env_output, expected);
}

#[test]
fn test_private_vars_dont_affect_comment_order() {
    let source = r#"
# Public variable
PUBLIC = "public"
# Private variable comment
private PRIVATE = "private"
# Another public
PUBLIC2 = "public2"
"#;

    let env_output = eval_code_env(source);

    // Comments for private variables are still preserved
    assert!(env_output.contains(&"# Public variable".to_string()));
    assert!(env_output.contains(&"# Private variable comment".to_string()));
    assert!(env_output.contains(&"# Another public".to_string()));

    // But private variable itself is not in output
    assert!(!env_output.iter().any(|line| line.starts_with("PRIVATE=")));

    // Public variables are present
    assert!(env_output.iter().any(|line| line.starts_with("PUBLIC=")));
    assert!(env_output.iter().any(|line| line.starts_with("PUBLIC2=")));
}

#[test]
fn test_multiline_comments_not_preserved() {
    let source = r#"
/* This is a
   multiline comment
   that should NOT appear */
VAR = "value"
# This should appear
"#;

    let env_output = eval_code_env(source);

    // Multiline comments should NOT be in output
    assert!(
        !env_output
            .iter()
            .any(|line| line.contains("multiline comment"))
    );

    // But # comment should be
    assert!(env_output.contains(&"# This should appear".to_string()));

    // Variable should be present
    assert!(env_output.iter().any(|line| line.starts_with("VAR=")));
}
