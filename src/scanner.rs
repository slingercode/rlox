use std::collections::HashMap;

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
            num_literal: 0.0,
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
                self.advance();
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
                        self.advance();
                    }
                } else {
                    self.add_token(Tokens::Slash);
                }
            },
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' { self.line += 1 }
                    self.advance();
                }
                
                if self.is_at_end() {
                    self.rlox.error(self.line, "Interminated string\n");
                    return;
                }
                
                self.advance();

                let text = String::from(&self.source);
                let literal = &text[(self.start + 1).try_into().unwrap()..self.current.try_into().unwrap()];

                self.add_token_with_literal(Tokens::String, literal.to_string());
            },
            '0'..='9' => {
                while self.peek().is_numeric() { self.advance(); }

                if self.peek() == '.' && self.peek_plus_one().is_numeric() {
                    self.advance();
                    while self.peek().is_numeric() { self.advance(); }
                    self.advance();
                } else {
                    self.advance();
                }

                let text = String::from(&self.source);
                let literal = &text[self.start.try_into().unwrap()..self.current.try_into().unwrap()];
                let number = literal.parse::<f64>().unwrap();
                self.add_token_with_number_literal(Tokens::Number, number);
                return;
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                while self.peek().is_alphanumeric() || self.peek() == '_' { self.advance(); }
                self.advance();

                let keywords = self.generate_keywords();

                let text = String::from(&self.source);
                let literal = &text[self.start.try_into().unwrap()..self.current.try_into().unwrap()];

                match keywords.get(literal) {
                    Some(keyword) => {
                        match keyword {
                            Tokens::And => self.add_token(Tokens::And),
                            Tokens::Class => self.add_token(Tokens::Class),
                            Tokens::Else => self.add_token(Tokens::Else),
                            Tokens::False => self.add_token(Tokens::False),
                            Tokens::Fun => self.add_token(Tokens::Fun),
                            Tokens::For => self.add_token(Tokens::For),
                            Tokens::If => self.add_token(Tokens::If),
                            Tokens::Nil => self.add_token(Tokens::Nil),
                            Tokens::Or => self.add_token(Tokens::Or),
                            Tokens::Print => self.add_token(Tokens::Print),
                            Tokens::Return => self.add_token(Tokens::Return),
                            Tokens::Super => self.add_token(Tokens::Super),
                            Tokens::This => self.add_token(Tokens::This),
                            Tokens::True => self.add_token(Tokens::True),
                            Tokens::Var => self.add_token(Tokens::Var),
                            Tokens::While => self.add_token(Tokens::While),
                            _ => self.add_token_with_literal(Tokens::Identifier, literal.to_string()),
                        }
                    },
                    _ => self.add_token_with_literal(Tokens::Identifier, literal.to_string()),
                };

                return;
            },
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {},
            _ => self.rlox.error(self.line, "Unexpected character"),
        }

        self.advance();
        println!("");
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    fn is_at_end_plus_one(&self) -> bool {
        (self.current + 1) >= self.source.len().try_into().unwrap()
    }

    fn is_at_end_plus_two(&self) -> bool {
        (self.current + 2) >= self.source.len().try_into().unwrap()
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

    fn get_char_plus_two(&self) -> char {
        return self.source.chars().nth(
            (self.current + 2).try_into().unwrap()
        ).unwrap();
    }

    fn char_next_match(&mut self, expected: char) -> bool {
        if self.is_at_end_plus_one() {
            return false;
        }

        let char = self.get_char_plus_one();

        if char != expected {
            return false;
        }

        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end_plus_one() {
            return '\0';
        } else {
            return self.get_char_plus_one();
        }
    }

    fn peek_plus_one(&self) -> char {
        if self.is_at_end_plus_two() {
            return '\0';
        } else {
            return self.get_char_plus_two();
        }
    }

    fn add_token(&mut self, token: Tokens) {
        let text = String::from(&self.source);
        let new_text = &text[self.start.try_into().unwrap()..self.current.try_into().unwrap()];

        self.tokens.push(Token {
            token_type: token,
            lexeme: new_text.to_string(),
            literal: String::new(),
            num_literal: 0.0,
            line: self.line,
        });
    }

    fn add_token_with_literal(&mut self, token: Tokens, literal: String) {
        let text = String::from(&self.source);
        let new_text = &text[self.start.try_into().unwrap()..self.current.try_into().unwrap()];

        self.tokens.push(Token {
            token_type: token,
            lexeme: new_text.to_string(),
            literal,
            num_literal: 0.0,
            line: self.line,
        });
    }

    fn add_token_with_number_literal(&mut self, token: Tokens, num_literal: f64) {
        let text = String::from(&self.source);
        let new_text = &text[self.start.try_into().unwrap()..self.current.try_into().unwrap()];

        self.tokens.push(Token {
            token_type: token,
            lexeme: new_text.to_string(),
            literal: String::new(),
            num_literal,
            line: self.line,
        });
    }

    fn generate_keywords(&self) -> HashMap<&str, Tokens>{
        let mut keywords = HashMap::new();

        keywords.insert("and", Tokens::And);
        keywords.insert("class", Tokens::Class);
        keywords.insert("else", Tokens::Else);
        keywords.insert("false", Tokens::False);
        keywords.insert("fun", Tokens::Fun);
        keywords.insert("for", Tokens::For);
        keywords.insert("if", Tokens::If);
        keywords.insert("nil", Tokens::Nil);
        keywords.insert("or", Tokens::Or);
        keywords.insert("print", Tokens::Print);
        keywords.insert("return", Tokens::Return);
        keywords.insert("super", Tokens::Super);
        keywords.insert("this", Tokens::This);
        keywords.insert("true", Tokens::True);
        keywords.insert("var", Tokens::Var);
        keywords.insert("while", Tokens::While);

        keywords
    }
}
