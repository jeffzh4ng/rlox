use crate::{interpreter::RuntimeError, token::{Literal, Token}};

use std::collections::HashMap;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Literal>
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }
    
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else {
            match &self.enclosing {
                Some(e) => {
                    e.get(name)
                },
                None => {
                    Err(RuntimeError(name.clone(), format!("Undefined variable {}", name.lexeme)))
                }
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else {
            match &mut self.enclosing {
                Some(e) => {
                    e.assign(name, value)
                },
                None => {
                    Err(RuntimeError(name.clone(), format!("Undefined variable {}", name.lexeme)))
                }
            }
        }
    }
}