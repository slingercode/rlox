use crate::{token::{Tokens, Token}, rlox::Rlox};

pub struct Scanner<'a> {
    pub rlox: &'a mut Rlox,
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: u32,
    pub current: u32,
    pub line: u32,
}

impl Scanner<'_> {
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: Tokens::Eol,
            lexeme: String::new(),
            literal: String::new(),
            line: self.line,
        });

        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.get_current_char();
        println!("Char {}", c);

        match c {
            '(' => self.add_token(Tokens::LeftParen),
            ')' => self.add_token(Tokens::RightParen),
            '{' => self.add_token(Tokens::LeftBrace),
            '}' => self.add_token(Tokens::RightBrace),
            ',' => self.add_token(Tokens::Comma),
            '.' => self.add_token(Tokens::Dot),
            '-' => self.add_token(Tokens::Minus),
            '+' => self.add_token(Tokens::Plus),
            ';' => self.add_token(Tokens::Semicolon),
            '*' => self.add_token(Tokens::Star),
            '!' => {
                let token_matched = self.char_next_match('=');
                self.add_token(if token_matched { Tokens::BangEqual } else { Tokens::Bang });
            },
            '=' => {
                let token_matched = self.char_next_match('=');
                self.add_token(if token_matched { Tokens::EqualEqual } else { Tokens::Equal });
            },
            '<' => {
                let token_matched = self.char_next_match('=');
                self.add_token(if token_matched { Tokens::LessEqual } else { Tokens::Less });
            },
            '>' => {
                let token_matched = self.char_next_match('=');
                self.add_token(if token_matched { Tokens::GreaterEqual } else { Tokens::Greater });
            },
            '/' => {
                if self.char_next_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance()
                    }
                } else {
                    self.add_token(Tokens::Slash);
                }
            },
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {},
            _ => self.rlox.error(self.line, "Unexpected character"),
        }

        self.advance();
        println!("");
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    fn get_current_char(&self) -> char {
        return self.source.chars().nth(
            self.current.try_into().unwrap()
        ).unwrap();
    }

    fn get_char_plus_one(&self) -> char {
        return self.source.chars().nth(
            (self.current + 1).try_into().unwrap()
        ).unwrap();
    }

    fn char_next_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let char = self.get_char_plus_one();

        if char != expected {
            return false;
        }

        return true;
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.get_char_plus_one();
        }
    }

    fn add_token(&mut self, token: Tokens) {
        let text = String::from(&self.source);
        let new_text = &text[self.start.try_into().unwrap()..self.current.try_into().unwrap()];

        self.tokens.push(Token {
            token_type: token,
            lexeme: new_text.to_string(),
            literal: String::new(),
            line: self.line,
        });
    }
}
