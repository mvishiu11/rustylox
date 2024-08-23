use std::fmt;

/// Represents a parsing error with a message and the line number where it occurred.
#[derive(Debug)]
pub struct ParserError {
    line: usize,
    message: String,
}

impl ParserError {
    /// Create a new ParserError.
    pub fn new(line: usize, message: String) -> Self {
        ParserError { line, message }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

impl std::error::Error for ParserError {}