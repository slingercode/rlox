#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
    Comma,
    Dot,
    LeftBrace,
    LeftParen,
    Minus,
    Plus,
    RightBrace,
    RightParen,
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
    Identifier,
    Number,
    String,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

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
