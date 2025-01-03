use std::mem::uninitialized;

use crate::expr::LiteralValue;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::Expr;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

#[derive(Debug)]
struct ParseError {}

impl Parser {
    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    pub fn equality(&mut self) -> Expr {
        let expr = self.comparison();

        while self.match_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            return Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let expr = self.term();

        while self.match_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            return Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let expr = self.factor();

        while self.match_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            return Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let expr = self.unary();

        while self.match_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();

            return Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();

            return Expr::Unary {
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_types(&[TokenType::False]) {
            return Expr::Literal(LiteralValue::Boolean(false));
        };

        if self.match_types(&[TokenType::True]) {
            return Expr::Literal(LiteralValue::Boolean(true));
        };

        if self.match_types(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal(LiteralValue::String(self.previous().literal.unwrap()));
        };

        if self.match_types(&[TokenType::LeftParen]) {
            let expr = self.expression();
            match self.consume(
                &TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            ) {
                Ok(t) => return Expr::Grouping(Box::new(expr)),
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        };

        panic!("Unknown token");
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(&self.peek(), message))
    }

    fn error(&self, token: &Token, message: String) -> ParseError {
        ParseError {}
    }

    fn match_types(&mut self, types: &[TokenType]) -> bool {
        types.iter().any(|t| {
            if self.check(t) {
                self.advance();
                true
            } else {
                false
            }
        })
    }

    fn check(&self, token_type: &TokenType) -> bool {
        match self.is_at_end() {
            true => false,
            false => self.peek().token_type == *token_type,
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }
}
