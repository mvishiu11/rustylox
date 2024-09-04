use std::fmt;

/// Represents a parsing error with a message and the line number where it occurred.
#[derive(Debug, Clone)]
pub struct ParserError {
    line: usize,
    message: String,
}

/// Represents a runtime error with a message and the line number where it occurred.
#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum ControlFlow {
    Break,
    Continue,
}

impl ParserError {
    /// Create a new ParserError.
    pub fn new(line: usize, message: String) -> Self {
        ParserError { line, message }
    }
}

// Define the evaluator errors
#[derive(Debug)]
#[allow(dead_code)]
pub enum EvalError {
    DivisionByZero,
    UndefinedVariable(String),
    TypeError(String),
    SyntaxError(String),
    ControlFlow(ControlFlow),
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