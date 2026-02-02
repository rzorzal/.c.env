// Parser module - entry point for all parsing operations

mod array_parser;
mod control_flow_parser;
mod core;
mod expression_parser;
mod function_parser;
mod literal_parser;
pub mod operator;
mod statement_parser;

// Re-export the main public API
pub use core::build_statements;
