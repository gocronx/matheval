use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::bytecode::{BuiltinFn, OpCode, Program};
use std::collections::HashMap;

/// Compiler with constant folding optimization
pub struct Compiler {
    program: Program,
    var_map: HashMap<String, u16>,
    func_map: HashMap<String, u16>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            program: Program::new(),
            var_map: HashMap::new(),
            func_map: HashMap::new(),
        }
    }

    pub fn compile(mut self, expr: Expr) -> Program {
        // Optimize AST before compilation
        let optimized = self.optimize_expr(expr);
        self.compile_expr(optimized);
        self.program
    }

    /// Constant folding optimization
    fn optimize_expr(&self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary { op, left, right } => {
                let left = self.optimize_expr(*left);
                let right = self.optimize_expr(*right);

                // Fold if both operands are constants
                if let (Expr::Number(a), Expr::Number(b)) = (&left, &right) {
                    let result = match op {
                        BinaryOp::Add => a + b,
                        BinaryOp::Sub => a - b,
                        BinaryOp::Mul => a * b,
                        BinaryOp::Div => {
                            if *b == 0.0 {
                                // Don't fold division by zero
                                return Expr::Binary {
                                    op,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                };
                            }
                            a / b
                        }
                        BinaryOp::Pow => a.powf(*b),
                    };
                    return Expr::Number(result);
                }

                // Algebraic optimizations
                match (&left, &right, op) {
                    // x + 0 = x, 0 + x = x
                    (Expr::Number(0.0), _, BinaryOp::Add) => right,
                    (_, Expr::Number(0.0), BinaryOp::Add) => left,
                    // x - 0 = x
                    (_, Expr::Number(0.0), BinaryOp::Sub) => left,
                    // x * 0 = 0, 0 * x = 0
                    (Expr::Number(0.0), _, BinaryOp::Mul) => Expr::Number(0.0),
                    (_, Expr::Number(0.0), BinaryOp::Mul) => Expr::Number(0.0),
                    // x * 1 = x, 1 * x = x
                    (Expr::Number(1.0), _, BinaryOp::Mul) => right,
                    (_, Expr::Number(1.0), BinaryOp::Mul) => left,
                    // x / 1 = x
                    (_, Expr::Number(1.0), BinaryOp::Div) => left,
                    // x ^ 0 = 1
                    (_, Expr::Number(0.0), BinaryOp::Pow) => Expr::Number(1.0),
                    // x ^ 1 = x
                    (_, Expr::Number(1.0), BinaryOp::Pow) => left,
                    _ => Expr::Binary {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }
            Expr::Unary { op, expr } => {
                let expr = self.optimize_expr(*expr);
                if let Expr::Number(n) = expr {
                    match op {
                        UnaryOp::Neg => Expr::Number(-n),
                    }
                } else {
                    Expr::Unary {
                        op,
                        expr: Box::new(expr),
                    }
                }
            }
            Expr::Call { func, args } => {
                let args = args.into_iter().map(|a| self.optimize_expr(a)).collect();
                Expr::Call { func, args }
            }
            _ => expr,
        }
    }

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Number(n) => {
                let idx = self.add_constant(n);
                self.emit_load_const(idx);
            }
            Expr::Variable(name) => {
                let idx = self.resolve_var(name);
                self.emit_load_var(idx);
            }
            Expr::Binary { op, left, right } => {
                self.compile_expr(*left);
                self.compile_expr(*right);
                self.emit_binop(op);
            }
            Expr::Unary { op, expr } => {
                self.compile_expr(*expr);
                self.emit_unop(op);
            }
            Expr::Call { func, args } => {
                let arg_count = args.len() as u8;
                for arg in args {
                    self.compile_expr(arg);
                }
                let func_idx = self.resolve_func(func);
                self.emit_call(func_idx, arg_count);
            }
        }
    }

    fn add_constant(&mut self, value: f64) -> u16 {
        // Reuse existing constants
        if let Some(idx) = self.program.constants.iter().position(|&c| c == value) {
            return idx as u16;
        }
        let idx = self.program.constants.len() as u16;
        self.program.constants.push(value);
        idx
    }

    fn resolve_var(&mut self, name: String) -> u16 {
        if let Some(&idx) = self.var_map.get(&name) {
            return idx;
        }
        let idx = self.program.var_names.len() as u16;
        self.program.var_names.push(name.clone());
        self.var_map.insert(name, idx);
        idx
    }

    fn resolve_func(&mut self, name: String) -> u16 {
        if let Some(&idx) = self.func_map.get(&name) {
            return idx;
        }
        let idx = self.program.func_names.len() as u16;
        
        // Register built-in function
        let func_ptr = match name.as_str() {
            "sin" => builtin_sin as BuiltinFn,
            "cos" => builtin_cos as BuiltinFn,
            "tan" => builtin_tan as BuiltinFn,
            "sqrt" => builtin_sqrt as BuiltinFn,
            "abs" => builtin_abs as BuiltinFn,
            "floor" => builtin_floor as BuiltinFn,
            "ceil" => builtin_ceil as BuiltinFn,
            "round" => builtin_round as BuiltinFn,
            "exp" => builtin_exp as BuiltinFn,
            "ln" => builtin_ln as BuiltinFn,
            "log10" => builtin_log10 as BuiltinFn,
            "max" => builtin_max as BuiltinFn,
            "min" => builtin_min as BuiltinFn,
            _ => {
                // Unknown function - will error at runtime
                builtin_unknown as BuiltinFn
            }
        };
        
        self.program.func_table.push(func_ptr);
        self.program.func_names.push(name.clone());
        self.func_map.insert(name, idx);
        idx
    }

    fn emit_load_const(&mut self, idx: u16) {
        self.program.instructions.push(OpCode::LoadConst as u8);
        self.emit_u16(idx);
    }

    fn emit_load_var(&mut self, idx: u16) {
        self.program.instructions.push(OpCode::LoadVar as u8);
        self.emit_u16(idx);
    }

    fn emit_binop(&mut self, op: BinaryOp) {
        let opcode = match op {
            BinaryOp::Add => OpCode::Add,
            BinaryOp::Sub => OpCode::Sub,
            BinaryOp::Mul => OpCode::Mul,
            BinaryOp::Div => OpCode::Div,
            BinaryOp::Pow => OpCode::Pow,
        };
        self.program.instructions.push(opcode as u8);
    }

    fn emit_unop(&mut self, op: UnaryOp) {
        let opcode = match op {
            UnaryOp::Neg => OpCode::Neg,
        };
        self.program.instructions.push(opcode as u8);
    }

    fn emit_call(&mut self, func_idx: u16, arg_count: u8) {
        self.program.instructions.push(OpCode::Call as u8);
        self.emit_u16(func_idx);
        self.program.instructions.push(arg_count);
    }

    fn emit_u16(&mut self, value: u16) {
        self.program.instructions.push((value >> 8) as u8);
        self.program.instructions.push((value & 0xFF) as u8);
    }
}

