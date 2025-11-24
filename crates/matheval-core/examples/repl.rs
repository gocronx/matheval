use matheval_core::{Compiler, Context};
use std::io::{self, Write};

fn main() {
    println!("=== matheval REPL ===");
    println!("Type 'exit' or 'quit' to leave.");
    println!("Supported syntax: +, -, *, /, ^, (, ), sin, cos, max, min, etc.");
    println!("Variables are supported (e.g., x, y), but currently context is reset per line in this simple REPL.");
    
    let compiler = Compiler::new();
    let mut context = Context::new();
    // Pre-populate some constants
    context.set("PI", std::f64::consts::PI);
    context.set("E", std::f64::consts::E);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "exit" || input == "quit" {
            break;
        }

        // In this simple REPL, we handle "var = val" manually for demonstration,
        // since the parser currently only supports expressions, not assignments.
        if let Some((var, val_str)) = input.split_once('=') {
            let var = var.trim();
            let val_str = val_str.trim();
            match val_str.parse::<f64>() {
                Ok(val) => {
                    context.set(var, val);
                    println!("{} = {}", var, val);
                }
                Err(_) => println!("Error: Invalid number format for assignment"),
            }
            continue;
        }

        match compiler.compile(input) {
            Ok(program) => match program.eval(&context) {
                Ok(result) => println!("= {}", result),
                Err(e) => println!("Runtime Error: {}", e),
            },
            Err(e) => println!("Syntax Error: {}", e),
        }
    }
}
