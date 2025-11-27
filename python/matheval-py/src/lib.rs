use pyo3::prelude::*;
use matheval_core::{Compiler as RsCompiler, Program as RsProgram};
use std::collections::HashMap;

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
    variables: HashMap<String, f64>,
}

#[pymethods]
impl Context {
    #[new]
    fn new() -> Self {
        Context { variables: HashMap::new() }
    }

    fn set(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }
    
    fn get(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }
}

#[pyclass]
struct Program {
    inner: RsProgram,
}

#[pymethods]
impl Program {
    fn eval(&self, context: &Context) -> PyResult<f64> {
        // Create optimized Rust context from Python context
        let mut rs_context = self.inner.create_context();
        
        // Map variables by index for optimal performance
        for (idx, var_name) in self.inner.var_names.iter().enumerate() {
            if let Some(&value) = context.variables.get(var_name) {
                rs_context.set_by_index(idx, value);
            } else {
                return Err(pyo3::exceptions::PyRuntimeError::new_err(
                    format!("Undefined variable: {}", var_name)
                ));
            }
        }
        
        match self.inner.eval(&rs_context) {
            Ok(val) => Ok(val),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
    }
    
    #[getter]
    fn var_names(&self) -> Vec<String> {
        self.inner.var_names.clone()
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

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new();
        assert!(compiler.inner.compile("1 + 1").is_ok());
    }

    #[test]
    fn test_context_operations() {
        let mut context = Context::new();
        context.set("x", 10.0);
        assert_eq!(context.get("x"), Some(10.0));
        assert_eq!(context.get("y"), None);
    }

    #[test]
    fn test_compilation_and_eval() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y").unwrap();
        
        let mut context = Context::new();
        context.set("x", 10.0);
        context.set("y", 20.0);
        
        assert_eq!(program.eval(&context).unwrap(), 30.0);
    }

    #[test]
    fn test_invalid_expression() {
        let compiler = Compiler::new();
        assert!(compiler.compile("1 + + 2").is_err());
    }

    #[test]
    fn test_arithmetic_operations() {
        let compiler = Compiler::new();
        let ctx = Context::new();
        
        let prog1 = compiler.compile("1 + 2 * 3").unwrap();
        assert_eq!(prog1.eval(&ctx).unwrap(), 7.0);
        
        let prog2 = compiler.compile("2 ^ 3 ^ 2").unwrap();
        assert_eq!(prog2.eval(&ctx).unwrap(), 512.0);
    }

    #[test]
    fn test_undefined_variable() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y").unwrap();
        let mut context = Context::new();
        context.set("x", 10.0);
        // Missing 'y' - should return RuntimeError
        
        let result = program.eval(&context);
        assert!(result.is_err());
        // The error should contain "Undefined variable"
    }

    #[test]
    fn test_var_names_property() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y * z").unwrap();
        let var_names = program.var_names();
        
        assert_eq!(var_names.len(), 3);
        assert!(var_names.contains(&"x".to_string()));
        assert!(var_names.contains(&"y".to_string()));
        assert!(var_names.contains(&"z".to_string()));
    }

    #[test]
    fn test_constant_folding_in_python_wrapper() {
        let compiler = Compiler::new();
        let program = compiler.compile("1 + 2 * 3").unwrap();
        // Should be folded to 7
        assert_eq!(program.inner.constants.len(), 1);
        assert_eq!(program.inner.constants[0], 7.0);
    }
}
