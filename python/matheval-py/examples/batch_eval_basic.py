"""
批量求值示例 - 基础用法

演示如何使用 eval_batch() 进行高效的批量计算
"""

import matheval
import time

def main():
    print("=== 批量求值基础示例 ===\n")
    
    # 1. 编译表达式
    compiler = matheval.Compiler()
    program = compiler.compile("x * 2 + y")
    
    print(f"表达式: x * 2 + y")
    print(f"变量顺序: {program.var_names}\n")
    
    # 2. 准备多组变量值
    var_sets = [
        [1.0, 2.0],  # x=1, y=2 -> 1*2+2 = 4
        [3.0, 4.0],  # x=3, y=4 -> 3*2+4 = 10
        [5.0, 6.0],  # x=5, y=6 -> 5*2+6 = 16
    ]
    
    # 3. 批量求值
    results = program.eval_batch(var_sets)
    
    print("批量求值结果:")
    for i, (vars, result) in enumerate(zip(var_sets, results)):
        print(f"  第 {i+1} 组: x={vars[0]}, y={vars[1]} -> 结果: {result}")
    
    print("\n=== 性能对比 ===\n")
    
    # 准备大量数据
    iterations = 10000
    large_var_sets = [[float(i), float(i) * 0.5] for i in range(iterations)]
    
    # 方法 1: 循环调用 eval()
    start = time.time()
    results_loop = []
    for var_set in large_var_sets:
        context = matheval.Context()
        context.set("x", var_set[0])
        context.set("y", var_set[1])
        results_loop.append(program.eval(context))
    time_loop = time.time() - start
    
    # 方法 2: 批量求值
    start = time.time()
    results_batch = program.eval_batch(large_var_sets)
    time_batch = time.time() - start
    
    print(f"计算 {iterations} 次:")
    print(f"  循环调用 eval():  {time_loop*1000:.2f} ms")
    print(f"  批量求值 eval_batch(): {time_batch*1000:.2f} ms")
    print(f"  加速比: {time_loop/time_batch:.2f}x")
    
    # 验证结果一致
    assert results_loop == results_batch, "结果不一致！"
    print(f"\n✓ 结果验证通过")

if __name__ == "__main__":
    main()
