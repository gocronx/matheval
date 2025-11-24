use crate::bytecode::{OpCode, Program};
use std::collections::HashMap;

pub struct Context {
    vars: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: f64) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.vars.get(name).cloned()
    }
}

pub struct VM<'a> {
    program: &'a Program,
    stack: Vec<f64>,
}

impl<'a> VM<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            stack: Vec::with_capacity(32),
        }
    }

    pub fn run(&mut self, context: &Context) -> Result<f64, String> {
        // Optimization: Pre-resolve variables to a Vec<f64> based on program.var_names
        // This makes the inner loop O(1) for variable access.
        let mut var_values = Vec::with_capacity(self.program.var_names.len());
        for name in &self.program.var_names {
            match context.get(name) {
                Some(v) => var_values.push(v),
                None => return Err(format!("Undefined variable: {}", name)),
            }
        }

        for op in &self.program.ops {
            match op {
                OpCode::LoadConst(v) => self.stack.push(*v),
                OpCode::LoadVar(idx) => {
                    // Direct index access! Fast!
                    self.stack.push(var_values[*idx as usize]);
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a + b);
                }
                OpCode::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a - b);
                }
                OpCode::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a * b);
                }
                OpCode::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a / b);
                }
                OpCode::Pow => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a.powf(b));
                }
                OpCode::Neg => {
                    let a = self.pop()?;
                    self.stack.push(-a);
                }
                OpCode::Call(func_idx, arg_count) => {
                    let func_name = &self.program.func_names[*func_idx as usize];
                    let mut args = Vec::with_capacity(*arg_count as usize);
                    for _ in 0..*arg_count {
                        args.push(self.pop()?);
                    }
                    args.reverse(); // Stack is LIFO, args are pushed left-to-right
                    
                    let result = self.call_builtin(func_name, &args)?;
                    self.stack.push(result);
                }
            }
        }

        self.pop()
    }

    fn pop(&mut self) -> Result<f64, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }

    fn call_builtin(&self, name: &str, args: &[f64]) -> Result<f64, String> {
        match name {
            "sin" => Ok(args[0].sin()),
            "cos" => Ok(args[0].cos()),
            "tan" => Ok(args[0].tan()),
            "sqrt" => Ok(args[0].sqrt()),
            "max" => Ok(args.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))),
            "min" => Ok(args.iter().fold(f64::INFINITY, |a, &b| a.min(b))),
            _ => Err(format!("Unknown function: {}", name)),
        }
    }
}
