use matheval_core::Compiler;
use std::time::Instant;

// Simple Xorshift for fair comparison (avoiding heavy rand crate dependency overhead in benchmark)
struct Xorshift { state: u64 }
impl Xorshift {
    fn next_f64(&mut self) -> f64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x as f64) / (u64::MAX as f64)
    }
    fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64();
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

fn main() {
    println!("=== Benchmark: Rust 'matheval' ===");
    println!("Scenario: Evaluating a user-provided formula 1,000,000 times.");

    let formula = "max(S_det * (E ^ (vol_part * Z)) - K, 0) * discount";
    
    // Parameters
    let s: f64 = 100.0;
    let k: f64 = 105.0;
    let t: f64 = 1.0;
    let r: f64 = 0.05;
    let sigma: f64 = 0.2;

    let discount = (-r * t).exp();
    let drift = (r - 0.5 * sigma.powi(2)) * t;
    let vol_part = sigma * t.sqrt();
    let s_det = s * drift.exp();

    // 1. Compile (Once)
    let compiler = Compiler::new();
    let program = compiler.compile(formula).expect("Compile failed");

    let mut context = program.create_context();
    // Use indexed access for maximum performance
    context.set_by_index(0, discount);
    context.set_by_index(1, std::f64::consts::E);
    context.set_by_index(2, k);
    context.set_by_index(3, s_det);
    context.set_by_index(4, vol_part);

    let iterations = 1_000_000;
    let mut rng = Xorshift { state: 123456789 };
    let mut sum = 0.0;

    println!("\nRunning Rust VM Loop...");
    let start = Instant::now();
    
    for _ in 0..iterations {
        let z = rng.next_normal();
        context.set_by_index(5, z); // Z
        
        // VM Execution
        let res = program.eval(&context).unwrap();
        sum += res;
    }
    
    let duration = start.elapsed();
    println!("Time: {:.4}s", duration.as_secs_f64());
    println!("Check sum: {:.2}", sum);
}
