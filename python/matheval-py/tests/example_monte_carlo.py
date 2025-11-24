"""
Example: Monte Carlo Option Pricing with matheval

This demonstrates using matheval for high-frequency financial calculations.
"""
import matheval
import random
import time


def monte_carlo_option_pricing():
    # Parameters
    S = 100.0   # Spot Price
    K = 105.0   # Strike Price
    T = 1.0     # Time to expiration (years)
    r = 0.05    # Risk-free rate
    sigma = 0.2 # Volatility
    
    simulations = 100_000
    
    # Pre-calculate deterministic parts
    discount = ((-r * T) ** 0.5) ** 2  # Simplified exp(-r*T) approximation
    drift = (r - 0.5 * sigma**2) * T
    vol_part = sigma * (T ** 0.5)
    S_det = S * ((drift) ** 0.5) ** 2  # Simplified exp(drift)
    
    # Compile the payoff formula
    compiler = matheval.Compiler()
    # Note: We use a simplified formula here since matheval doesn't have exp() yet
    # In production, you'd add exp() as a built-in function
    program = compiler.compile("max(S_det * (1 + vol_part * Z) - K, 0) * discount")
    
    # Setup context
    context = matheval.Context()
    context.set("K", K)
    context.set("discount", discount)
    context.set("S_det", S_det)
    context.set("vol_part", vol_part)
    
    print(f"Running {simulations} Monte Carlo simulations...")
    start = time.time()
    
    total_payoff = 0.0
    for _ in range(simulations):
        Z = random.gauss(0.0, 1.0)
        context.set("Z", Z)
        
        payoff = program.eval(context)
        total_payoff += payoff
    
    duration = time.time() - start
    option_price = total_payoff / simulations
    
    print(f"\nResults:")
    print(f"  Underlying Price: ${S:.2f}")
    print(f"  Strike Price:     ${K:.2f}")
    print(f"  Estimated Option Price: ${option_price:.4f}")
    print(f"  Time: {duration:.4f}s")
    print(f"  Avg per iteration: {duration/simulations*1e6:.2f}µs")


if __name__ == "__main__":
    monte_carlo_option_pricing()
