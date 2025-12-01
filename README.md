# matheval

A high-performance mathematical expression evaluator implementing a **Pratt parser** with **bytecode compilation** and a **stack-based virtual machine**.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

## Design Philosophy

Traditional expression evaluators rely on the Shunting-yard algorithm and tree-walking interpretation. This implementation adopts a **compiler-VM architecture** for superior performance:

- **Pratt Parsing**: Top-down operator precedence parsing with O(n) complexity
- **Bytecode Compilation**: AST flattening into compact, cache-friendly instruction sequences
- **Stack-based VM**: Zero-overhead execution with tight instruction dispatch loop

## Performance Characteristics

| Component | Optimization | Complexity |
|-----------|-------------|------------|
| Variable Resolution | Index-based interning | O(1) |
| Constant Pool | HashMap deduplication | O(1) |
| Function Dispatch | Direct pointer table | O(1) |
| Batch Evaluation | Shared VM instance | O(n·m) |

**Key Optimizations:**
- **Constant Folding**: Compile-time evaluation of constant expressions
- **Algebraic Simplification**: Identity elimination (x·1 → x, x+0 → x)
- **Zero-copy Execution**: Direct stack manipulation without heap allocation
- **Function Validation**: Compile-time arity checking with metadata

## Architecture

```
┌─────────────┐
│   Source    │
└──────┬──────┘
       │ Lexical Analysis
       ▼
┌─────────────┐
│   Tokens    │
└──────┬──────┘
       │ Pratt Parsing
       ▼
┌─────────────┐
│     AST     │
└──────┬──────┘
       │ Optimization & Compilation
       ▼
┌─────────────┐     ┌──────────────┐
│  Bytecode   │────▶│ Symbol Table │
└──────┬──────┘     └──────────────┘
       │
       │ VM Execution
       ▼
┌─────────────┐
│   Result    │
└─────────────┘
```

## Quick Start

### Rust

```rust
use matheval_core::Compiler;

let compiler = Compiler::new();
let program = compiler.compile("x^2 + 2*x + 1").unwrap();

let mut context = program.create_context();
context.set_by_index(0, 3.0);  // x = 3

let result = program.eval(&context).unwrap();
assert_eq!(result, 16.0);  // 3² + 2·3 + 1 = 16
```

### Python

```python
import matheval

compiler = matheval.Compiler()
program = compiler.compile("x^2 + 2*x + 1")

context = matheval.Context()
context.set("x", 3.0)

result = program.eval(context)
assert result == 16.0
```

## Advanced Features

### Batch Evaluation

Optimized for Monte Carlo simulations and parameter sweeps:

```rust
let program = compiler.compile("S * exp(r*T) - K").unwrap();

// Vectorized evaluation: reuses VM instance
let var_sets = vec![
    &[100.0, 0.05, 1.0, 105.0][..],  // S, r, T, K
    &[110.0, 0.05, 1.0, 105.0][..],
    &[120.0, 0.05, 1.0, 105.0][..],
];

let results = program.eval_batch(&var_sets).unwrap();
```

**Performance**: 6-7× faster than loop-based evaluation for large datasets.

### Error Handling

Comprehensive error reporting with position tracking:

```rust
use matheval_core::{Error, ErrorKind, Position};

let err = Error::wrong_arg_count("sin", 1, 2)
    .with_position(Position::new(1, 5, 4))
    .with_source("x + sin(1, 2)".to_string());

// Output:
// Error: Function 'sin' expects 1 argument, but got 2 at line 1, column 5
//
//   1 | x + sin(1, 2)
//           ^
//
// Hint: Check the function documentation for the correct number of arguments
```

## Supported Operations

**Arithmetic**: `+`, `-`, `*`, `/`, `^` (right-associative)  
**Functions**: `sin`, `cos`, `tan`, `sqrt`, `abs`, `floor`, `ceil`, `round`, `exp`, `ln`, `log10`, `max`, `min`  
**Precedence**: Standard mathematical order with parentheses support

## Implementation Details

### Compiler Pipeline

1. **Lexical Analysis**: Tokenization with position tracking
2. **Syntax Analysis**: Pratt parsing with binding power resolution
3. **Optimization**: Constant folding and algebraic simplification
4. **Code Generation**: Bytecode emission with symbol table construction

### Virtual Machine

- **Instruction Set**: 9 opcodes (LoadConst, LoadVar, Add, Sub, Mul, Div, Pow, Neg, Call)
- **Stack Model**: f64 operand stack with bounds checking
- **Function Table**: Direct function pointer dispatch
- **Metadata**: Compile-time arity validation

### Memory Layout

```
Program {
    instructions: Vec<u8>,           // Compact bytecode
    constants: Vec<f64>,             // Deduplicated constant pool
    var_names: Vec<String>,          // Variable name table
    func_table: Vec<BuiltinFn>,      // Function pointer table
    func_metadata: Vec<Metadata>,    // Arity information
}
```

## Benchmarks

Expression: `x * 2 + sin(y)` (10,000 iterations)

| Method | Time | Throughput |
|--------|------|------------|
| Loop eval() | 34.5 ms | 290k ops/s |
| Batch eval_batch() | 5.0 ms | 2M ops/s |

**Speedup**: 6.9× for batch evaluation

## Safety & Correctness

- **100% Safe Rust**: No `unsafe` blocks
- **Comprehensive Testing**: 97 unit tests, 9 Python integration tests
- **Validation**: Compile-time function arity checking
- **Error Recovery**: Graceful handling of division by zero, stack underflow