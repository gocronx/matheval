import math
import random

def black_scholes_monte_carlo():
    print("=== Financial Example: Black-Scholes Option Pricing (Python) ===")
    print("Calculating the theoretical price of a European Call Option.")
    
    # Parameters
    S = 100.0   # Spot Price
    K = 105.0   # Strike Price
    T = 1.0     # Time to expiration (years)
    r = 0.05    # Risk-free rate
    sigma = 0.2 # Volatility
    
    simulations = 100_000
    print(f"Simulating {simulations} paths...")
    
    # Pre-calculate deterministic parts
    discount = math.exp(-r * T)
    drift = (r - 0.5 * sigma**2) * T
    vol_part = sigma * math.sqrt(T)
    S_det = S * math.exp(drift)
    
    total_payoff = 0.0
    
    # Monte Carlo Loop
    # In Python, this loop is significantly slower than Rust for large N
    for _ in range(simulations):
        # Generate standard normal random variable Z
        Z = random.gauss(0.0, 1.0)
        
        # Calculate S_final
        # Formula: S_final = S * exp((r - 0.5*sigma^2)*T + sigma*sqrt(T)*Z)
        # Optimized: S_final = S_det * exp(vol_part * Z)
        S_final = S_det * math.exp(vol_part * Z)
        
        # Calculate Payoff
        payoff = max(S_final - K, 0) * discount
        total_payoff += payoff
        
    option_price = total_payoff / simulations
    
    print("\n--- Results ---")
    print(f"Underlying Price: ${S:.2f}")
    print(f"Strike Price:     ${K:.2f}")
    print(f"Estimated Call Option Price: ${option_price:.4f}")
    print("(Theoretical Black-Scholes price is approx $8.02)")

if __name__ == "__main__":
    black_scholes_monte_carlo()
