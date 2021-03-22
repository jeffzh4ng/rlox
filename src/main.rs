use std::env;
use std::process;
use std::io;
use std::fs;

fn main() {
    let mut lox = Lox{
        had_error: false
    };
    lox.main();
}

struct Lox {
    had_error: bool
}

impl Lox {
    fn main(&mut self) {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            println!("Usage: rlox [script]");
            process::exit(64);
        } else if args.len() == 1 {
            self.run_file(&args[0]).unwrap();
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&self, path: &str) -> io::Result<()> {
        let bytes = fs::read(path).unwrap();
        let string = std::str::from_utf8(&bytes).unwrap().to_owned();
        run(string);

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
            run(line);

            self.had_error = false; // reset after every loop. if a user makes a mistake, it shouldn't kill their entire session.
        }
    }

    fn error(&mut self, line: u32, message: String) {
        self.report(line, "".to_owned(), message);
    }

    fn report(&mut self, line: u32, error: String, message: String) {
        println!("[line {}] Error {}: {}", line, error, message);
        self.had_error = true;
    }
}

struct Scanner {
    source: String
}

impl Scanner {
    fn scan_tokens(&self) -> Vec<Token> {
        todo!();
    }
}
#[derive(Debug)]
struct Token {}

fn run(source: String) {
    let scanner = Scanner {
        source
    };
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token)
    }
}