"""
批量求值示例 - 蒙特卡洛模拟

演示使用批量求值进行期权定价的蒙特卡洛模拟
"""

import matheval
import random
import time
import math

def main():
    print("=== 蒙特卡洛期权定价模拟 ===\n")
    
    # 期权参数
    S = 100.0      # 标的资产当前价格
    K = 105.0      # 行权价
    T = 1.0        # 到期时间（年）
    r = 0.05       # 无风险利率
    sigma = 0.2    # 波动率
    
    print(f"期权参数:")
    print(f"  标的价格 S: ${S}")
    print(f"  行权价 K: ${K}")
    print(f"  到期时间 T: {T} 年")
    print(f"  无风险利率 r: {r*100}%")
    print(f"  波动率 σ: {sigma*100}%\n")
    
    # 简化的期权定价公式
    # Payoff = max(S - K, 0) * discount
    compiler = matheval.Compiler()
    program = compiler.compile("max(S - K, 0) * discount")
    
    print(f"定价公式: {program.var_names}")
    print(f"变量顺序: {program.var_names}\n")
    
    # 预计算常量
    discount = math.exp(-r * T)
    
    # 蒙特卡洛模拟
    num_simulations = 100000
    print(f"运行 {num_simulations:,} 次模拟...\n")
    
    # 生成随机股价路径
    random.seed(42)
    stock_prices = []
    for _ in range(num_simulations):
        # 简化：直接生成到期时的股价
        # S_T = S * exp((r - 0.5*sigma^2)*T + sigma*sqrt(T)*Z)
        Z = random.gauss(0, 1)  # 标准正态分布
        drift = (r - 0.5 * sigma**2) * T
        diffusion = sigma * math.sqrt(T) * Z
        S_T = S * math.exp(drift + diffusion)
        stock_prices.append(S_T)
    
    # 构建变量集合
    # 变量顺序: S, K, discount
    var_sets = [[s, K, discount] for s in stock_prices]
    
    # 批量计算期权价值
    start = time.time()
    payoffs = program.eval_batch(var_sets)
    time_elapsed = time.time() - start
    
    # 计算期权价格（平均折现收益）
    option_price = sum(payoffs) / num_simulations
    
    # 统计分析
    positive_payoffs = [p for p in payoffs if p > 0]
    prob_itm = len(positive_payoffs) / num_simulations
    avg_positive_payoff = sum(positive_payoffs) / len(positive_payoffs) if positive_payoffs else 0
    
    print("=== 模拟结果 ===")
    print(f"期权理论价格: ${option_price:.4f}")
    print(f"价内概率: {prob_itm*100:.2f}%")
    print(f"平均正收益: ${avg_positive_payoff:.4f}")
    print(f"计算时间: {time_elapsed*1000:.2f} ms")
    print(f"吞吐量: {num_simulations/time_elapsed:,.0f} 次/秒")
    
    # 理论价格（Black-Scholes 公式的近似）
    # 注意：这只是一个粗略的参考值
    print(f"\n(Black-Scholes 理论价格约为 $8.02)")
    
    # 展示一些样本结果
    print(f"\n前 10 个模拟结果:")
    for i in range(10):
        s_t = stock_prices[i]
        payoff = payoffs[i]
        print(f"  #{i+1}: S_T=${s_t:.2f}, Payoff=${payoff:.4f}")

if __name__ == "__main__":
    main()
