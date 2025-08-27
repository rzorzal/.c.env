mod lexing;
mod grama;

use std::env;
use std::fs;
use grama::Program;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let filename = &args[1];
    println!("Filename: {}", filename);
    let contents: String = fs::read_to_string(filename).expect("Failed to read file");
    println!("File contents:\n{}", contents);

    println!("\nAnalyzing code...\n");
    let tokens: Vec<lexing::Token> = lexing::analyze_code(&contents);
    for token in &tokens {
        println!("{:?}", token);
    }
    println!("\nBuilding statements...\n");
    let program: Option<Program> = grama::build_statements(&tokens);
    println!("Program: {:#?}", program);
}
