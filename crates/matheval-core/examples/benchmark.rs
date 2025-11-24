use matheval_core::{Compiler, Context};
use std::time::Instant;

fn main() {
    println!("=== Benchmark Simulation Example ===");
    println!("Simulating a scenario where we compile once and run millions of times.");

    // Complex expression
    let expr = "sin(x) * cos(y) + (x / y) ^ 2";
    println!("Expression: {}", expr);

    // 1. Compile (Expensive step, done once)
    let start_compile = Instant::now();
    let compiler = Compiler::new();
    let program = compiler.compile(expr).expect("Failed to compile");
    println!("Compilation took: {:?}", start_compile.elapsed());

    // 2. Prepare Context
    let mut context = Context::new();
    
    // 3. Run Loop
    let iterations = 1_000_000;
    println!("Running {} iterations...", iterations);
    
    let start_eval = Instant::now();
    let mut sum = 0.0;
    for i in 0..iterations {
        let val = i as f64 * 0.001;
        // In a real scenario, we might update variables in the context efficiently
        // Here we just set them.
        context.set("x", val);
        context.set("y", val + 1.0);
        
        // The VM execution is very fast because variable lookup is O(1) (index-based)
        // and bytecode is linear.
        let res = program.eval(&context).unwrap();
        sum += res;
    }
    let duration = start_eval.elapsed();
    
    println!("Total evaluation time: {:?}", duration);
    println!("Average time per evaluation: {:?}", duration / iterations as u32);
    println!("Check sum: {}", sum);
}
