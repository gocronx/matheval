use crate::token::Token;
use crate::lexer::Lexer;
use crate::ast::{Expr, BinaryOp, UnaryOp};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self, String> {
        let current_token = lexer.next_token()?;
        Ok(Self {
            lexer,
            current_token,
        })
    }

    fn advance(&mut self) -> Result<(), String> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression(0)
    }

    fn expression(&mut self, min_bp: u8) -> Result<Expr, String> {
        let mut left = self.nud()?;

        loop {
            let token = self.current_token.clone();
            if token == Token::EOF || token == Token::RParen || token == Token::Comma {
                break;
            }

            let (l_bp, r_bp) = self.bp_infix(&token);
            if l_bp < min_bp {
                break;
            }

            self.advance()?;
            left = self.led(left, token, r_bp)?;
        }

        Ok(left)
    }

    fn nud(&mut self) -> Result<Expr, String> {
        let token = self.current_token.clone();
        self.advance()?;

        match token {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Identifier(name) => {
                if self.current_token == Token::LParen {
                    self.advance()?;
                    let mut args = Vec::new();
                    if self.current_token != Token::RParen {
                        loop {
                            args.push(self.expression(0)?);
                            if self.current_token == Token::Comma {
                                self.advance()?;
                            } else {
                                break;
                            }
                        }
                    }
                    if self.current_token != Token::RParen {
                        return Err("Expected ')' after function arguments".to_string());
                    }
                    self.advance()?;
                    Ok(Expr::Call { func: name, args })
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            Token::Minus => {
                let ((), r_bp) = self.bp_prefix(&Token::Minus);
                let expr = self.expression(r_bp)?;
                Ok(Expr::Unary { op: UnaryOp::Neg, expr: Box::new(expr) })
            }
            Token::LParen => {
                let expr = self.expression(0)?;
                if self.current_token != Token::RParen {
                    return Err("Expected ')'".to_string());
                }
                self.advance()?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in nud: {:?}", token)),
        }
    }

    fn led(&mut self, left: Expr, token: Token, r_bp: u8) -> Result<Expr, String> {
        let right = self.expression(r_bp)?;
        match token {
            Token::Plus => Ok(Expr::Binary { op: BinaryOp::Add, left: Box::new(left), right: Box::new(right) }),
            Token::Minus => Ok(Expr::Binary { op: BinaryOp::Sub, left: Box::new(left), right: Box::new(right) }),
            Token::Star => Ok(Expr::Binary { op: BinaryOp::Mul, left: Box::new(left), right: Box::new(right) }),
            Token::Slash => Ok(Expr::Binary { op: BinaryOp::Div, left: Box::new(left), right: Box::new(right) }),
            Token::Caret => Ok(Expr::Binary { op: BinaryOp::Pow, left: Box::new(left), right: Box::new(right) }),
            _ => Err(format!("Unexpected token in led: {:?}", token)),
        }
    }

    fn bp_infix(&self, token: &Token) -> (u8, u8) {
        match token {
            Token::Plus | Token::Minus => (10, 11),
            Token::Star | Token::Slash => (20, 21),
            Token::Caret => (30, 29), // Right associative
            _ => (0, 0),
        }
    }

    fn bp_prefix(&self, token: &Token) -> ((), u8) {
        match token {
            Token::Minus => ((), 99),
            _ => ((), 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Result<Expr, String> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer)?;
        parser.parse()
    }

    #[test]
    fn test_parse_number() {
        let expr = parse("42").unwrap();
        assert_eq!(expr, Expr::Number(42.0));
    }

    #[test]
    fn test_parse_variable() {
        let expr = parse("x").unwrap();
        assert_eq!(expr, Expr::Variable("x".to_string()));
    }

    #[test]
    fn test_parse_addition() {
        let expr = parse("1 + 2").unwrap();
        assert!(matches!(expr, Expr::Binary { op: BinaryOp::Add, .. }));
    }

    #[test]
    fn test_parse_precedence() {
        // 2 + 3 * 4 should parse as 2 + (3 * 4)
        let expr = parse("2 + 3 * 4").unwrap();
        
        if let Expr::Binary { op: BinaryOp::Add, left, right } = expr {
            assert_eq!(*left, Expr::Number(2.0));
            assert!(matches!(*right, Expr::Binary { op: BinaryOp::Mul, .. }));
        } else {
            panic!("Expected Add at top level");
        }
    }

    #[test]
    fn test_parse_right_associativity() {
        // 2 ^ 3 ^ 4 should parse as 2 ^ (3 ^ 4)
        let expr = parse("2 ^ 3 ^ 4").unwrap();
        
        if let Expr::Binary { op: BinaryOp::Pow, left, right } = expr {
            assert_eq!(*left, Expr::Number(2.0));
            assert!(matches!(*right, Expr::Binary { op: BinaryOp::Pow, .. }));
        } else {
            panic!("Expected Pow at top level");
        }
    }

    #[test]
    fn test_parse_parentheses() {
        // (2 + 3) * 4 should parse as (2 + 3) * 4
        let expr = parse("(2 + 3) * 4").unwrap();
        
        if let Expr::Binary { op: BinaryOp::Mul, left, right } = expr {
            assert!(matches!(*left, Expr::Binary { op: BinaryOp::Add, .. }));
            assert_eq!(*right, Expr::Number(4.0));
        } else {
            panic!("Expected Mul at top level");
        }
    }

    #[test]
    fn test_parse_negation() {
        let expr = parse("-5").unwrap();
        assert!(matches!(expr, Expr::Unary { op: UnaryOp::Neg, .. }));
    }

    #[test]
    fn test_parse_function_call() {
        let expr = parse("sin(x)").unwrap();
        
        if let Expr::Call { func, args } = expr {
            assert_eq!(func, "sin");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], Expr::Variable("x".to_string()));
        } else {
            panic!("Expected Call expression");
        }
    }

    #[test]
    fn test_parse_function_multiple_args() {
        let expr = parse("max(1, 2, 3)").unwrap();
        
        if let Expr::Call { func, args } = expr {
            assert_eq!(func, "max");
            assert_eq!(args.len(), 3);
        } else {
            panic!("Expected Call expression");
        }
    }

    #[test]
    fn test_parse_function_no_args() {
        let expr = parse("rand()").unwrap();
        
        if let Expr::Call { func, args } = expr {
            assert_eq!(func, "rand");
            assert_eq!(args.len(), 0);
        } else {
            panic!("Expected Call expression");
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        let expr = parse("x + 2 * sin(y) - 3").unwrap();
        assert!(matches!(expr, Expr::Binary { .. }));
    }

    #[test]
    fn test_parse_nested_functions() {
        let expr = parse("sin(cos(x))").unwrap();
        
        if let Expr::Call { func, args } = expr {
            assert_eq!(func, "sin");
            assert!(matches!(args[0], Expr::Call { .. }));
        } else {
            panic!("Expected Call expression");
        }
    }

    #[test]
    fn test_parse_error_missing_rparen() {
        let result = parse("(1 + 2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_function_rparen() {
        let result = parse("sin(x");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_all_operators() {
        assert!(parse("1 + 2").is_ok());
        assert!(parse("1 - 2").is_ok());
        assert!(parse("1 * 2").is_ok());
        assert!(parse("1 / 2").is_ok());
        assert!(parse("1 ^ 2").is_ok());
    }

    #[test]
    fn test_parse_whitespace_insensitive() {
        let expr1 = parse("1+2").unwrap();
        let expr2 = parse("1 + 2").unwrap();
        let expr3 = parse("  1  +  2  ").unwrap();
        
        assert_eq!(expr1, expr2);
        assert_eq!(expr2, expr3);
    }
}
