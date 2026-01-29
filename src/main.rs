mod lexing;
mod grama;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        eprintln!("Example: {} script.cenv", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    // Read the file
    let contents: String = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    // Check for --debug flag
    let debug_mode = args.len() > 2 && args[2] == "--debug";

    if debug_mode {
        println!("Filename: {}", filename);
        println!("File contents:\n{}\n", contents);
        println!("Analyzing code...\n");
    }

    // Lex the code
    let tokens: Vec<lexing::Token> = lexing::analyze_code(&contents);

    if debug_mode {
        for token in &tokens {
            println!("{:?}", token);
        }
        println!("\nBuilding statements...\n");
    }

    // Parse the code
    let program = match grama::build_statements(&tokens) {
        Ok(program) => {
            if debug_mode {
                println!("Program: {:#?}\n", program);
            }
            program
        }
        Err(err) => {
            eprintln!("\nParse Error:");
            eprintln!("{}", err.format_with_source(&contents));
            std::process::exit(1);
        }
    };

    // Evaluate the program
    if debug_mode {
        println!("Executing program...\n");
    }

    let mut evaluator = grama::Evaluator::new();
    match evaluator.eval_program(&program) {
        Ok(outputs) => {
            // Print all output lines (from print() calls)
            for output in outputs {
                println!("{}", output);
            }
        }
        Err(err) => {
            eprintln!("\nRuntime Error:");
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
