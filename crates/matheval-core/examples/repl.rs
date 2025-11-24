use matheval_core::Compiler;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    println!("=== matheval REPL ===");
    println!("Type 'exit' or 'quit' to leave.");
    println!("Supported syntax: +, -, *, /, ^, (, ), sin, cos, max, min, etc.");
    println!("Variables: assign with 'x = 5', use in expressions like 'x * 2'");
    
    let compiler = Compiler::new();
    
    // Store variables in a HashMap for the REPL
    let mut variables: HashMap<String, f64> = HashMap::new();
    variables.insert("PI".to_string(), std::f64::consts::PI);
    variables.insert("E".to_string(), std::f64::consts::E);

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

        // Handle variable assignment: var = value
        if let Some((var, val_str)) = input.split_once('=') {
            let var = var.trim();
            let val_str = val_str.trim();
            match val_str.parse::<f64>() {
                Ok(val) => {
                    variables.insert(var.to_string(), val);
                    println!("{} = {}", var, val);
                }
                Err(_) => println!("Error: Invalid number format for assignment"),
            }
            continue;
        }

        // Compile and evaluate expression
        match compiler.compile(input) {
            Ok(program) => {
                // Create context from current variables
                let mut context = program.create_context();
                
                // Map variables to their indices
                for (idx, var_name) in program.var_names.iter().enumerate() {
                    if let Some(&value) = variables.get(var_name) {
                        context.set_by_index(idx, value);
                    } else {
                        println!("Error: Undefined variable '{}'", var_name);
                        continue;
                    }
                }
                
                match program.eval(&context) {
                    Ok(result) => println!("= {}", result),
                    Err(e) => println!("Runtime Error: {}", e),
                }
            },
            Err(e) => println!("Syntax Error: {}", e),
        }
    }
}
