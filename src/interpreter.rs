use crate::{Lox, environment::Environment, parser::{Expr, Stmt}, token::{Literal, Token, TokenType}};
use std::error;

pub struct Interpreter {
    environment: Box<Environment>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Box::new(Environment::new(None))
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Box<Stmt>>) {
        for s in stmts {
            self.interpret_stmt(s);
        }
    }

    pub fn interpret_stmt(&mut self, stmt: Box<Stmt>) -> Option<Literal> { // function has to be method due to weird lazy static error
        match *stmt {
            Stmt::Print(e) => {
                let f = self.evaluate(e);

                match f {
                    Ok(l) => { 
                        println!("{:?}", l);
                        None
                    },
                    Err(e) => {
                        Lox::runtime_error(e);
                        None
                    }
                }
            },
            Stmt::Var(name, initializer) => {
                let mut value = Literal::Nil;

                match *initializer {
                    Some(e) => {
                        let f = self.evaluate(Box::new(e));

                        match f {
                            Ok(l) => { 
                                value = l;
                            },
                            Err(e) => {
                                Lox::runtime_error(e);
                            }
                        };
                    },
                    None => {}
                }

                self.environment.define(name.lexeme, value);
                None
            },
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.interpret_stmt(Box::new(s));
                }
                
                None
            },
            Stmt::If(condition, then_branch, else_branch) => {
                if Interpreter::is_truthy(self.evaluate(condition).unwrap()) {
                    self.interpret_stmt(then_branch);
                } else {
                    self.interpret_stmt(else_branch.unwrap());
                }

                None
            },
            Stmt::Expr(e) => {
                let f = self.evaluate(e);
        
                match f {
                    Ok(l) => Some(l),
                    Err(e) => {
                        Lox::runtime_error(e);
                        None
                    }
                }
            },
        }
    }

    fn evaluate(&mut self, expr: Box<Expr>) -> Result<Literal, RuntimeError> {
        match *expr {
            Expr::Unary(t, e) => {
                self.evaluate_unary(t, e)
            },
            Expr::Binary(l, t, r) => {
                self.evaluate_binary(l, t, r)
            },
            Expr::Grouping(g) => {
                self.evaluate_grouping(g)
            },
            Expr::Literal(l) => {
                self.evaluate_literal(l)
            },
            Expr::Var(t) => {
                self.environment.get(t)
            },
            Expr::Assignment(t, expr) => {
                let value = self.evaluate(expr)?;
                self.environment.assign(t, value.clone())?;
                Ok(value)
            }
        }
    }

    fn evaluate_unary(&mut self, t: Token, r: Box<Expr>) -> Result<Literal, RuntimeError> {
        let r = self.evaluate(r)?;
    
        match t.token_type {
            TokenType::Bang => {
                return Ok(Literal::Bool(Interpreter::is_truthy(r)));
            },
            TokenType::Minus => {
                match r {
                    Literal::Number(r) => {
                        return Ok(Literal::Number(r * -1 as f64));
                    },
                    _ => Err(RuntimeError(t, "Operand must be a number".to_owned()))
                }
            },
            _ => Ok(Literal::Nil) // unreachable
        }
    }

    fn evaluate_binary(&mut self, l: Box<Expr>, t: Token, r: Box<Expr>) -> Result<Literal, RuntimeError> {
        let l = self.evaluate(l)?;
        let r = self.evaluate(r)?;

        match t.token_type {
            TokenType::Plus => {
                match (l, r) {
                    (Literal::Number(l), Literal::Number(r)) => {
                        return Ok(Literal::Number(l + r));
                    },
                    (Literal::String(l), Literal::String(r)) => {
                        return Ok(Literal::String(l + &r));
                    },
                    _ => Err(RuntimeError(t, "Operands must be numbers".to_owned()))
                }
            },
            TokenType::Minus => {
                match (l, r) {
                    (Literal::Number(l), Literal::Number(r)) => return Ok(Literal::Number(l-r)),
                    _ => Err(RuntimeError(t, "Operands must be numbers".to_owned()))
                }
            },
            TokenType::Star =>  {
                match (l, r) {
                    (Literal::Number(l), Literal::Number(r)) => return Ok(Literal::Number(l*r)),
                    _ => Err(RuntimeError(t, "Operands must be numbers".to_owned()))
                }
            },
            TokenType::Slash => {
                match (l, r) {
                    (Literal::Number(l), Literal::Number(r)) => return Ok(Literal::Number(l/r)),
                    _ => Err(RuntimeError(t, "Operands must be numbers".to_owned()))
                }
            },
            TokenType::Greater => {
                match (l, r) {
                    (Literal::Number(l), Literal::Number(r)) => return Ok(Literal::Bool(l > r)),
                    _ => Err(RuntimeError(t, "Operands must be numbers".to_owned()))
                }
            },
            TokenType::GreaterEqual => {
                match (l, r) {
                    (Literal::Number(l), Literal::Number(r)) => return Ok(Literal::Bool(l >= r)),
                    _ => Err(RuntimeError(t, "Operands must be numbers".to_owned()))
                }
            },
            TokenType::Less => {
                match (l, r) {
                    (Literal::Number(l), Literal::Number(r)) => return Ok(Literal::Bool(l < r)),
                    _ => Err(RuntimeError(t, "Operands must be numbers".to_owned()))
                }
            },
            TokenType::LessEqual => {
                match (l, r) {
                    (Literal::Number(l), Literal::Number(r)) => return Ok(Literal::Bool(l <= r)),
                    _ => Err(RuntimeError(t, "Operands must be numbers".to_owned()))
                }
            },
            TokenType::EqualEqual => return Ok(Literal::Bool(Interpreter::is_equal(l, r))),
            TokenType::BangEqual => return Ok(Literal::Bool(!Interpreter::is_equal(l, r))),
            _ => {
                Ok(Literal::Nil) // unreachable
            }
        }
    }

    fn evaluate_grouping(&mut self, g: Box<Expr>) -> Result<Literal, RuntimeError> {
        self.evaluate(g)
    }

    fn evaluate_literal(&self, l: Literal) -> Result<Literal, RuntimeError> {
        Ok(l)
    }

    // ======== HELPERS ========
    fn is_truthy(l: Literal) -> bool {
        match l {
            Literal::Nil => false,
            Literal::Bool(b) => b,
            _ => true
        }
    }

    fn is_equal(l: Literal, r: Literal) -> bool {
        match (l, r) {
            (Literal::Nil, Literal::Nil) => return true,
            (Literal::Nil, _) => return false,
            (l, r) => return l == r,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError(pub Token, pub String);

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0.to_string(), self.1)
    }
}

impl error::Error for RuntimeError {}