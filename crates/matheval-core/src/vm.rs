use crate::bytecode::{OpCode, Program};

/// Optimized execution context using indexed array for O(1) variable access
#[derive(Debug, Clone)]
pub struct Context {
    /// Variable values indexed by their position in Program.var_names
    values: Vec<f64>,
}

impl Context {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    /// Create a context pre-sized for a specific program
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: vec![0.0; capacity],
        }
    }

    /// Set variable by index (O(1) - recommended for hot paths)
    pub fn set_by_index(&mut self, index: usize, value: f64) {
        if index >= self.values.len() {
            self.values.resize(index + 1, 0.0);
        }
        self.values[index] = value;
    }

    /// Set variable by name (requires program for name lookup)
    pub fn set(&mut self, name: &str, value: f64, program: &Program) {
        if let Some(index) = program.var_names.iter().position(|n| n == name) {
            self.set_by_index(index, value);
        } else {
            // Variable not in program, extend context
            self.values.push(value);
        }
    }

    /// Get variable by index
    pub fn get_by_index(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }

    /// Get variable by name
    pub fn get(&self, name: &str, program: &Program) -> Option<f64> {
        program.var_names.iter()
            .position(|n| n == name)
            .and_then(|idx| self.get_by_index(idx))
    }

    /// Get internal values slice
    pub(crate) fn values(&self) -> &[f64] {
        &self.values
    }
}

