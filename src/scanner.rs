use crate::lox::Lox;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Scanner<'a> {
    source: String,
    lox: &'a mut Lox,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u16,
}

impl<'a> Scanner<'a> {
    pub fn from(source: String, lox: &'a mut Lox) -> Self {
        Self {
            source,
            lox,
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                self.tokens.push(token);
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
    }

    fn advance(&mut self) -> char {
        let result = self.source[self.current..].chars().next().unwrap();
        self.current += result.len_utf8();
        result
    }

    fn add_token(&self, token_type: TokenType, literal: Option<String>) -> Token {
        let text = &self.source[self.start..self.current];
        Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        }
    }

    fn match_ahead(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(c) = self.source[self.current..].chars().next() {
            if c != expected {
                return false;
            }
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current..].chars().next().unwrap())
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            self.source[self.current..].chars().nth(1)
        }
    }

    fn string(&mut self) -> Option<Token> {
        while let Some(c) = self.peek() {
            if c != '"' && !self.is_at_end() {
                if let Some(n) = self.peek() {
                    if n == '\n' {
                        self.line += 1;
                    }
                }
                self.advance();
            }

            if self.is_at_end() {
                self.lox
                    .error(self.line, "Unterminated string.".to_string());
                return None;
            }
        }

        self.advance();
        let val = self.source[self.start + 1..self.current - 1].to_string();
        Some(self.add_token(TokenType::String, Some(val)))
    }

    fn number(&mut self) -> Option<Token> {
        while let Some(c) = self.peek() {
            if !('0'..'9').contains(&c) {
                break;
            }
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().is_some() {
            self.advance();
        }

        while let Some(c) = self.peek() {
            if !('0'..'9').contains(&c) {
                break;
            }
            self.advance();
        }
        Some(self.add_token(
            TokenType::Number,
            Some(self.source[self.start..self.current].to_string()),
        ))
    }

    fn identifier(&mut self) -> Option<Token> {
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        Some(self.add_token(token_type, None))
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            '(' => Some(self.add_token(TokenType::LeftParen, None)),
            ')' => Some(self.add_token(TokenType::RightParen, None)),
            '{' => Some(self.add_token(TokenType::LeftBrace, None)),
            '}' => Some(self.add_token(TokenType::RightBrace, None)),
            ',' => Some(self.add_token(TokenType::Comma, None)),
            '.' => Some(self.add_token(TokenType::Dot, None)),
            '-' => Some(self.add_token(TokenType::Minus, None)),
            '+' => Some(self.add_token(TokenType::Plus, None)),
            ';' => Some(self.add_token(TokenType::SemiColon, None)),
            '*' => Some(self.add_token(TokenType::Star, None)),
            '!' => {
                let t = match self.match_ahead('=') {
                    true => self.add_token(TokenType::BangEqual, None),
                    false => self.add_token(TokenType::Bang, None),
                };
                Some(t)
            }
            '=' => {
                let t = match self.match_ahead('=') {
                    true => self.add_token(TokenType::EqualEqual, None),
                    false => self.add_token(TokenType::Equal, None),
                };
                Some(t)
            }
            '<' => {
                let t = match self.match_ahead('=') {
                    true => self.add_token(TokenType::LessEqual, None),
                    false => self.add_token(TokenType::Less, None),
                };
                Some(t)
            }
            '>' => {
                let t = match self.match_ahead('=') {
                    true => self.add_token(TokenType::GreaterEqual, None),
                    false => self.add_token(TokenType::Greater, None),
                };
                Some(t)
            }
            '/' => {
                if self.match_ahead('/') {
                    while let Some(c) = self.peek() {
                        if c == '\n' && self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                    None
                } else {
                    Some(self.add_token(TokenType::Slash, None))
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => self.string(),
            '0'..'9' => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            _c => {
                self.lox.error(
                    self.line,
                    format!("Unexpected character {}.", _c).to_string(),
                );
                None
            }
        }
    }
}
