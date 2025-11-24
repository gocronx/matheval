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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_numbers() {
        let mut lexer = Lexer::new("123 45.67 0.5 .25");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(123.0));
        assert_eq!(lexer.next_token().unwrap(), Token::Number(45.67));
        assert_eq!(lexer.next_token().unwrap(), Token::Number(0.5));
        assert_eq!(lexer.next_token().unwrap(), Token::Number(0.25));
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_operators() {
        let mut lexer = Lexer::new("+ - * / ^");
        assert_eq!(lexer.next_token().unwrap(), Token::Plus);
        assert_eq!(lexer.next_token().unwrap(), Token::Minus);
        assert_eq!(lexer.next_token().unwrap(), Token::Star);
        assert_eq!(lexer.next_token().unwrap(), Token::Slash);
        assert_eq!(lexer.next_token().unwrap(), Token::Caret);
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_parentheses() {
        let mut lexer = Lexer::new("( )");
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_identifiers() {
        let mut lexer = Lexer::new("x foo bar_123 _test");
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("foo".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("bar_123".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("_test".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_expression() {
        let mut lexer = Lexer::new("x + 2 * sin(y)");
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Plus);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(2.0));
        assert_eq!(lexer.next_token().unwrap(), Token::Star);
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("sin".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("y".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_whitespace() {
        let mut lexer = Lexer::new("  1  +  2  ");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(1.0));
        assert_eq!(lexer.next_token().unwrap(), Token::Plus);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(2.0));
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_function_call() {
        let mut lexer = Lexer::new("max(1, 2, 3)");
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("max".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(1.0));
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(2.0));
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(3.0));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lexer_invalid_number() {
        let mut lexer = Lexer::new("1.2.3");
        assert!(lexer.next_token().is_err());
    }

    #[test]
    fn test_lexer_invalid_character() {
        let mut lexer = Lexer::new("@");
        let result = lexer.next_token();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }

    #[test]
    fn test_lexer_iterator() {
        let lexer = Lexer::new("1 + 2");
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].as_ref().unwrap(), &Token::Number(1.0));
        assert_eq!(tokens[1].as_ref().unwrap(), &Token::Plus);
        assert_eq!(tokens[2].as_ref().unwrap(), &Token::Number(2.0));
    }

    #[test]
    fn test_lexer_empty_input() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }
}
