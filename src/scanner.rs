use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenType {
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

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

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

    Error,
    Eof,
}

#[derive(Copy, Clone)]
pub struct Token<'sc> {
    pub kind: TokenType,
    pub line: usize,
    pub lexeme: &'sc str,
}

impl<'sc> Token<'sc> {
    pub fn new() -> Self {
        Self {
            kind: TokenType::Nil,
            line: 0,
            lexeme: "nil",
        }
    }
}

pub struct Scanner<'sc> {
    keywords: HashMap<&'static str, TokenType>,
    code: &'sc str,
    start: usize,
    current: usize,
    pub line: usize,
}

impl<'sc> Scanner<'sc> {
    pub fn new(code: &'sc str) -> Self {
        let mut keywords = HashMap::with_capacity(16);
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Self {
            keywords,
            code,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_token(&mut self) -> Token<'sc> {
        println!("{}, {}", self.current, self.code.len());

        self.skip_whitespace();
        self.start = self.current;
        
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b';' => self.make_token(TokenType::Semicolon),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),

            b'!' if self.matches(b'=') => self.make_token(TokenType::BangEqual),
            b'!' => self.make_token(TokenType::Bang),
            b'=' if self.matches(b'=') => self.make_token(TokenType::EqualEqual),
            b'=' => self.make_token(TokenType::Equal),
            b'<' if self.matches(b'=') => self.make_token(TokenType::LessEqual),
            b'<' => self.make_token(TokenType::Less),
            b'>' if self.matches(b'=') => self.make_token(TokenType::GreaterEqual),
            b'>' => self.make_token(TokenType::Greater),

            b'"' => self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),

            _ => self.error_token("Unexpected character"),
        }
    }

    fn peek(&self) -> u8 {
        self.code.as_bytes()[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current == self.code.len() {
            b'\0'
        } else {
            self.code.as_bytes()[self.current + 1]
        }
    }

    fn advance(&mut self) -> u8 {
        let c: u8 = self.peek();
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.code.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let char = self.peek();
            let _: u8 = match char {
                b' ' | b'\r' | b'\t' => self.advance(),
                b'\n' => {
                    self.line += 1;
                    self.advance()
                }
                b'/' if self.peek_next() == b'/' => {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return;
                }
                _ => return,
            };
        }
    }

    fn identifier_type(&self) -> TokenType {
        self.keywords
            .get(&self.code[self.start..self.current])
            .cloned()
            .unwrap_or(TokenType::Identifier)
    }

    fn string(&mut self) -> Token<'sc> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated String")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn number(&mut self) -> Token<'sc> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        return self.make_token(TokenType::Number);
    }

    fn identifier(&mut self) -> Token<'sc> {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn make_token(&self, kind: TokenType) -> Token<'sc> {
        Token {
            kind: kind,
            line: self.line,
            lexeme: &self.code[self.start..self.current],
        }
    }

    fn error_token(&self, message: &'sc str) -> Token<'sc> {
        Token {
            kind: TokenType::Error,
            line: self.line,
            lexeme: &message,
        }
    }
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}
