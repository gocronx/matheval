# matheval

**matheval** is a modern, high-performance mathematical expression evaluator for Rust, built from scratch with a focus on flexibility and speed.

It abandons the traditional Shunting-yard algorithm in favor of **Pratt Parsing** combined with a **Stack-based Virtual Machine**.

## Features

- **🚀 High Performance**: 
  - **Linear Bytecode Execution**: Flattens expressions into cache-friendly bytecode for efficient VM execution
  - **Variable Interning**: O(1) variable lookups using indexed arrays instead of hash-based lookups
  - **Stack-based VM**: Simple and efficient execution model with excellent instruction cache locality
- **🛠 Flexible & Extensible Syntax**: 
  - **Pratt Parsing**: Top-down operator precedence parsing for elegant handling of complex expressions
  - **Right-associative Operators**: Native support for operators like `^` (exponentiation)
  - **Unary Operators & Function Calls**: Built-in support for mathematical functions
  
- **📦 Zero Heavy Dependencies**: Lightweight core with minimal dependencies
- **🔒 Safe**: Written in 100% safe Rust with no unsafe code
- **🚀 Fast**: Stack-based VM, O(1) variable access, constant folding
- **⚡️ Batch Evaluation**: Optimized API for bulk calculations (e.g., Monte Carlo)
- **🐍 Python Bindings**: High-performance Python interface via PyO3
  

## Architecture & References

This project implements a **Pratt Parser** (Top-Down Operator Precedence Parser) which is ideal for mathematical expressions. The parser produces an AST which is then compiled into bytecode for a custom **Stack-based Virtual Machine**.

Key optimizations include:
- **Indexed Variable Access**: Variables are resolved to indices at compile time for O(1) access.
- **Compact Bytecode**: Instructions are compressed to 1 byte for better cache locality.
- **Constant Folding**: Arithmetic on constants is evaluated at compile time.
- **Batch Evaluation**: Reuses VM resources for high-throughput bulk processing.

### System Architecture

The data flow proceeds in three stages:

```text
Source String  --> [ Lexer ] --> Tokens
                                  |
                                  v
                             [ Parser ] (Pratt)
                                  |
                                  v
                                 AST
                                  |
                                  v
                             [ Compiler ] --> (Symbol Table Resolution)
                                  |
                                  v
                            Bytecode (Program)
                                  |
                                  v
                             [    VM    ] --> Result
```

### Module Breakdown

1.  **`token` & `lexer`**
    *   **Responsibility**: Convert raw string input into a stream of `Token`s.
    *   **Key Types**: `Token` (Enum), `Lexer` (Iterator).

2.  **`parser` (Pratt Parser)**
    *   **Responsibility**: Convert `Token` stream into an Abstract Syntax Tree (AST).
    *   **Algorithm**: Top-Down Operator Precedence.
    *   **Key Concepts**: `bp` (Binding Power), `nud` (Null Denotation), `led` (Left Denotation).

3.  **`compiler`**
    *   **Responsibility**: Flatten the AST into Linear Bytecode and resolve variable names.
    *   **Key Actions**:
        *   **Interning**: Collects all unique variable names into a `Vec<String>` and replaces them with `u16` indices in the bytecode.
        *   **Code Gen**: Emits `OpCode`s for the VM.

4.  **`bytecode`**
    *   **Responsibility**: Define the instruction set for the VM.
    *   **Instructions**: `LoadConst`, `LoadVar`, `Add`, `Sub`, `Mul`, `Div`, `Pow`, `Call`.

5.  **`vm` (Virtual Machine)**
    *   **Responsibility**: Execute the bytecode.
    *   **State**: `stack` (Vec<f64>), `vars` (Context).
    *   **Execution**: A tight loop matching on `OpCode`s.

## Quick Start

```rust
use matheval::{Compiler, Context};

fn main() {
    // 1. Compile the expression
    // This step parses the string and generates optimized bytecode.
    let compiler = Compiler::new();
    let program = compiler.compile("x + sin(PI * y)").expect("Compilation failed");

    // 2. Prepare the context (variables)
    let mut context = Context::new();
    context.set("x", 1.5);
    context.set("y", 0.5);
    context.set("PI", std::f64::consts::PI);

    // 3. Evaluate
    let result = program.eval(&context).expect("Runtime error");
    println!("Result: {}", result);
}
```