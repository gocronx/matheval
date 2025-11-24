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
