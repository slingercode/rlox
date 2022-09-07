#[derive(Debug)]
pub enum Tokens {
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

pub struct Token {
    pub token_type: Tokens,
    pub lexeme: String,
    pub literal: String,
    pub num_literal: f64,
    pub line: u32,
}

impl Token {
    // fn to_string(&self) -> String {
    //     let mut value = String::new();

    //     // value.push_str(&self.token_type);
    //     value.push_str(" ");
    //     value.push_str(&self.lexeme);
    //     value.push_str(" ");
    //     value.push_str(&self.literal);

    //     return value;
    // }
}
