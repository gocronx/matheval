use pyo3::prelude::*;
use matheval_core::{Compiler as RsCompiler, Context as RsContext, Program as RsProgram};

#[pyclass]
struct Compiler {
    inner: RsCompiler,
}

#[pymethods]
impl Compiler {
    #[new]
    fn new() -> Self {
        Compiler { inner: RsCompiler::new() }
    }

    fn compile(&self, expr: &str) -> PyResult<Program> {
        match self.inner.compile(expr) {
            Ok(prog) => Ok(Program { inner: prog }),
            Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
        }
    }
}

#[pyclass]
struct Context {
    inner: RsContext,
}

#[pymethods]
impl Context {
    #[new]
    fn new() -> Self {
        Context { inner: RsContext::new() }
    }

    fn set(&mut self, name: &str, value: f64) {
        self.inner.set(name, value);
    }
}

#[pyclass]
struct Program {
    inner: RsProgram,
}

#[pymethods]
impl Program {
    fn eval(&self, context: &Context) -> PyResult<f64> {
        match self.inner.eval(&context.inner) {
            Ok(val) => Ok(val),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
    }
}

#[pymodule]
fn matheval(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Compiler>()?;
    m.add_class::<Context>()?;
    m.add_class::<Program>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the underlying Rust logic without PyO3
    #[test]
    fn test_compiler_inner() {
        let compiler = Compiler::new();
        assert!(compiler.inner.compile("1 + 1").is_ok());
    }

    #[test]
    fn test_context_inner() {
        let mut context = Context::new();
        context.inner.set("x", 10.0);
        // Verify it doesn't panic
    }

    #[test]
    fn test_compilation_inner() {
        let compiler = Compiler::new();
        let program = compiler.inner.compile("x + y").unwrap();
        let mut context = RsContext::new();
        context.set("x", 10.0);
        context.set("y", 20.0);
        assert_eq!(program.eval(&context).unwrap(), 30.0);
    }

    #[test]
    fn test_invalid_expression_inner() {
        let compiler = Compiler::new();
        assert!(compiler.inner.compile("1 + + 2").is_err());
    }

    #[test]
    fn test_arithmetic_operations() {
        let compiler = Compiler::new();
        let ctx = RsContext::new();
        
        assert_eq!(compiler.inner.compile("1 + 2 * 3").unwrap().eval(&ctx).unwrap(), 7.0);
        assert_eq!(compiler.inner.compile("2 ^ 3 ^ 2").unwrap().eval(&ctx).unwrap(), 512.0);
    }
}
