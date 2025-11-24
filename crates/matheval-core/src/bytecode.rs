#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    LoadConst(f64),
    LoadVar(u16),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Neg,
    Call(u16, u8), // func_id, arg_count
}

#[derive(Debug, Clone)]
pub struct Program {
    pub ops: Vec<OpCode>,
    pub consts: Vec<f64>, // Not strictly needed if we embed f64 in OpCode, but good for large strings etc. 
                          // Here we embed f64 directly in OpCode::LoadConst for simplicity and cache locality.
    pub var_names: Vec<String>, // Map index -> name (for debugging or reverse lookup)
    pub func_names: Vec<String>, // Map func_id -> name
}
