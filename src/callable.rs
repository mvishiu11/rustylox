use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::environ::Environment;
use crate::error::{ControlFlow, EvalError};
use crate::expr::{Expr, LiteralExpr};
use crate::interpreter::interpret_with_env;
use crate::resolver::Resolver;
use crate::stmt::Stmt;
use std::fmt::Debug;

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
    fn name(&self) -> &str;
    fn call(
        &self,
        arguments: Vec<LiteralExpr>, 
        environment: Rc<RefCell<Environment>>,
        resolver: &Resolver,
        output : &mut String
    ) -> Result<Expr, EvalError>;
}

impl Debug for dyn LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Callable({})", self.name())
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn call(
        &self, 
        arguments: Vec<LiteralExpr>, 
        _environment: Rc<RefCell<Environment>>,
        _resolver: &Resolver,
        output: &mut String
    ) -> Result<Expr, EvalError> {
        let mut function_env = Environment::new_enclosed(self.closure.clone());

        // Bind the arguments to the parameters
        for (param, arg) in self.params.iter().zip(arguments) {
            function_env.define(param.clone(), arg);
        }

        // Execute the function body and pass the output buffer
        let body_env = Rc::new(RefCell::new(function_env));
        match interpret_with_env(&self.body, Some(body_env), &_resolver, output) {
            Ok(_) => Ok(Expr::Literal(LiteralExpr::Nil)),
            Err(EvalError::ControlFlow(ControlFlow::Return(value))) => Ok(value),
            Err(e) => Err(e),
        }
    }
}


pub struct NativeFunction {
    name: String,
    arity: usize,
    function: fn(Vec<LiteralExpr>) -> Result<LiteralExpr, EvalError>,
}

impl NativeFunction {
    pub fn new(name: &str, arity: usize, function: fn(Vec<LiteralExpr>) -> Result<LiteralExpr, EvalError>) -> Self {
        NativeFunction {
            name: name.to_string(),
            arity,
            function,
        }
    }
}

impl LoxCallable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn call(
        &self, 
        arguments: Vec<LiteralExpr>, 
        _environment: Rc<RefCell<Environment>>,
        _resolver: &Resolver,
        _output : &mut String
    ) -> Result<Expr, EvalError> {
        let result = (self.function)(arguments)?;
        Ok(Expr::Literal(result))
    }
}