use std::fs;
use std::io::{self, BufRead, Write};

use crate::parser::Parser;
use crate::scanner::Scanner;

#[derive(Debug)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: u16, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: u16, location: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }

    pub fn run_file(&mut self, script: String) {
        let content = fs::read_to_string(script).unwrap();
        self.run(content);
        if self.had_error {
            panic!("Error in the file content.");
        }
    }

    pub fn run_prompt(&mut self) {
        print!("> ");
        io::stdout().flush().unwrap();

        for line in io::stdin().lock().lines() {
            match line {
                Ok(c) => {
                    self.run(c);
                    self.had_error = false;
                }
                _ => {
                    println!("Exiting...");
                    return;
                }
            };
        }
    }

    fn run(&mut self, source: String) {
        let mut s = Scanner::from(source, self);
        s.scan_tokens();

        let mut parser = Parser {
            tokens: s.tokens,
            current: 0,
        };

        let expr = parser.parse();

        println!("{:?}", expr);
    }
}
