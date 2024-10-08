use std::fmt;

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum ControlFlow {
    Break,
    Continue,
    Return(Expr),
}

impl ParserError {
    pub fn new(line: usize, message: String) -> Self {
        ParserError { line, message }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum EvalError {
    DivisionByZero,
    UndefinedVariable(String),
    TypeError(String),
    SyntaxError(String),
    ControlFlow(ControlFlow),
    ArityError(usize, usize),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

impl std::error::Error for RuntimeError {}

impl std::error::Error for ParserError {}