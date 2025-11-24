#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Caret,      // ^
    LParen,     // (
    RParen,     // )
    Comma,      // ,
    EOF,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_equality() {
        assert_eq!(Token::Plus, Token::Plus);
        assert_eq!(Token::Number(42.0), Token::Number(42.0));
        assert_eq!(Token::Identifier("x".to_string()), Token::Identifier("x".to_string()));
        assert_ne!(Token::Plus, Token::Minus);
    }

    #[test]
    fn test_token_clone() {
        let token = Token::Identifier("test".to_string());
        let cloned = token.clone();
        assert_eq!(token, cloned);
    }

    #[test]
    fn test_token_debug() {
        let token = Token::Number(3.14);
        let debug_str = format!("{:?}", token);
        assert!(debug_str.contains("Number"));
        assert!(debug_str.contains("3.14"));
    }

    #[test]
    fn test_all_operators() {
        let operators = vec![
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::Caret,
            Token::LParen,
            Token::RParen,
            Token::Comma,
            Token::EOF,
        ];
        
        for op in operators {
            assert_eq!(op.clone(), op);
        }
    }
}
