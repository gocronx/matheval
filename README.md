# matheval

**matheval** is a modern, high-performance mathematical expression evaluator for Rust, built from scratch with a focus on flexibility and speed.

It abandons the traditional Shunting-yard algorithm in favor of **Pratt Parsing** combined with a **Stack-based Virtual Machine**.

## Features

- **🚀 High Performance**: Uses linear bytecode and variable interning (O(1) lookups) for fast evaluation.
- **🛠 Flexible Syntax**: Powered by a Pratt Parser, easily supporting unary operators, function calls, and operator precedence.
- **📦 Zero Heavy Dependencies**: Lightweight and easy to audit.
- **🔒 Safe**: Written in pure safe Rust.

## Architecture & References

This project implements algorithms described in seminal computer science papers:

*   **Parsing Algorithm**:
    *   **Pratt Parsing (Top-Down Operator Precedence)**:
        *   Pratt, V. R. (1973). "Top down operator precedence". *Proceedings of the 1st Annual ACM SIGACT-SIGPLAN Symposium on Principles of Programming Languages (POPL '73)*.
        *   [Paper Link (ACM)](https://dl.acm.org/doi/10.1145/512927.512931) | [Archived PDF](https://tdop.github.io/)
        *   *Why we use it*: It allows for a highly extensible parser that handles operator precedence and associativity (including right-associative operators like `^`) more elegantly than the traditional Shunting-yard algorithm.

*   **Execution Model**:
    *   **Stack-based Virtual Machine**:
        *   Koopman, P. (1989). *Stack Computers: the new wave*.
        *   [Book Link (CMU)](https://users.ece.cmu.edu/~koopman/stack_computers/)
        *   *Why we use it*: Stack machines provide a simple yet efficient execution model. By flattening the AST into a linear sequence of instructions (bytecode), we improve instruction cache locality compared to tree-walking interpreters.

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

## Performance Comparison (Conceptual)

| Feature | `meval-rs` (Shunting-yard) | `matheval` (This Crate) |
| :--- | :--- | :--- |
| **Parsing Algorithm** | Shunting-yard | **Pratt Parsing** (More flexible) |
| **Execution Model** | RPN + Stack Interpretation | **Linear Bytecode VM** (Cache friendly) |
| **Variable Lookup** | HashMap (String hashing) | **Index Array** (Direct access) |
| **Extensibility** | Limited | High |

## License

Apache-2.0