/// Stack-based virtual machine with optimized instruction dispatch
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
        // Ensure context has all required variables
        if context.values.len() < self.program.var_names.len() {
            return Err(format!(
                "Context missing variables: expected {}, got {}",
                self.program.var_names.len(),
                context.values.len()
            ));
        }

        let var_values = context.values();
        let instructions = &self.program.instructions;
        let constants = &self.program.constants;
        let func_table = &self.program.func_table;

        let mut pc = 0; // Program counter
        
        while pc < instructions.len() {
            let opcode = instructions[pc];
            pc += 1;

            match opcode {
                op if op == OpCode::LoadConst as u8 => {
                    let idx = self.read_u16(instructions, &mut pc);
                    self.stack.push(constants[idx as usize]);
                }
                op if op == OpCode::LoadVar as u8 => {
                    let idx = self.read_u16(instructions, &mut pc);
                    self.stack.push(var_values[idx as usize]);
                }
                op if op == OpCode::Add as u8 => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a + b);
                }
                op if op == OpCode::Sub as u8 => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a - b);
                }
                op if op == OpCode::Mul as u8 => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a * b);
                }
                op if op == OpCode::Div as u8 => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    self.stack.push(a / b);
                }
                op if op == OpCode::Pow as u8 => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a.powf(b));
                }
                op if op == OpCode::Neg as u8 => {
                    let a = self.pop()?;
                    self.stack.push(-a);
                }
                op if op == OpCode::Call as u8 => {
                    let func_idx = self.read_u16(instructions, &mut pc) as usize;
                    let arg_count = instructions[pc] as usize;
                    pc += 1;

                    if func_idx >= func_table.len() {
                        return Err(format!("Invalid function index: {}", func_idx));
                    }

                    // Read args directly from stack without allocation
                    let stack_len = self.stack.len();
                    if stack_len < arg_count {
                        return Err("Stack underflow in function call".to_string());
                    }
                    
                    let args_start = stack_len - arg_count;
                    let result = func_table[func_idx](&self.stack[args_start..]);
                    
                    // Pop args and push result
                    self.stack.truncate(args_start);
                    self.stack.push(result);
                }
                _ => return Err(format!("Unknown opcode: {}", opcode)),
            }
        }

        self.pop()
    }

    #[inline]
    fn read_u16(&self, instructions: &[u8], pc: &mut usize) -> u16 {
        let high = instructions[*pc] as u16;
        let low = instructions[*pc + 1] as u16;
        *pc += 2;
        (high << 8) | low
    }

    #[inline]
    fn pop(&mut self) -> Result<f64, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }

    /// Batch evaluation: evaluate the same program with multiple variable sets
    /// This is much more efficient than calling eval() in a loop
    /// 
    /// # Arguments
    /// * `var_sets` - A slice where each inner slice contains variable values for one evaluation
    ///                Each inner slice must have length equal to program.var_names.len()
    /// 
    /// # Returns
    /// A vector of results, one for each variable set
    /// 
    /// # Example
    /// ```ignore
    /// let program = compiler.compile("x * 2 + y").unwrap();
    /// let var_sets = vec![
    ///     vec![1.0, 2.0],  // x=1, y=2 -> result: 4
    ///     vec![3.0, 4.0],  // x=3, y=4 -> result: 10
    ///     vec![5.0, 6.0],  // x=5, y=6 -> result: 16
    /// ];
    /// let results = program.eval_batch(&var_sets).unwrap();
    /// ```
    pub fn run_batch(&mut self, var_sets: &[&[f64]]) -> Result<Vec<f64>, String> {
        let expected_var_count = self.program.var_names.len();
        let mut results = Vec::with_capacity(var_sets.len());

        for (i, var_values) in var_sets.iter().enumerate() {
            if var_values.len() != expected_var_count {
                return Err(format!(
                    "Variable set {} has {} values, expected {}",
                    i,
                    var_values.len(),
                    expected_var_count
                ));
            }

            // Reset stack for each evaluation
            self.stack.clear();

            let instructions = &self.program.instructions;
            let constants = &self.program.constants;
            let func_table = &self.program.func_table;

            let mut pc = 0;

            while pc < instructions.len() {
                let opcode = instructions[pc];
                pc += 1;

                match opcode {
                    op if op == OpCode::LoadConst as u8 => {
                        let idx = self.read_u16(instructions, &mut pc);
                        self.stack.push(constants[idx as usize]);
                    }
                    op if op == OpCode::LoadVar as u8 => {
                        let idx = self.read_u16(instructions, &mut pc);
                        self.stack.push(var_values[idx as usize]);
                    }
                    op if op == OpCode::Add as u8 => {
                        let b = self.pop()?;
                        let a = self.pop()?;
                        self.stack.push(a + b);
                    }
                    op if op == OpCode::Sub as u8 => {
                        let b = self.pop()?;
                        let a = self.pop()?;
                        self.stack.push(a - b);
                    }
                    op if op == OpCode::Mul as u8 => {
                        let b = self.pop()?;
                        let a = self.pop()?;
                        self.stack.push(a * b);
                    }
                    op if op == OpCode::Div as u8 => {
                        let b = self.pop()?;
                        let a = self.pop()?;
                        if b == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        self.stack.push(a / b);
                    }
                    op if op == OpCode::Pow as u8 => {
                        let b = self.pop()?;
                        let a = self.pop()?;
                        self.stack.push(a.powf(b));
                    }
                    op if op == OpCode::Neg as u8 => {
                        let a = self.pop()?;
                        self.stack.push(-a);
                    }
                    op if op == OpCode::Call as u8 => {
                        let func_idx = self.read_u16(instructions, &mut pc) as usize;
                        let arg_count = instructions[pc] as usize;
                        pc += 1;

                        if func_idx >= func_table.len() {
                            return Err(format!("Invalid function index: {}", func_idx));
                        }

                        let stack_len = self.stack.len();
                        if stack_len < arg_count {
                            return Err("Stack underflow in function call".to_string());
                        }

                        let args_start = stack_len - arg_count;
                        let result = func_table[func_idx](&self.stack[args_start..]);

                        self.stack.truncate(args_start);
                        self.stack.push(result);
                    }
                    _ => return Err(format!("Unknown opcode: {}", opcode)),
                }
            }

            results.push(self.pop()?);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = Context::new();
        assert_eq!(ctx.values.len(), 0);

        let ctx = Context::with_capacity(5);
        assert_eq!(ctx.values.len(), 5);
    }

    #[test]
    fn test_context_set_by_index() {
        let mut ctx = Context::new();
        ctx.set_by_index(0, 42.0);
        ctx.set_by_index(2, 99.0);
        
        assert_eq!(ctx.get_by_index(0), Some(42.0));
        assert_eq!(ctx.get_by_index(1), Some(0.0));
        assert_eq!(ctx.get_by_index(2), Some(99.0));
    }

    #[test]
    fn test_context_with_program() {
        let mut program = Program::new();
        program.var_names.push("x".to_string());
        program.var_names.push("y".to_string());

        let mut ctx = Context::new();
        ctx.set("x", 10.0, &program);
        ctx.set("y", 20.0, &program);

        assert_eq!(ctx.get("x", &program), Some(10.0));
        assert_eq!(ctx.get("y", &program), Some(20.0));
        assert_eq!(ctx.get("z", &program), None);
    }

    #[test]
    fn test_vm_simple_arithmetic() {
        // Program: 2 + 3
        let mut program = Program::new();
        program.constants.push(2.0);
        program.constants.push(3.0);
        
        program.instructions.push(OpCode::LoadConst as u8);
        program.instructions.extend_from_slice(&[0, 0]); // const[0]
        program.instructions.push(OpCode::LoadConst as u8);
        program.instructions.extend_from_slice(&[0, 1]); // const[1]
        program.instructions.push(OpCode::Add as u8);

        let ctx = Context::new();
        let mut vm = VM::new(&program);
        let result = vm.run(&ctx).unwrap();
        
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_vm_variable_load() {
        // Program: x + y
        let mut program = Program::new();
        program.var_names.push("x".to_string());
        program.var_names.push("y".to_string());
        
        program.instructions.push(OpCode::LoadVar as u8);
        program.instructions.extend_from_slice(&[0, 0]); // var[0]
        program.instructions.push(OpCode::LoadVar as u8);
        program.instructions.extend_from_slice(&[0, 1]); // var[1]
        program.instructions.push(OpCode::Add as u8);

        let mut ctx = Context::with_capacity(2);
        ctx.set_by_index(0, 10.0);
        ctx.set_by_index(1, 20.0);

        let mut vm = VM::new(&program);
        let result = vm.run(&ctx).unwrap();
        
        assert_eq!(result, 30.0);
    }

    #[test]
    fn test_vm_negation() {
        // Program: -5
        let mut program = Program::new();
        program.constants.push(5.0);
        
        program.instructions.push(OpCode::LoadConst as u8);
        program.instructions.extend_from_slice(&[0, 0]);
        program.instructions.push(OpCode::Neg as u8);

        let ctx = Context::new();
        let mut vm = VM::new(&program);
        let result = vm.run(&ctx).unwrap();
        
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_vm_division_by_zero() {
        // Program: 1 / 0
        let mut program = Program::new();
        program.constants.push(1.0);
        program.constants.push(0.0);
        
        program.instructions.push(OpCode::LoadConst as u8);
        program.instructions.extend_from_slice(&[0, 0]);
        program.instructions.push(OpCode::LoadConst as u8);
        program.instructions.extend_from_slice(&[0, 1]);
        program.instructions.push(OpCode::Div as u8);

        let ctx = Context::new();
        let mut vm = VM::new(&program);
        let result = vm.run(&ctx);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_vm_function_call() {
        fn test_max(args: &[f64]) -> f64 {
            args.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
        }

        // Program: max(2, 5)
        let mut program = Program::new();
        program.constants.push(2.0);
        program.constants.push(5.0);
        program.func_table.push(test_max);
        
        program.instructions.push(OpCode::LoadConst as u8);
        program.instructions.extend_from_slice(&[0, 0]);
        program.instructions.push(OpCode::LoadConst as u8);
        program.instructions.extend_from_slice(&[0, 1]);
        program.instructions.push(OpCode::Call as u8);
        program.instructions.extend_from_slice(&[0, 0]); // func[0]
        program.instructions.push(2); // 2 args

        let ctx = Context::new();
        let mut vm = VM::new(&program);
        let result = vm.run(&ctx).unwrap();
        
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_vm_stack_underflow() {
        let mut program = Program::new();
        program.instructions.push(OpCode::Add as u8);

        let ctx = Context::new();
        let mut vm = VM::new(&program);
        let result = vm.run(&ctx);
        
        assert!(result.is_err());
    }
}
