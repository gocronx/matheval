use matheval_core::{Compiler, Context};

fn main() {
    println!("=== Simple Usage Example ===");

    // 1. Define an expression
    let expr = "x + y * 2";
    println!("Expression: {}", expr);

    // 2. Compile it
    let compiler = Compiler::new();
    let program = compiler.compile(expr).expect("Failed to compile");

    // 3. Create a context with variables
    let mut context = Context::new();
    context.set("x", 10.0);
    context.set("y", 5.0);
    println!("Context: x = 10.0, y = 5.0");

    // 4. Evaluate
    let result = program.eval(&context).expect("Failed to evaluate");
    println!("Result: {}", result);
    
    assert_eq!(result, 20.0);
}
