use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        match self.input.peek() {
            None => Ok(Token::EOF),
            Some(&c) => match c {
                '+' => { self.input.next(); Ok(Token::Plus) }
                '-' => { self.input.next(); Ok(Token::Minus) }
                '*' => { self.input.next(); Ok(Token::Star) }
                '/' => { self.input.next(); Ok(Token::Slash) }
                '^' => { self.input.next(); Ok(Token::Caret) }
                '(' => { self.input.next(); Ok(Token::LParen) }
                ')' => { self.input.next(); Ok(Token::RParen) }
                ',' => { self.input.next(); Ok(Token::Comma) }
                '0'..='9' | '.' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                _ => Err(format!("Unexpected character: {}", c)),
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut number_str = String::new();
        let mut has_decimal = false;

        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) {
                number_str.push(c);
                self.input.next();
            } else if c == '.' {
                if has_decimal {
                    return Err("Invalid number: multiple decimal points".to_string());
                }
                has_decimal = true;
                number_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        number_str.parse::<f64>()
            .map(Token::Number)
            .map_err(|_| format!("Invalid number format: {}", number_str))
    }

    fn read_identifier(&mut self) -> Result<Token, String> {
        let mut ident_str = String::new();

        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        Ok(Token::Identifier(ident_str))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        match token {
            Ok(Token::EOF) => None,
            _ => Some(token),
        }
    }
}
