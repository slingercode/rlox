#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Number,
    String,

    Eof,
}

#[derive(Debug)]
pub struct Literal {
    pub number: Option<f64>,
    pub string: Option<String>,
}

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub token_type: TokenType,
}
