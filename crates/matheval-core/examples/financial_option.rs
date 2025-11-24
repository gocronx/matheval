use matheval_core::Compiler;

// Option Pricing Model Parameters
struct OptionParams {
    s: f64, // Spot price of the underlying asset
    k: f64, // Strike price
    t: f64, // Time to expiration (in years)
    r: f64, // Risk-free interest rate
    sigma: f64, // Volatility
}

fn main() {
    println!("=== Financial Example: Black-Scholes Option Pricing (Approximation) ===");
    println!("Calculating the theoretical price of a European Call Option.");
    
    // The Black-Scholes formula involves the cumulative distribution function (CDF) of the normal distribution.
    // Since our VM doesn't have a built-in 'norm_cdf' yet, we will use a standard numerical approximation 
    // for the error function 'erf' to compute CDF.
    //
    // CDF(x) = 0.5 * (1 + erf(x / sqrt(2)))
    //
    // We will define a helper formula for d1 and d2 first, but for this single-expression engine,
    // we'll combine them or pre-calculate d1/d2 for simplicity in the formula, 
    // OR we can demonstrate a simplified payoff model if we want to keep the formula short.
    //
    // However, to show the power of the engine, let's implement a Payoff Simulation (Monte Carlo style)
    // which is very common when closed-form formulas are too complex or for exotic options.
    
    println!("\n--- Scenario: Monte Carlo Simulation for Option Pricing ---");
    println!("Payoff = max(S_final - K, 0) * e^(-r * T)");
    println!("Where S_final = S * e^((r - 0.5 * sigma^2) * T + sigma * sqrt(T) * Z)");
    println!("(Z is a random standard normal variable)");

    // 1. Define the Payoff Formula
    // We will pre-calculate the deterministic part of S_final in Rust for efficiency,
    // and let the engine handle the dynamic part involving the random variable Z.
    //
    // Formula: Payoff = max(S_det * (E ^ (vol_part * Z)) - K, 0) * discount
    let payoff_formula = "max(S_det * (E ^ (vol_part * Z)) - K, 0) * discount";
    println!("Formula: {}", payoff_formula);

    let compiler = Compiler::new();
    let program = compiler.compile(payoff_formula).expect("Compilation failed");

    // 2. Setup Parameters
    let params = OptionParams {
        s: 100.0,   // Current Price $100
        k: 105.0,   // Strike Price $105 (Out of the money)
        t: 1.0,     // 1 Year
        r: 0.05,    // 5% Risk-free rate
        sigma: 0.2, // 20% Volatility
    };

    // Pre-calculate constants for the simulation
    let discount = (-params.r * params.t).exp();
    let drift = (params.r - 0.5 * params.sigma.powi(2)) * params.t;
    let vol_part = params.sigma * params.t.sqrt();
    let s_det = params.s * drift.exp();

    let mut context = program.create_context();
    // Use indexed access for best performance in hot loop
    // Variable order: discount, E, K, S_det, vol_part, Z
    context.set_by_index(0, discount);
    context.set_by_index(1, std::f64::consts::E);
    context.set_by_index(2, params.k);
    context.set_by_index(3, s_det);
    context.set_by_index(4, vol_part);
    // Z will be set in the loop

    // 3. Run Simulation
    let simulations = 100_000;
    println!("Simulating {} paths...", simulations);

    // Simple Box-Muller transform to generate standard normal random numbers
    // Since we don't have 'rand' dependency in this minimal example, we'll use a simple LCG 
    // or just mock it for demonstration. In a real app, use `rand::thread_rng`.
    // For this demo, to keep it zero-dependency, I'll implement a tiny pseudo-rng.
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
        // Box-Muller transform
        fn next_normal(&mut self) -> f64 {
            let u1 = self.next_f64();
            let u2 = self.next_f64();
            (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
        }
    }
    let mut rng = Xorshift { state: 123456789 };

    let mut total_payoff = 0.0;
    
    for _ in 0..simulations {
        let z = rng.next_normal();
        context.set_by_index(5, z); // Z
        
        let payoff = program.eval(&context).unwrap();
        total_payoff += payoff;
    }

    let option_price = total_payoff / simulations as f64;

    println!("\n--- Results ---");
    println!("Underlying Price: ${:.2}", params.s);
    println!("Strike Price:     ${:.2}", params.k);
    println!("Estimated Call Option Price: ${:.4}", option_price);
    println!("(Theoretical Black-Scholes price is approx $8.02)");
}
