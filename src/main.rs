mod lexing;
mod grama;

use std::env;
use std::fs;
use std::io::Write;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("C.env - A .env file compiler");
    println!();
    println!("USAGE:");
    println!("    cenv <file> [OPTIONS]");
    println!();
    println!("ARGS:");
    println!("    <file>              .cenv source file to compile");
    println!();
    println!("OPTIONS:");
    println!("    --module=<value>    Set the module variable for dynamic imports");
    println!("                        Example: --module=production");
    println!();
    println!("    --output=<file>     Specify output filename (default: .env)");
    println!("                        Example: --output=.env.production");
    println!();
    println!("    --dry               Dry run mode - output to stdout instead of .env file");
    println!();
    println!("    --debug             Enable debug mode (show tokens and AST)");
    println!();
    println!("    -h, --help          Print help information");
    println!("    -v, --version       Print version information");
    println!();
    println!("EXAMPLES:");
    println!("    cenv config.cenv --module=production       # Generates .env");
    println!("    cenv config.cenv --output=.env.production  # Generates .env.production");
    println!("    cenv config.cenv --output=.env.custom      # Generates .env.custom");
    println!("    cenv config.cenv --dry                     # Output to stdout");
    println!("    cenv script.cenv --debug --dry             # Debug + dry run");
    println!();
    println!("OUTPUT:");
    println!("    By default, generates .env file");
    println!("    Use --output to specify a custom filename");
    println!("    Use --dry to output to stdout instead");
    println!();
    println!("For more information, visit: https://github.com/yourusername/c.env.lang");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle no arguments
    if args.len() < 2 {
        eprintln!("Error: No input file specified");
        eprintln!();
        eprintln!("Usage: cenv <file> [OPTIONS]");
        eprintln!("Try 'cenv --help' for more information.");
        std::process::exit(1);
    }

    // Handle --help, -h, --version, -v flags
    let first_arg = &args[1];
    if first_arg == "--help" || first_arg == "-h" {
        print_help();
        return;
    }
    if first_arg == "--version" || first_arg == "-v" {
        println!("cenv {}", VERSION);
        return;
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

    // Check for --debug, --module, --output, and --dry flags
    let mut debug_mode = false;
    let mut dry_run = false;
    let mut module_value: Option<String> = None;
    let mut output_file: Option<String> = None;

    for arg in &args[2..] {
        if arg == "--debug" {
            debug_mode = true;
        } else if arg == "--dry" {
            dry_run = true;
        } else if arg.starts_with("--module=") {
            module_value = Some(arg[9..].to_string()); // Skip "--module="
        } else if arg.starts_with("--output=") {
            output_file = Some(arg[9..].to_string()); // Skip "--output="
        } else if arg == "--help" || arg == "-h" {
            print_help();
            return;
        } else if arg == "--version" || arg == "-v" {
            println!("cenv {}", VERSION);
            return;
        } else {
            eprintln!("Warning: Unknown option '{}'", arg);
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

    let mut evaluator = grama::Evaluator::with_module(module_value.clone());
    match evaluator.eval_program(&program) {
        Ok(outputs) => {
            // In dry run mode, output everything to stdout
            if dry_run {
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
            } else {
                // Default mode: write to .env file
                let env_output = evaluator.get_env_output();

                if !env_output.is_empty() {
                    // Determine output filename (priority: --output > default .env)
                    let output_filename = if let Some(ref custom) = output_file {
                        custom.clone()
                    } else {
                        ".env".to_string()
                    };

                    // Write to file
                    match fs::File::create(&output_filename) {
                        Ok(mut file) => {
                            for line in &env_output {
                                if let Err(e) = writeln!(file, "{}", line) {
                                    eprintln!("Error writing to '{}': {}", output_filename, e);
                                    std::process::exit(1);
                                }
                            }
                            eprintln!("✓ Generated {}", output_filename);
                        }
                        Err(e) => {
                            eprintln!("Error creating file '{}': {}", output_filename, e);
                            std::process::exit(1);
                        }
                    }
                }

                // Always print print() outputs to stdout
                for output in outputs {
                    println!("{}", output);
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