// Built-in function implementations
fn builtin_sin(args: &[f64]) -> f64 { args[0].sin() }
fn builtin_cos(args: &[f64]) -> f64 { args[0].cos() }
fn builtin_tan(args: &[f64]) -> f64 { args[0].tan() }
fn builtin_sqrt(args: &[f64]) -> f64 { args[0].sqrt() }
fn builtin_abs(args: &[f64]) -> f64 { args[0].abs() }
fn builtin_floor(args: &[f64]) -> f64 { args[0].floor() }
fn builtin_ceil(args: &[f64]) -> f64 { args[0].ceil() }
fn builtin_round(args: &[f64]) -> f64 { args[0].round() }
fn builtin_exp(args: &[f64]) -> f64 { args[0].exp() }
fn builtin_ln(args: &[f64]) -> f64 { args[0].ln() }
fn builtin_log10(args: &[f64]) -> f64 { args[0].log10() }
fn builtin_max(args: &[f64]) -> f64 {
    args.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
}
fn builtin_min(args: &[f64]) -> f64 {
    args.iter().fold(f64::INFINITY, |a, &b| a.min(b))
}
fn builtin_unknown(_args: &[f64]) -> f64 {
    f64::NAN
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn compile_expr(input: &str) -> Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse().unwrap();
        Compiler::new().compile(ast)
    }

    #[test]
    fn test_constant_folding_arithmetic() {
        let compiler = Compiler::new();
        
        // 2 + 3 should fold to 5
        let expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::Number(3.0)),
        };
        let optimized = compiler.optimize_expr(expr);
        assert_eq!(optimized, Expr::Number(5.0));
    }

    #[test]
    fn test_constant_folding_multiply_zero() {
        let compiler = Compiler::new();
        
        // x * 0 should fold to 0
        let expr = Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(Expr::Variable("x".to_string())),
            right: Box::new(Expr::Number(0.0)),
        };
        let optimized = compiler.optimize_expr(expr);
        assert_eq!(optimized, Expr::Number(0.0));
    }

    #[test]
    fn test_constant_folding_multiply_one() {
        let compiler = Compiler::new();
        
        // x * 1 should fold to x
        let expr = Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(Expr::Variable("x".to_string())),
            right: Box::new(Expr::Number(1.0)),
        };
        let optimized = compiler.optimize_expr(expr);
        assert_eq!(optimized, Expr::Variable("x".to_string()));
    }

    #[test]
    fn test_constant_folding_power() {
        let compiler = Compiler::new();
        
        // 2 ^ 3 should fold to 8
        let expr = Expr::Binary {
            op: BinaryOp::Pow,
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::Number(3.0)),
        };
        let optimized = compiler.optimize_expr(expr);
        assert_eq!(optimized, Expr::Number(8.0));
    }

    #[test]
    fn test_constant_folding_negation() {
        let compiler = Compiler::new();
        
        // -5 should fold to -5
        let expr = Expr::Unary {
            op: UnaryOp::Neg,
            expr: Box::new(Expr::Number(5.0)),
        };
        let optimized = compiler.optimize_expr(expr);
        assert_eq!(optimized, Expr::Number(-5.0));
    }

    #[test]
    fn test_constant_reuse() {
        // Use a case where constants won't be folded
        let program = compile_expr("x + 1 + y + 1");
        // Should reuse constant 1.0
        assert_eq!(program.constants.len(), 1);
        assert_eq!(program.constants[0], 1.0);
    }

    #[test]
    fn test_variable_resolution() {
        let program = compile_expr("x + y + x");
        assert_eq!(program.var_names.len(), 2);
        assert!(program.var_names.contains(&"x".to_string()));
        assert!(program.var_names.contains(&"y".to_string()));
    }

    #[test]
    fn test_function_registration() {
        let program = compile_expr("sin(x) + cos(y)");
        assert_eq!(program.func_names.len(), 2);
        assert_eq!(program.func_table.len(), 2);
    }

    #[test]
    fn test_builtin_functions() {
        assert_eq!(builtin_sin(&[0.0]), 0.0);
        assert!((builtin_cos(&[0.0]) - 1.0).abs() < 1e-10);
        assert_eq!(builtin_sqrt(&[4.0]), 2.0);
        assert_eq!(builtin_abs(&[-5.0]), 5.0);
        assert_eq!(builtin_max(&[1.0, 5.0, 3.0]), 5.0);
        assert_eq!(builtin_min(&[1.0, 5.0, 3.0]), 1.0);
    }

    #[test]
    fn test_compact_bytecode_generation() {
        let program = compile_expr("2 + 3");
        // Should generate compact bytecode
        assert!(program.instructions.len() > 0);
        assert_eq!(program.instructions[0], OpCode::LoadConst as u8);
    }

    #[test]
    fn test_algebraic_optimization_add_zero() {
        let compiler = Compiler::new();
        let expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Variable("x".to_string())),
            right: Box::new(Expr::Number(0.0)),
        };
        let optimized = compiler.optimize_expr(expr);
        assert_eq!(optimized, Expr::Variable("x".to_string()));
    }

    #[test]
    fn test_division_by_zero_not_folded() {
        let compiler = Compiler::new();
        let expr = Expr::Binary {
            op: BinaryOp::Div,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(0.0)),
        };
        let optimized = compiler.optimize_expr(expr);
        // Should not fold division by zero
        assert!(matches!(optimized, Expr::Binary { .. }));
    }
}
