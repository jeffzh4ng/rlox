mod token;
mod scanner;
mod parser;
mod interpreter;

use std::{env};
use std::process;
use std::io;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};

use interpreter::{Interpreter, RuntimeError};
use token::{Literal, Token, TokenType};
use scanner::Scanner;
use parser::{ParseError, Parser};

use lazy_static::lazy_static;


fn main() {
    let mut lox = Lox{
        had_error: false
    };
    lox.main();
}

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

lazy_static! {
    static ref INTERPRETER: Interpreter = Interpreter::new();
}


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

        if HAD_ERROR.load(Ordering::Relaxed) == true {
            process::exit(64);
        }
        if HAD_RUNTIME_ERROR.load(Ordering::Relaxed) == true {
            process::exit(70);
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
        let stmts = parser.parse();

        if self.had_error {
            return;
        }

        INTERPRETER.interpret(stmts);
    }

    fn error(line: u32, message: String) {
        Lox::report(line, "".to_owned(), message);
    }

    fn parse_error(error: ParseError) {
        let ParseError(token, message) = error;
        
        if token.token_type == TokenType::Eof {
            Lox::report(token.line, "at end".to_owned(), message)
        } else {
            Lox::report(token.line, format!("at, {}", token.lexeme), message)
        }
    }

    fn runtime_error(error: RuntimeError) {
        let RuntimeError(token, message) = error;

        println!("{} \n[line {}]", message, token.line);
        HAD_RUNTIME_ERROR.store(true, Ordering::Relaxed);
    }

    fn report(line: u32, where_: String, message: String) {
        println!("[line {}] Error {}: {}", line, where_, message);
        HAD_ERROR.store(true, Ordering::Relaxed);
    }
}
