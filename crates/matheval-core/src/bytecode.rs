/// Optimized bytecode representation with compact instruction encoding
/// and function pointer table for zero-overhead function calls.

/// Compact instruction set - uses u8 opcodes instead of large enums
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    LoadConst = 0,  // [u16: const_idx]
    LoadVar = 1,    // [u16: var_idx]
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
    Pow = 6,
    Neg = 7,
    Call = 8,       // [u16: func_idx, u8: arg_count]
}

/// Function signature for built-in functions
pub type BuiltinFn = fn(&[f64]) -> f64;

/// Metadata for function validation
#[derive(Debug, Clone, Copy)]
pub struct FunctionMetadata {
    /// Expected number of arguments (None = variadic)
    pub expected_args: Option<usize>,
}

impl FunctionMetadata {
    pub fn new(expected_args: Option<usize>) -> Self {
        Self { expected_args }
    }
    
    pub fn fixed(count: usize) -> Self {
        Self { expected_args: Some(count) }
    }
    
    pub fn variadic() -> Self {
        Self { expected_args: None }
    }
}

/// Compiled program with optimized bytecode
#[derive(Clone)]
pub struct Program {
    /// Compact instruction stream: [opcode, operands...]
    pub instructions: Vec<u8>,
    
    /// Constant pool for numeric literals
    pub constants: Vec<f64>,
    
    /// Variable name mapping (index -> name)
    pub var_names: Vec<String>,
    
    /// Function pointer table for O(1) function dispatch
    pub func_table: Vec<BuiltinFn>,
    
    /// Function names for debugging
    pub func_names: Vec<String>,
    
    /// Function metadata for validation
    pub func_metadata: Vec<FunctionMetadata>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            var_names: Vec::new(),
            func_table: Vec::new(),
            func_names: Vec::new(),
            func_metadata: Vec::new(),
        }
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Program")
            .field("instructions_len", &self.instructions.len())
            .field("constants", &self.constants)
            .field("var_names", &self.var_names)
            .field("func_names", &self.func_names)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_size() {
        // Ensure opcodes are compact (1 byte)
        assert_eq!(std::mem::size_of::<OpCode>(), 1);
    }

    #[test]
    fn test_opcode_values() {
        assert_eq!(OpCode::LoadConst as u8, 0);
        assert_eq!(OpCode::Add as u8, 2);
        assert_eq!(OpCode::Call as u8, 8);
    }

    #[test]
    fn test_program_creation() {
        let program = Program::new();
        assert_eq!(program.instructions.len(), 0);
        assert_eq!(program.constants.len(), 0);
        assert_eq!(program.var_names.len(), 0);
    }

    #[test]
    fn test_builtin_fn_signature() {
        fn test_fn(args: &[f64]) -> f64 {
            args.iter().sum()
        }
        
        let func: BuiltinFn = test_fn;
        let result = func(&[1.0, 2.0, 3.0]);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_program_clone() {
        let mut program = Program::new();
        program.constants.push(42.0);
        program.var_names.push("x".to_string());
        
        let cloned = program.clone();
        assert_eq!(cloned.constants, program.constants);
        assert_eq!(cloned.var_names, program.var_names);
    }
}
