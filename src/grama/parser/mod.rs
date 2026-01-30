// Parser module - entry point for all parsing operations

mod parser;
mod statement_parser;
mod expression_parser;
mod operator;
mod array_parser;
mod function_parser;
mod control_flow_parser;
mod literal_parser;

// Re-export the main public API
pub use parser::{build_statements, parse_expression};
pub use operator::parse_binary_expression;
