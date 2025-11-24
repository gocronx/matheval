# matheval-py

Python bindings for `matheval-core`, a high-performance mathematical expression evaluator.

## Installation

### From Source (Development)

1. Install `maturin`:
   ```bash
   pip install maturin
   ```

2. Build and install in development mode:
   ```bash
   cd python/matheval-py
   maturin develop
   ```

### From PyPI (Future)

Once published:
```bash
pip install matheval
```

## Usage

```python
import matheval

# 1. Create a compiler
compiler = matheval.Compiler()

# 2. Compile an expression
program = compiler.compile("x + sin(PI * y)")

# 3. Create a context and set variables
context = matheval.Context()
context.set("x", 1.5)
context.set("y", 0.5)
context.set("PI", 3.14159)

# 4. Evaluate
result = program.eval(context)
print(f"Result: {result}")  # Output: Result: 2.5
```

## API Reference

### `Compiler`

- `Compiler()`: Create a new compiler instance.
- `compile(expr: str) -> Program`: Compile an expression string into a `Program`.

### `Context`

- `Context()`: Create a new context for storing variables.
- `set(name: str, value: float)`: Set a variable value.

### `Program`

- `eval(context: Context) -> float`: Evaluate the program with the given context.

## Examples

See the `tests/` directory for more examples.
