// Parser module - entry point for all parsing operations

mod core;
mod statement_parser;
mod expression_parser;
pub mod operator;
mod array_parser;
mod function_parser;
mod control_flow_parser;
mod literal_parser;

// Re-export the main public API
pub use core::build_statements;
