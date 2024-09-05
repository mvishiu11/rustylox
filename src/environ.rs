use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::expr::LiteralExpr;
use crate::error::EvalError;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, LiteralExpr>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(enclosing),
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
            if let Some(enclosing) = &self.enclosing {
                enclosing.borrow_mut().assign(name, value)
            } else {
                Err(EvalError::UndefinedVariable(name.lexeme.clone()))
            }
        }
    }

    pub fn get(&self, name: &Token) -> Result<LiteralExpr, EvalError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            if let Some(enclosing) = &self.enclosing {
                enclosing.borrow().get(name)
            } else {
                Err(EvalError::UndefinedVariable(name.lexeme.clone()))
            }
        }
    }

    pub fn get_at_depth(&self, name: &Token, depth: usize) -> Result<LiteralExpr, EvalError> {
        let mut environment = Rc::new(RefCell::new(self.clone()));
    
        for _ in 0..depth {
            let env = environment.clone();
            environment = match &env.borrow().enclosing {
                Some(enclosing) => enclosing.clone(),
                None => return Err(EvalError::UndefinedVariable(name.lexeme.clone())),
            };
        }
    
        let temp = environment.borrow().get(name); temp
    }    
}
