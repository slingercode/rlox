use crate::token::{Literal, Token, TokenType, KEYWORDS};

pub struct Scanner {
    /// Current buffer position
    current: usize,
    /// Current line block
    line_block: usize,
    /// Current line
    line: usize,
    /// Raw stringify source code
    source: String,
    /// Start buffer position
    start: usize,
    /// Tokens scanned
    tokens: Vec<Token>,
}

impl Scanner {
    /// Create a new `Scanner` instance
    pub fn new(source: String) -> Self {
        Self {
            current: 0,
            line_block: 1,
            line: 1,
            source,
            start: 0,
            tokens: vec![],
        }
    }

    /// Initiates the scan process
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token(self.get_current_char());
        }

        self.create_token(TokenType::Eof);

        self.tokens
    }

    /// Method that handles the scan process for a character
    fn scan_token(&mut self, current_char: char) {
        match current_char {
            '(' => self.handle_single_token(TokenType::LeftParen),
            ')' => self.handle_single_token(TokenType::RightParen),
            '{' => self.handle_single_token(TokenType::LeftBrace),
            '}' => self.handle_single_token(TokenType::RightBrace),
            ',' => self.handle_single_token(TokenType::Comma),
            '.' => self.handle_single_token(TokenType::Dot),
            '-' => self.handle_single_token(TokenType::Minus),
            '+' => self.handle_single_token(TokenType::Plus),
            ';' => self.handle_single_token(TokenType::Semicolon),
            '*' => self.handle_single_token(TokenType::Star),
            '!' => self.handle_multiple_token('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.handle_multiple_token('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.handle_multiple_token('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.handle_multiple_token('=', TokenType::GreaterEqual, TokenType::Greater),
            '"' => self.handle_string_literal(),
            '0'..='9' => self.handle_number_literal(),
            'a'..='z' | 'A'..='Z' | '_' => self.handle_keyword(),
            '/' => self.handle_slash_token_or_comment(),
            ' ' | '\r' | '\t' => self.handle_whitespace(),
            '\n' => self.handle_line_break(),
            _ => self.handle_error(current_char),
        }
    }

    /// Verify if the EOF is reached
    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Advance the current and the line block buffer
    fn advance(&mut self) {
        self.current += 1;
        self.line_block += 1;
    }

    /// Gets a character in an specific position of the source code string
    fn get_char_on_position(&self, index: usize) -> char {
        self.source.chars().nth(index).unwrap_or('\0')
    }

    /// **Build for simplicity**
    ///
    /// Gets the char in the current position
    fn get_current_char(&self) -> char {
        self.get_char_on_position(self.current)
    }

    /// **Build for simplicity**
    ///
    /// Gets the char in the next position
    fn get_next_char(&self) -> char {
        self.get_char_on_position(self.current + 1)
    }

    /// Validate if the `expected_char` is the next in the sequence
    fn match_next_char(&mut self, expected_char: char) -> bool {
        if self.is_eof() {
            return false;
        }

        if self.get_next_char() == expected_char {
            return true;
        } else {
            return false;
        }
    }

    /// Create the lexeme of a new `Token` founded
    fn create_lexeme(&self) -> String {
        let lexeme = &self.source[self.start..self.current + 1];
        lexeme.to_string()
    }

    /// Create a new `Token` in the vector
    fn create_token(&mut self, token_type: TokenType) {
        if self.is_eof() {
            self.tokens.push(Token {
                lexeme: String::from("\0"),
                literal: None,
                token_type,
            })
        } else {
            self.tokens.push(Token {
                lexeme: self.create_lexeme(),
                literal: None,
                token_type,
            })
        }
    }

    /// **Handler method**
    ///
    /// Whitespace
    fn handle_whitespace(&mut self) {
        self.advance();
    }

    /// **Handler method**
    ///
    /// Line break
    fn handle_line_break(&mut self) {
        self.line += 1;
        self.line_block = 0;
        self.advance();
    }

    /// **Handler method**
    ///
    /// Single token
    fn handle_single_token(&mut self, token_type: TokenType) {
        self.create_token(token_type);
        self.advance();
    }

    /// **Handler method**
    ///
    /// Multiple token
    fn handle_multiple_token(
        &mut self,
        expected_char: char,
        token_if_match: TokenType,
        default_token: TokenType,
    ) {
        let mut token = default_token;

        if self.match_next_char(expected_char) {
            token = token_if_match;
            self.advance();
        }

        self.create_token(token);
        self.advance();
    }

    /// **Handler method**
    ///
    /// String literal
    fn handle_string_literal(&mut self) {
        while !self.is_eof() && self.get_next_char() != '"' {
            if self.get_next_char() == '\n' {
                self.handle_line_break();
            } else {
                self.advance();
            }
        }

        if self.is_eof() {
            self.handle_error('\0');
            return;
        }

        // This is the char `"`
        self.advance();

        let literal = &self.source[self.start + 1..self.current];

        self.tokens.push(Token {
            lexeme: String::from(""),
            literal: Some(Literal {
                number: None,
                string: Some(literal.to_string()),
            }),
            token_type: TokenType::String,
        });

        self.advance();
    }

    /// **Handler method**
    ///
    /// Number literal
    fn handle_number_literal(&mut self) {
        while self.get_next_char().is_numeric() {
            self.advance();
        }

        if self.get_next_char() == '.' && self.get_char_on_position(self.current + 2).is_numeric() {
            self.advance();

            while self.get_next_char().is_numeric() {
                self.advance();
            }
        }

        let literal = &self.source[self.start..self.current + 1];

        self.tokens.push(Token {
            lexeme: String::from(""),
            literal: Some(Literal {
                number: Some(literal.parse::<f64>().unwrap_or(0.0)),
                string: None,
            }),
            token_type: TokenType::Number,
        });

        self.advance();
    }

    /// **Handler method**
    ///
    /// Keywords (reserved words or identifiers)
    fn handle_keyword(&mut self) {
        while self.get_next_char().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current + 1];

        match KEYWORDS.get(text) {
            Some(token) => self.create_token(*token),
            None => self.create_token(TokenType::Identifier),
        }

        self.advance();
    }

    /// **Handler method**
    ///
    /// Comment token
    fn handle_slash_token_or_comment(&mut self) {
        if self.get_next_char() == '/' {
            self.advance();

            while !self.is_eof() && self.get_next_char() != '\n' {
                self.advance();
            }

            self.advance();
        } else {
            self.create_token(TokenType::Slash);
            self.advance();
        }
    }

    /// **Handler method**
    ///
    /// Error
    fn handle_error(&mut self, char: char) {
        eprintln!(
            "Invalid character '{char}' in {}:{}",
            self.line, self.line_block
        );

        self.advance();
    }
}
