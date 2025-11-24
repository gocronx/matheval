mod token;
mod lexer;
mod ast;
mod parser;
mod bytecode;
mod compiler;
mod vm;

pub use vm::Context;
pub use bytecode::Program;

use lexer::Lexer;
use parser::Parser;
use compiler::Compiler as BytecodeCompiler;
use vm::VM;

/// High-level compiler interface
pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    /// Compile a mathematical expression into optimized bytecode
    pub fn compile(&self, input: &str) -> Result<Program, String> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer)?;
        let ast = parser.parse()?;
        
        let compiler = BytecodeCompiler::new();
        Ok(compiler.compile(ast))
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    /// Evaluate the program with the given context
    pub fn eval(&self, context: &Context) -> Result<f64, String> {
        let mut vm = VM::new(self);
        vm.run(context)
    }

    /// Create a context pre-sized for this program
    pub fn create_context(&self) -> Context {
        Context::with_capacity(self.var_names.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        let compiler = Compiler::new();
        let program = compiler.compile("1 + 2 * 3").unwrap();
        let context = Context::new();
        let result = program.eval(&context).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_constant_folding() {
        let compiler = Compiler::new();
        // 1 + 2 should be folded to 3 at compile time
        let program = compiler.compile("1 + 2").unwrap();
        // Should only have one constant (3.0)
        assert_eq!(program.constants.len(), 1);
        assert_eq!(program.constants[0], 3.0);
    }

    #[test]
    fn test_variables_new_api() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y").unwrap();
        
        // New API: use indexed access
        let mut context = program.create_context();
        context.set_by_index(0, 10.0); // x
        context.set_by_index(1, 20.0); // y
        
        let result = program.eval(&context).unwrap();
        assert_eq!(result, 30.0);
    }

    #[test]
    fn test_variables_with_program() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y").unwrap();
        
        // Alternative API: use names (requires program reference)
        let mut context = Context::new();
        context.set("x", 10.0, &program);
        context.set("y", 20.0, &program);
        
        let result = program.eval(&context).unwrap();
        assert_eq!(result, 30.0);
    }

    #[test]
    fn test_functions() {
        let compiler = Compiler::new();
        let program = compiler.compile("max(1, 2, 3) + min(4, 5)").unwrap();
        let context = Context::new();
        let result = program.eval(&context).unwrap();
        assert_eq!(result, 3.0 + 4.0);
    }

    #[test]
    fn test_math_functions() {
        let compiler = Compiler::new();
        
        // Test sqrt
        let program = compiler.compile("sqrt(4)").unwrap();
        let result = program.eval(&Context::new()).unwrap();
        assert_eq!(result, 2.0);
        
        // Test abs
        let program = compiler.compile("abs(-5)").unwrap();
        let result = program.eval(&Context::new()).unwrap();
        assert_eq!(result, 5.0);
    }
    
    #[test]
    fn test_precedence() {
        let compiler = Compiler::new();
        // 2 * 3 + 4 = 10
        // 2 + 3 * 4 = 14
        let p1 = compiler.compile("2 * 3 + 4").unwrap();
        let p2 = compiler.compile("2 + 3 * 4").unwrap();
        let ctx = Context::new();
        assert_eq!(p1.eval(&ctx).unwrap(), 10.0);
        assert_eq!(p2.eval(&ctx).unwrap(), 14.0);
    }
    
    #[test]
    fn test_right_associativity() {
        let compiler = Compiler::new();
        // 2 ^ 3 ^ 2 = 2 ^ (9) = 512
        // (2 ^ 3) ^ 2 = 8 ^ 2 = 64
        let program = compiler.compile("2 ^ 3 ^ 2").unwrap();
        let ctx = Context::new();
        assert_eq!(program.eval(&ctx).unwrap(), 512.0);
    }

    #[test]
    fn test_complex_expression() {
        let compiler = Compiler::new();
        let program = compiler.compile("x * 2 + sin(y) - 3 / z").unwrap();
        
        let mut ctx = program.create_context();
        ctx.set_by_index(0, 5.0);  // x
        ctx.set_by_index(1, 0.0);  // y (sin(0) = 0)
        ctx.set_by_index(2, 1.0);  // z
        
        let result = program.eval(&ctx).unwrap();
        assert_eq!(result, 5.0 * 2.0 + 0.0 - 3.0 / 1.0);
    }

    #[test]
    fn test_algebraic_optimization() {
        let compiler = Compiler::new();
        
        // x * 0 should be optimized to 0
        let program = compiler.compile("x * 0").unwrap();
        assert_eq!(program.constants.len(), 1);
        assert_eq!(program.constants[0], 0.0);
        
        // x * 1 should be optimized to just x
        let program = compiler.compile("x * 1").unwrap();
        // Should have no constants, just load variable
        assert_eq!(program.constants.len(), 0);
    }

    #[test]
    fn test_error_undefined_variable() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y").unwrap();
        
        // Context with only 1 variable (needs 2)
        let ctx = Context::with_capacity(1);
        let result = program.eval(&ctx);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_error_division_by_zero() {
        let compiler = Compiler::new();
        let program = compiler.compile("1 / 0").unwrap();
        let result = program.eval(&Context::new());
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_compiler_default() {
        let compiler = Compiler::default();
        let program = compiler.compile("1 + 1").unwrap();
        assert!(program.eval(&Context::new()).is_ok());
    }

    #[test]
    fn test_reusable_program() {
        let compiler = Compiler::new();
        let program = compiler.compile("x * 2").unwrap();
        
        // Reuse same program with different contexts
        let mut ctx1 = program.create_context();
        ctx1.set_by_index(0, 5.0);
        assert_eq!(program.eval(&ctx1).unwrap(), 10.0);
        
        let mut ctx2 = program.create_context();
        ctx2.set_by_index(0, 10.0);
        assert_eq!(program.eval(&ctx2).unwrap(), 20.0);
    }
}
