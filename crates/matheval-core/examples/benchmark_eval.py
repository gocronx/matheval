import math
import random
import time

def benchmark_eval():
    print("=== Benchmark: Python 'eval()' vs Hardcoded ===")
    print("Scenario: Evaluating a user-provided formula 1,000,000 times.")
    
    # The formula (same as before)
    formula_str = "max(S_det * math.exp(vol_part * Z) - K, 0) * discount"
    
    # Parameters
    S = 100.0
    K = 105.0
    T = 1.0
    r = 0.05
    sigma = 0.2
    
    discount = math.exp(-r * T)
    drift = (r - 0.5 * sigma**2) * T
    vol_part = sigma * math.sqrt(T)
    S_det = S * math.exp(drift)
    
    iterations = 1_000_000
    
    # 1. Hardcoded (Baseline)
    print(f"\n1. Hardcoded Python (Native Speed)")
    start = time.time()
    sum_res = 0.0
    for _ in range(iterations):
        Z = random.gauss(0.0, 1.0)
        # Direct execution
        res = max(S_det * math.exp(vol_part * Z) - K, 0) * discount
        sum_res += res
    duration = time.time() - start
    print(f"Time: {duration:.4f}s")
    
    # 2. Dynamic 'eval()' (Simulating user input)
    print(f"\n2. Python 'eval()' (Dynamic Formula)")
    print("Note: This parses the string every single time!")
    start = time.time()
    sum_res = 0.0
    
    # Pre-compile context for eval to be fair (locals dictionary)
    # But eval() itself still has to parse/execute the code object or string.
    # Using string eval is the direct comparison to "interpreting a string".
    
    for _ in range(iterations):
        Z = random.gauss(0.0, 1.0)
        # Dynamic execution
        res = eval(formula_str)
        sum_res += res
    duration = time.time() - start
    print(f"Time: {duration:.4f}s")

    # 3. Python 'compile()' (Optimized Dynamic)
    print(f"\n3. Python 'compile()' + 'eval()' (Best Case Dynamic)")
    print("Compiling the string once, then executing the code object.")
    start = time.time()
    sum_res = 0.0
    
    # Compile once
    code_obj = compile(formula_str, "<string>", "eval")
    
    for _ in range(iterations):
        Z = random.gauss(0.0, 1.0)
        # Execute pre-compiled code
        res = eval(code_obj)
        sum_res += res
    duration = time.time() - start
    print(f"Time: {duration:.4f}s")

if __name__ == "__main__":
    benchmark_eval()
