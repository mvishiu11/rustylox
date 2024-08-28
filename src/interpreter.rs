use std::error::Error;
use crate::{error::EvalError, expr::{Expr, LiteralExpr}, token::TokenType};

impl Error for EvalError {}

// Function to print or handle the result of an expression
pub fn interpret(expr: &Expr) -> Result<(), EvalError> {
    match evaluate(expr)? {
        Expr::Literal(literal) => {
            match literal {
                LiteralExpr::Number(n) => println!("{}", n),
                LiteralExpr::String(s) => println!("{}", s),
                LiteralExpr::Boolean(b) => println!("{}", b),
                LiteralExpr::Nil => println!("Nil"),
            }
            Ok(())
        },
        // Handle other types of expressions if needed
        _ => Err(EvalError::TypeError("Unhandled expression type".to_string())),
    }
}

// Define the `evaluate` function
pub fn evaluate(expr: &Expr) -> Result<Expr, EvalError> {
    match expr {
        Expr::Literal(literal) => Ok(Expr::Literal(literal.clone())), // Return the literal as-is
        Expr::Unary(unary) => {
            let right = evaluate(&unary.right)?;
            match right {
                Expr::Literal(LiteralExpr::Number(n)) => match unary.operator.token_type {
                    TokenType::Minus => Ok(Expr::Literal(LiteralExpr::Number(-n))),
                    TokenType::Bang => Ok(Expr::Literal(LiteralExpr::Boolean(n == 0.0))),
                    _ => Err(EvalError::SyntaxError("Unknown unary operator".to_string())),
                },
                Expr::Literal(LiteralExpr::Boolean(b)) => match unary.operator.token_type {
                    TokenType::Bang => Ok(Expr::Literal(LiteralExpr::Boolean(!b))),
                    _ => Err(EvalError::SyntaxError("Unknown unary operator".to_string())),
                },
                _ => Err(EvalError::TypeError("Cannot apply unary operator to non-numeric or non-boolean type".to_string())),
            }
        },
        Expr::Binary(binary) => {
            let left = evaluate(&binary.left)?;
            let right = evaluate(&binary.right)?;
            match (left, right) {
                (Expr::Literal(LiteralExpr::Number(l)), Expr::Literal(LiteralExpr::Number(r))) => match binary.operator.token_type {
                    TokenType::Plus => Ok(Expr::Literal(LiteralExpr::Number(l + r))),
                    TokenType::Minus => Ok(Expr::Literal(LiteralExpr::Number(l - r))),
                    TokenType::Star => Ok(Expr::Literal(LiteralExpr::Number(l * r))),
                    TokenType::Slash => if r == 0.0 {
                        Err(EvalError::DivisionByZero)
                    } else {
                        Ok(Expr::Literal(LiteralExpr::Number(l / r)))
                    },
                    TokenType::EqualEqual => Ok(Expr::Literal(LiteralExpr::Boolean(l == r))),
                    TokenType::BangEqual => Ok(Expr::Literal(LiteralExpr::Boolean(l != r))),
                    TokenType::Greater => Ok(Expr::Literal(LiteralExpr::Boolean(l > r))),
                    TokenType::GreaterEqual => Ok(Expr::Literal(LiteralExpr::Boolean(l >= r))),
                    TokenType::Less => Ok(Expr::Literal(LiteralExpr::Boolean(l < r))),
                    TokenType::LessEqual => Ok(Expr::Literal(LiteralExpr::Boolean(l <= r))),
                    _ => Err(EvalError::SyntaxError("Unknown binary operator".to_string())),
                },
                (Expr::Literal(LiteralExpr::String(l)), Expr::Literal(LiteralExpr::String(r))) => match binary.operator.token_type {
                    TokenType::Plus => Ok(Expr::Literal(LiteralExpr::String(l + &r))),
                    _ => Err(EvalError::TypeError("Unsupported operation for strings".to_string())),
                },
                (Expr::Literal(LiteralExpr::Number(l)), Expr::Literal(LiteralExpr::String(r))) => match binary.operator.token_type {
                    TokenType::Plus => Ok(Expr::Literal(LiteralExpr::String(format!("{}{}", l, r)))),
                    _ => Err(EvalError::TypeError("Unsupported operation for mixed types".to_string())),
                },
                (Expr::Literal(LiteralExpr::String(l)), Expr::Literal(LiteralExpr::Number(r))) => match binary.operator.token_type {
                    TokenType::Plus => Ok(Expr::Literal(LiteralExpr::String(format!("{}{}", l, r)))),
                    _ => Err(EvalError::TypeError("Unsupported operation for mixed types".to_string())),
                },
                _ => Err(EvalError::TypeError("Operands must be compatible for the operation".to_string())),
            }
        },
        Expr::Grouping(grouping) => evaluate(&**grouping),
    }
}
