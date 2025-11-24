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

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, input: &str) -> Result<Program, String> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer)?;
        let ast = parser.parse()?;
        
        let compiler = BytecodeCompiler::new();
        Ok(compiler.compile(ast))
    }
}

impl Program {
    pub fn eval(&self, context: &Context) -> Result<f64, String> {
        let mut vm = VM::new(self);
        vm.run(context)
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
    fn test_variables() {
        let compiler = Compiler::new();
        let program = compiler.compile("x + y").unwrap();
        let mut context = Context::new();
        context.set("x", 10.0);
        context.set("y", 20.0);
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
}
