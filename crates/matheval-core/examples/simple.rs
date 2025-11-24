use matheval_core::Compiler;

fn main() {
    println!("=== Simple Usage Example ===");

    // 1. Define an expression
    let expr = "x + y * 2";
    println!("Expression: {}", expr);

    // 2. Compile it
    let compiler = Compiler::new();
    let program = compiler.compile(expr).expect("Failed to compile");

    // 3. Create a context optimized for this program
    let mut context = program.create_context();
    
    // Use indexed access for best performance (O(1))
    context.set_by_index(0, 10.0); // x
    context.set_by_index(1, 5.0);  // y
    println!("Context: x = 10.0, y = 5.0");

    // 4. Evaluate
    let result = program.eval(&context).expect("Failed to evaluate");
    println!("Result: {}", result);
    
    assert_eq!(result, 20.0);
}
