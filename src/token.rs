#[derive(Debug)]
pub(crate) enum Tokens {
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
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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
    Eol,
}

pub(crate) struct Token {
    pub(crate) token_type: Tokens,
    pub(crate) literal: Literal,
}

pub (crate) struct Literal {
    pub(crate) string: String,
    pub(crate) number: f64,
}

impl Literal {
    pub(crate) fn void() -> Self {
        Self {
            string: String::new(),
            number: 0.0,
        }
    }
}
