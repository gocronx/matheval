"""
批量求值示例 - 参数扫描

演示使用批量求值进行参数空间扫描和敏感性分析
"""

import matheval
import time

def main():
    print("=== 参数扫描示例 ===\n")
    
    # 物理公式：自由落体距离
    # d = v0*t + 0.5*g*t^2
    compiler = matheval.Compiler()
    program = compiler.compile("v0 * t + 0.5 * g * t^2")
    
    print(f"公式: d = v0*t + 0.5*g*t^2")
    print(f"变量: {program.var_names}\n")
    
    # 参数范围
    g = 9.8  # 重力加速度 (m/s^2)
    v0_values = [0, 5, 10, 15, 20]  # 初速度 (m/s)
    t_values = [0.5, 1.0, 1.5, 2.0, 2.5]  # 时间 (s)
    
    print("参数扫描:")
    print(f"  重力加速度 g = {g} m/s²")
    print(f"  初速度 v0 = {v0_values} m/s")
    print(f"  时间 t = {t_values} s\n")
    
    # 生成所有参数组合
    var_sets = []
    for v0 in v0_values:
        for t in t_values:
            # 变量顺序: g, t, v0
            var_sets.append([g, t, v0])
    
    print(f"总共 {len(var_sets)} 种参数组合\n")
    
    # 批量计算
    start = time.time()
    distances = program.eval_batch(var_sets)
    time_elapsed = time.time() - start
    
    # 展示结果表格
    print("结果表格 (距离单位: 米):")
    print(f"{'v0':>8}", end="")
    print("    ", end="") # Tab replacement
    for t in t_values:
        print(f"{t:>8.1f}", end="")
    print()
    print("-" * 56)
    
    idx = 0
    for v0 in v0_values:
        print(f"{v0:>8.1f}", end="")
        for _ in t_values:
            print(f"{distances[idx]:>8.2f}", end="")
            idx += 1
        print()
    
    print(f"\n计算时间: {time_elapsed*1000:.2f} ms")
    print(f"平均每次: {time_elapsed/len(var_sets)*1000000:.2f} μs")
    
    # 敏感性分析
    print("\n=== 敏感性分析 ===\n")
    
    # 固定时间，分析初速度的影响
    t_fixed = 2.0
    v0_range = [float(i) for i in range(0, 51, 5)]
    var_sets_v0 = [[g, t_fixed, v0] for v0 in v0_range]
    distances_v0 = program.eval_batch(var_sets_v0)
    
    print(f"固定时间 t={t_fixed}s，改变初速度:")
    for v0, d in zip(v0_range, distances_v0):
        print(f"  v0={v0:>4.0f} m/s -> d={d:>6.2f} m")
    
    # 固定初速度，分析时间的影响
    print(f"\n固定初速度 v0={v0_values[2]}m/s，改变时间:")
    v0_fixed = v0_values[2]
    t_range = [float(i) * 0.2 for i in range(0, 16)]
    var_sets_t = [[g, t, v0_fixed] for t in t_range]
    distances_t = program.eval_batch(var_sets_t)
    
    for t, d in zip(t_range, distances_t):
        print(f"  t={t:>4.1f} s -> d={d:>6.2f} m")

if __name__ == "__main__":
    main()
