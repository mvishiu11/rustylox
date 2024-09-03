use std::collections::HashMap;
use crate::expr::LiteralExpr;
use crate::error::EvalError;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, LiteralExpr>,
}

#[allow(dead_code)]
impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosed(enclosing: Environment) -> Self {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralExpr) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: LiteralExpr) -> Result<(), EvalError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            if let Some(enclosing) = &mut self.enclosing {
                enclosing.assign(name, value)
            } else {
                Err(EvalError::UndefinedVariable(name.lexeme.clone()))
            }
        }
    }

    // Get the value of a variable or throw Runtime Error if not exists
    pub fn get(&self, name: &Token) -> Result<LiteralExpr, EvalError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            if let Some(enclosing) = &self.enclosing {
                enclosing.get(name)
            } else {
                Err(EvalError::UndefinedVariable(name.lexeme.clone()))
            }
        }
    }
}
