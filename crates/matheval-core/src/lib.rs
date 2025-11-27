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

    /// Batch evaluation: evaluate with multiple variable sets efficiently
    /// 
    /// This is significantly faster than calling `eval()` in a loop because:
    /// - Reuses the same VM instance
    /// - Avoids repeated context creation
    /// - Better cache locality
    /// 
    /// # Arguments
    /// * `var_sets` - Slice of variable value slices. Each inner slice must contain
    ///                values in the same order as `program.var_names`
    /// 
    /// # Returns
    /// Vector of results, one for each variable set
    /// 
    /// # Example
    /// ```
    /// use matheval_core::Compiler;
    /// 
    /// let compiler = Compiler::new();
    /// let program = compiler.compile("x * 2 + y").unwrap();
    /// 
    /// // Batch evaluate with 3 different variable sets
    /// let var_sets: Vec<&[f64]> = vec![
    ///     &[1.0, 2.0],  // x=1, y=2 -> result: 4
    ///     &[3.0, 4.0],  // x=3, y=4 -> result: 10
    ///     &[5.0, 6.0],  // x=5, y=6 -> result: 16
    /// ];
    /// 
    /// let results = program.eval_batch(&var_sets).unwrap();
    /// assert_eq!(results, vec![4.0, 10.0, 16.0]);
    /// ```
    pub fn eval_batch(&self, var_sets: &[&[f64]]) -> Result<Vec<f64>, String> {
        let mut vm = VM::new(self);
        vm.run_batch(var_sets)
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

    #[test]
    fn test_eval_batch_basic() {
        let compiler = Compiler::new();
        let program = compiler.compile("x * 2 + y").unwrap();
        
        let var_sets: Vec<&[f64]> = vec![
            &[1.0, 2.0],  // x=1, y=2 -> 1*2+2 = 4
            &[3.0, 4.0],  // x=3, y=4 -> 3*2+4 = 10
            &[5.0, 6.0],  // x=5, y=6 -> 5*2+6 = 16
        ];
        
        let results = program.eval_batch(&var_sets).unwrap();
        assert_eq!(results, vec![4.0, 10.0, 16.0]);
    }

    #[test]
    fn test_eval_batch_with_functions() {
        let compiler = Compiler::new();
        let program = compiler.compile("sin(x) + cos(y)").unwrap();
        
        let var_sets: Vec<&[f64]> = vec![
            &[0.0, 0.0],
            &[std::f64::consts::PI / 2.0, 0.0],
        ];
        
        let results = program.eval_batch(&var_sets).unwrap();
        assert!((results[0] - 1.0).abs() < 1e-10);  // sin(0) + cos(0) = 1
        assert!((results[1] - 2.0).abs() < 1e-6);   // sin(π/2) + cos(0) = 1 + 1 = 2
    }

    #[test]
    fn test_eval_batch_empty() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y").unwrap();
        
        let var_sets: Vec<&[f64]> = vec![];
        let results = program.eval_batch(&var_sets).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_eval_batch_wrong_var_count() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y").unwrap();  // Expects 2 variables
        
        let var_sets: Vec<&[f64]> = vec![
            &[1.0],  // Only 1 variable - should error
        ];
        
        let result = program.eval_batch(&var_sets);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expected 2"));
    }

    #[test]
    fn test_eval_batch_monte_carlo_simulation() {
        // Simulate simplified option pricing
        let compiler = Compiler::new();
        let program = compiler.compile("max(S - K, 0) * discount").unwrap();
        
        // Variables are ordered by first appearance: S, K, discount
        
        // Prepare parameters
        let k = 105.0;
        let discount = 0.95;
        
        // Different stock prices
        let s_values = vec![90.0, 100.0, 110.0, 120.0, 130.0];
        let var_sets: Vec<Vec<f64>> = s_values.iter().map(|&s| {
            vec![s, k, discount]  // S, K, discount (order of first appearance)
        }).collect();
        let var_sets_refs: Vec<&[f64]> = var_sets.iter().map(|v| v.as_slice()).collect();
        
        let results = program.eval_batch(&var_sets_refs).unwrap();
        
        // All results should be non-negative (option payoff)
        for result in &results {
            assert!(*result >= 0.0);
        }
        
        // Results: max(S-K, 0) * discount for each S value
        assert_eq!(results[0], 0.0);     // max(90-105, 0) * 0.95 = 0
        assert_eq!(results[1], 0.0);     // max(100-105, 0) * 0.95 = 0  
        assert!((results[2] - 4.75).abs() < 0.01);  // max(110-105, 0) * 0.95 = 4.75
        assert!((results[3] - 14.25).abs() < 0.01); // max(120-105, 0) * 0.95 = 14.25
        assert!((results[4] - 23.75).abs() < 0.01); // max(130-105, 0) * 0.95 = 23.75
    }
}
