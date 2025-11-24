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
