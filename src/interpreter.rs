use std::error::Error;
use crate::{error::EvalError, expr::{Expr, LiteralExpr}, token::TokenType};

impl Error for EvalError {}

// Define the `interpret` function
pub fn interpret(expr: &Expr) -> Result<f64, EvalError> {
    match expr {
        Expr::Literal(literal) => match literal {
            LiteralExpr::Number(n) => Ok(*n),
            LiteralExpr::String(_) => Err(EvalError::TypeError("Cannot interpret string as number".to_string())),
            LiteralExpr::Boolean(_) => Err(EvalError::TypeError("Cannot interpret boolean as number".to_string())),
            LiteralExpr::Nil => Err(EvalError::TypeError("Cannot interpret nil as number".to_string())),
        },
        Expr::Unary(unary) => {
            let right = interpret(&unary.right)?;
            match unary.operator.token_type {
                TokenType::Minus => Ok(-right),
                TokenType::Bang => Ok(if right == 0.0 { 1.0 } else { 0.0 }), // Logical NOT
                _ => Err(EvalError::SyntaxError("Unknown unary operator".to_string())),
            }
        },
        Expr::Binary(binary) => {
            let left = interpret(&binary.left)?;
            let right = interpret(&binary.right)?;
            match binary.operator.token_type {
                TokenType::Plus => Ok(left + right),
                TokenType::Minus => Ok(left - right),
                TokenType::Star => Ok(left * right),
                TokenType::Slash => {
                    if right == 0.0 {
                        Err(EvalError::DivisionByZero)
                    } else {
                        Ok(left / right)
                    }
                },
                TokenType::EqualEqual => Ok(if left == right { 1.0 } else { 0.0 }),
                TokenType::BangEqual => Ok(if left != right { 1.0 } else { 0.0 }),
                TokenType::Greater => Ok(if left > right { 1.0 } else { 0.0 }),
                TokenType::GreaterEqual => Ok(if left >= right { 1.0 } else { 0.0 }),
                TokenType::Less => Ok(if left < right { 1.0 } else { 0.0 }),
                TokenType::LessEqual => Ok(if left <= right { 1.0 } else { 0.0 }),
                _ => Err(EvalError::SyntaxError("Unknown binary operator".to_string())),
            }
        },
        Expr::Grouping(grouping) => interpret(&**grouping),
    }
}
