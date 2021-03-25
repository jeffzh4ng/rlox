use crate::{Lox, parser::Expr, token::{Literal, Token, TokenType}};
use std::error;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter{}
    }

    pub fn interpret(&self, expr: Box<Expr>) -> Option<Literal> { // function has to be method due to weird lazy static error
        let f = Interpreter::evaluate(expr);

        match f {
            Ok(l) => Some(l),
            Err(e) => {
                Lox::runtime_error(e);
                None
            }
        }
    }

    fn evaluate(expr: Box<Expr>) -> Result<Literal, RuntimeError> {
        match *expr {
            Expr::Unary(t, e) => {
                Interpreter::evaluate_unary(t, e)
            },
            Expr::Binary(l, t, r) => {
                Interpreter::evaluate_binary(l, t, r)
            },
            Expr::Grouping(g) => {
                Interpreter::evaluate_grouping(g)
            },
            Expr::Literal(l) => {
                Interpreter::evaluate_literal(l)
            },
        }
    }

    fn evaluate_unary(t: Token, r: Box<Expr>) -> Result<Literal, RuntimeError> {
        let r = Interpreter::evaluate(r)?;
    
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

    fn evaluate_binary(l: Box<Expr>, t: Token, r: Box<Expr>) -> Result<Literal, RuntimeError> {
        let l = Interpreter::evaluate(l)?;
        let r = Interpreter::evaluate(r)?;

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

    fn evaluate_grouping(g: Box<Expr>) -> Result<Literal, RuntimeError> {
        Interpreter::evaluate(g)
    }

    fn evaluate_literal(l: Literal) -> Result<Literal, RuntimeError> {
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