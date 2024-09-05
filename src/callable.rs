use std::cell::RefCell;
use std::rc::Rc;

use crate::environ::Environment;
use crate::error::{ControlFlow, EvalError};
use crate::expr::{Expr, LiteralExpr};
use crate::interpreter::interpret_with_env;
use crate::stmt::Stmt;

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(name: String, params: Vec<String>, body: Vec<Stmt>, closure: Rc<RefCell<Environment>>) -> Self {
        LoxFunction { name, params, body, closure }
    }
}

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        arguments: Vec<LiteralExpr>, 
        environment: Rc<RefCell<Environment>>
    ) -> Result<Expr, EvalError>;
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self, 
        arguments: Vec<LiteralExpr>, 
        _environment: Rc<RefCell<Environment>>
    ) -> Result<Expr, EvalError> {
        let mut new_env = Environment::new_enclosed(self.closure.clone());

        // Bind the arguments to the parameters
        for (param, arg) in self.params.iter().zip(arguments) {
            new_env.define(param.clone(), arg);
        }

        // Execute the function body
        let body_env = Rc::new(RefCell::new(new_env));
        match interpret_with_env(&self.body, Some(body_env)) {
            Ok(_) => Ok(Expr::Literal(LiteralExpr::Nil)),  // If no explicit return, return nil
            Err(EvalError::ControlFlow(ControlFlow::Return(value))) => Ok(value),  // Handle return
            Err(e) => Err(e),
        }
    }
}
