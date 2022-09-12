use std::collections::HashMap;

use crate::token::{Tokens, Token, Literal};

pub(crate) struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end(self.current) {
            self.start = self.current;
            let current_char = self.get_char(self.current);
            self.scan(current_char);
        }

        self.tokens.push(Token { token_type: Tokens::Eol, literal: Literal::void() });

        &self.tokens
    }

    fn scan(&mut self, c: char) {
        match c {
            '(' => { self.scan_single(Tokens::LeftParen) },
            ')' => { self.scan_single(Tokens::RightParen) },
            '{' => { self.scan_single(Tokens::LeftBrace) },
            '}' => { self.scan_single(Tokens::RightBrace) },
            ',' => { self.scan_single(Tokens::Comma) },
            '.' => { self.scan_single(Tokens::Dot) },
            '-' => { self.scan_single(Tokens::Minus) },
            '+' => { self.scan_single(Tokens::Plus) },
            ';' => { self.scan_single(Tokens::Semicolon) },
            '*' => { self.scan_single(Tokens::Star) },
            '!' => { self.scan_multiple('=', Tokens::BangEqual, Tokens::Bang) },
            '=' => { self.scan_multiple('=', Tokens::EqualEqual, Tokens::Equal) },
            '<' => { self.scan_multiple('=', Tokens::LessEqual, Tokens::Less) },
            '>' => { self.scan_multiple('=', Tokens::GreaterEqual, Tokens::Greater) },
            '/' => { self.scan_slash_or_comment() },
            '"' => { self.scan_string() },
            '0'..='9' => { self.scan_number() },
            'a'..='z' | 'A'..='Z' | '_' => { self.scan_identifier() },
            '\n' => { self.scan_new_line() },
            ' ' | '\r' | '\t' => { self.advance(1) },
            _ => { self.scan_default() },
        }
    }

    fn scan_single(&mut self, token: Tokens) {
        self.add_token(token);
        self.advance(1);
    }

    fn scan_multiple(&mut self, expected: char, token_match: Tokens, token_default: Tokens) {
        let token_matched = self.char_next(expected);

        if token_matched {
            self.add_token(token_match);
            self.advance(2);
        } else {
            self.add_token(token_default);
            self.advance(1);
        }
    }

    fn scan_slash_or_comment(&mut self) {
        if self.char_next('/') {
            while self.peek(self.current + 1) != '\n' && !self.is_at_end(self.current + 1) {
                self.advance(1);
            }

            self.advance(1);
        } else {
            self.add_token(Tokens::Slash);
            self.advance(1);
        }
    }

    fn scan_string(&mut self) {
        while self.peek(self.current + 1) != '"' && !self.is_at_end(self.current + 1) {
            if self.peek(self.current + 1) == '\n' { self.line += 1 }
            self.advance(1);
        }

        self.advance(1);

        if self.is_at_end(self.current) {
            println!("Interminated string\n");
        } else {
            let literal = self.get_literal(self.start + 1, self.current);
            self.add_token_with_string_literal(Tokens::String, literal);
            self.advance(1);
        }
    }

    fn scan_number(&mut self) {
        while self.peek(self.current + 1).is_numeric() { self.advance(1) }

        self.advance(1);

        if self.peek(self.current) == '.' && self.peek(self.current + 1).is_numeric() {
            while self.peek(self.current + 1).is_numeric() { self.advance(1) }
            self.advance(1);
        }

        let literal = self.get_literal(self.start, self.current);
        let number = literal.parse::<f64>().unwrap();
        self.add_token_with_numeric_literal(Tokens::Number, number);
    }

    fn scan_identifier(&mut self) {
        while self.peek(self.current + 1).is_alphanumeric() || self.peek(self.current + 1) == '_' { self.advance(1) }

        self.advance(1);

        let literal = self.get_literal(self.start, self.current);

        let keywords = self.generate_keywords();

        match keywords.get(&literal) {
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
                    _ => self.add_token_with_string_literal(Tokens::Identifier, literal.to_string()),
                }
            },
            _ => self.add_token_with_string_literal(Tokens::Identifier, literal.to_string()),
        };
    }

    fn scan_new_line(&mut self) {
        self.line += 1;
        self.advance(1);
    }

    fn scan_default(&mut self) {
        let s = self.get_char(self.current+1);
        println!("Unexpected character {}", s);
        self.advance(1);
    }

    fn add_token(&mut self, token: Tokens) {
        self.tokens.push(Token { token_type: token, literal: Literal::void() });
    }

    fn add_token_with_string_literal(&mut self, token: Tokens, string: String) {
        self.tokens.push(Token { token_type: token, literal: Literal { string, number: 0.0 } });
    }

    fn add_token_with_numeric_literal(&mut self, token: Tokens, number: f64) {
        self.tokens.push(Token { token_type: token, literal: Literal { string: String::new(), number } });
    }

    fn generate_keywords(&self) -> HashMap<String, Tokens>{
        let mut keywords = HashMap::new();

        keywords.insert(String::from("and"), Tokens::And);
        keywords.insert(String::from("class"), Tokens::Class);
        keywords.insert(String::from("else"), Tokens::Else);
        keywords.insert(String::from("false"), Tokens::False);
        keywords.insert(String::from("fun"), Tokens::Fun);
        keywords.insert(String::from("for"), Tokens::For);
        keywords.insert(String::from("if"), Tokens::If);
        keywords.insert(String::from("nil"), Tokens::Nil);
        keywords.insert(String::from("or"), Tokens::Or);
        keywords.insert(String::from("print"), Tokens::Print);
        keywords.insert(String::from("return"), Tokens::Return);
        keywords.insert(String::from("super"), Tokens::Super);
        keywords.insert(String::from("this"), Tokens::This);
        keywords.insert(String::from("true"), Tokens::True);
        keywords.insert(String::from("var"), Tokens::Var);
        keywords.insert(String::from("while"), Tokens::While);

        keywords
    }
}

trait ScannerHelpers {
    fn advance(&mut self, amount: u32);

    fn is_at_end(&self, on: u32) -> bool;

    fn get_char(&self, on: u32) -> char;

    fn char_next(&self, is: char) -> bool;

    fn peek(&self, on: u32) -> char;

    fn get_literal(&self, start: u32, end: u32) -> String;
}

impl ScannerHelpers for Scanner {
    fn advance(&mut self, amount: u32) {
        self.current += amount;
    }

    fn is_at_end(&self, on: u32) -> bool {
        on >= self.source.len().try_into().unwrap()
    }

    fn get_char(&self, on: u32) -> char {
        self.source.chars().nth(on.try_into().unwrap()).unwrap()
    }

    fn char_next(&self, is: char) -> bool {
        if self.is_at_end(self.current + 1) {
            return false;
        }
    
        let char = self.get_char(self.current + 1);
    
        if char != is {
            return false;
        }
    
        return true;
    }

    fn peek(&self, on: u32) -> char {
        if self.is_at_end(on) { '\0' } else { self.get_char(on) }
    }

    fn get_literal(&self, start: u32, end: u32) -> String {
        let text = String::from(&self.source);
        let literal = &text[start.try_into().unwrap()..end.try_into().unwrap()];
        literal.to_string()
    }
}
