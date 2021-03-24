mod token;
mod scanner;
mod parser;

use std::{env};
use std::process;
use std::io;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};

use token::{Token, TokenType};
use scanner::Scanner;
use parser::Parser;

fn main() {
    let mut lox = Lox{
        had_error: false
    };
    lox.main();
}

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

struct Lox {
    had_error: bool
}

impl Lox {
    fn main(&mut self) {
        let args: Vec<String> = env::args().collect();

        self.run_prompt();

        // if args.len() > 1 {
        //     println!("Usage: rlox [script]");
        //     process::exit(64);
        // } else if args.len() == 1 {
        //     self.run_file(&args[0]).unwrap();
        // } else {
        //     self.run_prompt();
        // }
    }

    fn run_file(&self, path: &str) -> io::Result<()> {
        let bytes = fs::read(path).unwrap();
        let string = std::str::from_utf8(&bytes).unwrap().to_owned();
        self.run(string);

        if self.had_error {
            process::exit(64);
        }
        Ok(())
    }

    fn run_prompt(&mut self) {
        loop {
            println!("> ");
            
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            if line.len() == 0 {
                break;
            }
            self.run(line);

            self.had_error = false; // reset after every loop. if a user makes a mistake, it shouldn't kill their entire session.
        }
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        if self.had_error || expr.is_none() {
            return;
        } else {
            println!("{:?}", expr);
        }
    }

    fn error(line: u32, message: String) {
        Lox::report(line, "".to_owned(), message);
    }

    fn token_error(token: Token, message: String) {
        if token.token_type == TokenType::Eof {
            Lox::report(token.line, " at end".to_owned(), message)
        } else {
            Lox::report(token.line, format!("at, {}", token.lexeme), message)
        }
    }

    fn report(line: u32, error: String, message: String) {
        println!("[line {}] Error {}: {}", line, error, message);
        HAD_ERROR.store(true, Ordering::Relaxed);
    }
}
