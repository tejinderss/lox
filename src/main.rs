use std::env;

mod expr;
mod lox;
mod parser;
mod scanner;
mod token;
mod token_type;

use expr::{Expr, LiteralValue};
use lox::Lox;
use token::Token;
use token_type::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();

    if args.len() > 2 {
        panic!("Usage: lox [script]");
    } else if args.len() == 2 {
        lox.run_file(args[1].clone());
    } else {
        lox.run_prompt();
    }
}

fn main2() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(LiteralValue::Number(123.0))),
        }),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping(Box::new(Expr::Literal(
            LiteralValue::Number(45.67),
        )))),
    };

    println!("{}", expr.print());
}
