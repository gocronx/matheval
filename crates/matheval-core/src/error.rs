use std::fmt;

/// Position in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
    
    pub fn start() -> Self {
        Self { line: 1, column: 1, offset: 0 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Error kinds with detailed context
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // Lexer errors
    UnexpectedCharacter(char),
    InvalidNumber(String),
    
    // Parser errors
    UnexpectedToken(String),
    ExpectedToken { expected: String, found: String },
    MissingClosingParen,
    MissingFunctionClosingParen(String),
    
    // Compiler errors
    UnknownFunction(String),
    
    // Runtime errors
    DivisionByZero,
    StackUnderflow,
    UndefinedVariable(String),
    InvalidFunctionIndex(usize),
    WrongArgumentCount {
        function: String,
        expected: usize,
        got: usize,
    },
    VariableCountMismatch {
        expected: usize,
        got: usize,
    },
    UnknownOpcode(u8),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UnexpectedCharacter(ch) => {
                write!(f, "Unexpected character '{}'", ch)
            }
            ErrorKind::InvalidNumber(s) => {
                write!(f, "Invalid number format: '{}'", s)
            }
            ErrorKind::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {}", token)
            }
            ErrorKind::ExpectedToken { expected, found } => {
                write!(f, "Expected {}, but found {}", expected, found)
            }
            ErrorKind::MissingClosingParen => {
                write!(f, "Missing closing parenthesis ')'")
            }
            ErrorKind::MissingFunctionClosingParen(func) => {
                write!(f, "Missing closing parenthesis ')' in function call '{}'", func)
            }
            ErrorKind::UnknownFunction(name) => {
                write!(f, "Unknown function: '{}'", name)
            }
            ErrorKind::DivisionByZero => {
                write!(f, "Division by zero")
            }
            ErrorKind::StackUnderflow => {
                write!(f, "Stack underflow (internal error)")
            }
            ErrorKind::UndefinedVariable(name) => {
                write!(f, "Undefined variable: '{}'", name)
            }
            ErrorKind::InvalidFunctionIndex(idx) => {
                write!(f, "Invalid function index: {} (internal error)", idx)
            }
            ErrorKind::WrongArgumentCount { function, expected, got } => {
                write!(
                    f,
                    "Function '{}' expects {} argument{}, but got {}",
                    function,
                    expected,
                    if *expected == 1 { "" } else { "s" },
                    got
                )
            }
            ErrorKind::VariableCountMismatch { expected, got } => {
                write!(
                    f,
                    "Expected {} variable{}, but got {}",
                    expected,
                    if *expected == 1 { "" } else { "s" },
                    got
                )
            }
            ErrorKind::UnknownOpcode(op) => {
                write!(f, "Unknown opcode: {} (internal error)", op)
            }
        }
    }
}

/// Main error type with position information
#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Option<Position>,
    pub source: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            position: None,
            source: None,
        }
    }
    
    pub fn with_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
    
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    
    /// Create a lexer error
    pub fn unexpected_char(ch: char, position: Position) -> Self {
        Self::new(ErrorKind::UnexpectedCharacter(ch))
            .with_position(position)
    }
    
    /// Create a parser error
    pub fn expected_token(expected: &str, found: &str, position: Position) -> Self {
        Self::new(ErrorKind::ExpectedToken {
            expected: expected.to_string(),
            found: found.to_string(),
        })
        .with_position(position)
    }
    
    /// Create a runtime error
    pub fn division_by_zero() -> Self {
        Self::new(ErrorKind::DivisionByZero)
    }
    
    pub fn wrong_arg_count(function: &str, expected: usize, got: usize) -> Self {
        Self::new(ErrorKind::WrongArgumentCount {
            function: function.to_string(),
            expected,
            got,
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Main error message
        write!(f, "Error: {}", self.kind)?;
        
        // Add position if available
        if let Some(pos) = self.position {
            write!(f, " at {}", pos)?;
        }
        
        // Add source context if available
        if let Some(source) = &self.source {
            if let Some(pos) = self.position {
                write!(f, "\n\n{}", self.format_source_context(source, pos))?;
            }
        }
        
        // Add helpful hint
        if let Some(hint) = self.hint() {
            write!(f, "\n\nHint: {}", hint)?;
        }
        
        Ok(())
    }
}

impl Error {
    fn format_source_context(&self, source: &str, pos: Position) -> String {
        let lines: Vec<&str> = source.lines().collect();
        if pos.line == 0 || pos.line > lines.len() {
            return String::new();
        }
        
        let line_idx = pos.line - 1;
        let line = lines[line_idx];
        
        let mut result = String::new();
        
        // Show line number and content
        result.push_str(&format!("  {} | {}\n", pos.line, line));
        
        // Show pointer to error position
        let padding = format!("  {} | ", pos.line).len();
        result.push_str(&" ".repeat(padding + pos.column - 1));
        result.push_str("^");
        
        result
    }
    
    fn hint(&self) -> Option<&str> {
        match &self.kind {
            ErrorKind::DivisionByZero => {
                Some("Make sure the divisor is not zero")
            }
            ErrorKind::MissingClosingParen | ErrorKind::MissingFunctionClosingParen(_) => {
                Some("Check that all opening parentheses '(' have matching closing parentheses ')'")
            }
            ErrorKind::UndefinedVariable(_) => {
                Some("Make sure all variables are defined in the context before evaluation")
            }
            ErrorKind::WrongArgumentCount { .. } => {
                Some("Check the function documentation for the correct number of arguments")
            }
            ErrorKind::UnknownFunction(_) => {
                Some("Available functions: sin, cos, tan, sqrt, abs, floor, ceil, round, exp, ln, log10, max, min")
            }
            _ => None,
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_display() {
        let pos = Position::new(5, 10, 42);
        assert_eq!(pos.to_string(), "line 5, column 10");
    }

    #[test]
    fn test_error_kind_display() {
        let err = ErrorKind::WrongArgumentCount {
            function: "sin".to_string(),
            expected: 1,
            got: 2,
        };
        assert_eq!(err.to_string(), "Function 'sin' expects 1 argument, but got 2");
    }

    #[test]
    fn test_error_with_position() {
        let err = Error::unexpected_char('@', Position::new(1, 5, 4));
        assert!(err.to_string().contains("Unexpected character '@'"));
        assert!(err.to_string().contains("line 1, column 5"));
    }

    #[test]
    fn test_error_with_source_context() {
        let source = "x + @ - y";
        let err = Error::unexpected_char('@', Position::new(1, 5, 4))
            .with_source(source.to_string());
        
        let display = err.to_string();
        assert!(display.contains("x + @ - y"));
        assert!(display.contains("^"));
    }

    #[test]
    fn test_error_hints() {
        let err = Error::division_by_zero();
        assert!(err.hint().is_some());
        assert!(err.hint().unwrap().contains("divisor"));
    }
}
