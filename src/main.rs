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

    // Check for --debug and --module flags
    let mut debug_mode = false;
    let mut module_value: Option<String> = None;

    for arg in &args[2..] {
        if arg == "--debug" {
            debug_mode = true;
        } else if arg.starts_with("--module=") {
            module_value = Some(arg[9..].to_string()); // Skip "--module="
        }
    }

    if debug_mode {
        println!("Filename: {}", filename);
        if let Some(ref m) = module_value {
            println!("Module: {}", m);
        }
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

    let mut evaluator = grama::Evaluator::with_module(module_value);
    match evaluator.eval_program(&program) {
        Ok(outputs) => {
            // Print all output lines (from print() calls)
            for output in outputs {
                println!("{}", output);
            }

            // Print .env formatted public variables (if any)
            let env_output = evaluator.get_env_output();
            if !env_output.is_empty() {
                if debug_mode {
                    println!("\n# .env output:");
                }
                for line in env_output {
                    println!("{}", line);
                }
            }
        }
        Err(err) => {
            eprintln!("\nRuntime Error:");
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
