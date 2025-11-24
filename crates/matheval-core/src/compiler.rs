use crate::ast::{Expr, BinaryOp, UnaryOp};
use crate::bytecode::{OpCode, Program};
use std::collections::HashMap;

pub struct Compiler {
    ops: Vec<OpCode>,
    var_map: HashMap<String, u16>,
    var_names: Vec<String>,
    func_map: HashMap<String, u16>,
    func_names: Vec<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            var_map: HashMap::new(),
            var_names: Vec::new(),
            func_map: HashMap::new(),
            func_names: Vec::new(),
        }
    }

    pub fn compile(mut self, expr: Expr) -> Program {
        self.compile_expr(expr);
        Program {
            ops: self.ops,
            consts: vec![], // Not used in this design
            var_names: self.var_names,
            func_names: self.func_names,
        }
    }

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Number(n) => self.emit(OpCode::LoadConst(n)),
            Expr::Variable(name) => {
                let idx = self.resolve_var(name);
                self.emit(OpCode::LoadVar(idx));
            }
            Expr::Binary { op, left, right } => {
                self.compile_expr(*left);
                self.compile_expr(*right);
                match op {
                    BinaryOp::Add => self.emit(OpCode::Add),
                    BinaryOp::Sub => self.emit(OpCode::Sub),
                    BinaryOp::Mul => self.emit(OpCode::Mul),
                    BinaryOp::Div => self.emit(OpCode::Div),
                    BinaryOp::Pow => self.emit(OpCode::Pow),
                }
            }
            Expr::Unary { op, expr } => {
                self.compile_expr(*expr);
                match op {
                    UnaryOp::Neg => self.emit(OpCode::Neg),
                }
            }
            Expr::Call { func, args } => {
                let arg_count = args.len() as u8;
                for arg in args {
                    self.compile_expr(arg);
                }
                let func_idx = self.resolve_func(func);
                self.emit(OpCode::Call(func_idx, arg_count));
            }
        }
    }

    fn emit(&mut self, op: OpCode) {
        self.ops.push(op);
    }

    fn resolve_var(&mut self, name: String) -> u16 {
        if let Some(&idx) = self.var_map.get(&name) {
            return idx;
        }
        let idx = self.var_names.len() as u16;
        self.var_names.push(name.clone());
        self.var_map.insert(name, idx);
        idx
    }

    fn resolve_func(&mut self, name: String) -> u16 {
        if let Some(&idx) = self.func_map.get(&name) {
            return idx;
        }
        let idx = self.func_names.len() as u16;
        self.func_names.push(name.clone());
        self.func_map.insert(name, idx);
        idx
    }
}
