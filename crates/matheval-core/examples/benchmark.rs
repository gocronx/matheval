use matheval_core::Compiler;
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

    // 2. Prepare Context (pre-sized for optimal performance)
    let mut context = program.create_context();
    
    // 3. Run Loop
    let iterations = 1_000_000;
    println!("Running {} iterations...", iterations);
    
    let start_eval = Instant::now();
    let mut sum = 0.0;
    for i in 0..iterations {
        let val = i as f64 * 0.001;
        
        // Use indexed access for maximum performance (O(1), no hashing)
        context.set_by_index(0, val);       // x
        context.set_by_index(1, val + 1.0); // y
        
        // The VM execution is very fast because:
        // - Variable lookup is O(1) (direct array indexing)
        // - Bytecode is compact and cache-friendly
        // - Function calls use function pointers (no string matching)
        let res = program.eval(&context).unwrap();
        sum += res;
    }
    let duration = start_eval.elapsed();
    
    println!("Total evaluation time: {:?}", duration);
    println!("Average time per evaluation: {:?}", duration / iterations as u32);
    println!("Throughput: {:.2} million evals/sec", iterations as f64 / duration.as_secs_f64() / 1_000_000.0);
    println!("Check sum: {}", sum);
}
