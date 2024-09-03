use std::error::Error;
use std::fmt::Write;
use crate::{error::EvalError, expr::{Expr, LiteralExpr}, stmt::Stmt, token::TokenType};
use crate::environ::Environment;

impl Error for EvalError {}

pub fn interpret(statements: &[Stmt]) -> Result<String, EvalError> {
    let mut environment = Environment::new();
    let mut output = String::new();
    for statement in statements {
        execute(statement, &mut environment, &mut output)?;
    }
    Ok(output)
}

fn execute(stmt: &Stmt, environment: &mut Environment, output: &mut String) -> Result<(), EvalError> {
    match stmt {
        Stmt::Expression(expr) => {
            evaluate(expr, environment)?;
        }
        Stmt::Print(expr) => {
            let value = evaluate(expr, environment)?;
            match value {
                Expr::Literal(literal) => {
                    match literal {
                        LiteralExpr::Number(n) => writeln!(output, "{}", n).unwrap(),
                        LiteralExpr::String(s) => writeln!(output, "{}", s).unwrap(),
                        LiteralExpr::Boolean(b) => writeln!(output, "{}", b).unwrap(),
                        LiteralExpr::Nil => writeln!(output, "nil").unwrap(),
                    }
                },
                _ => return Err(EvalError::TypeError("Invalid expression type in print statement".to_string())),
            }
        }
        Stmt::Block(statements) => {
            let mut block_environment = Environment::new_enclosed(environment.clone());
            for statement in statements {
                execute(statement, &mut block_environment, output)?;
            }
        }
        Stmt::Var(name, initializer) => {
            let value = if let Some(expr) = initializer {
                evaluate(expr, environment)?
            } else {
                Expr::Literal(LiteralExpr::Nil)
            };

            // Define the variable in the environment
            if let Expr::Literal(literal_value) = value {
                environment.define(name.clone(), literal_value);
            }
        }
        _ => return Err(EvalError::SyntaxError("Unknown statement type".to_string())),
    }
    Ok(())
}

/// Evaluate an expression and return the result.
pub fn evaluate(expr: &Expr, environment: &mut Environment) -> Result<Expr, EvalError> {
    match expr {
        Expr::Literal(literal) => Ok(Expr::Literal(literal.clone())),
        Expr::Unary(unary) => {
            let right = evaluate(&unary.right, environment)?;
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
            let left = evaluate(&binary.left, environment)?;
            let right = evaluate(&binary.right, environment)?;
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
        Expr::Grouping(grouping) => evaluate(&**grouping, environment),
        Expr::Variable(name) => {
            let value = environment.get(&name);
            match value {
                Ok(literal) => Ok(Expr::Literal(literal)),
                Err(e) => Err(e),
            }
        },
        Expr::Assign(name, expr) => {
            let value = evaluate(&expr, environment)?;
            if let Expr::Literal(ref literal) = value {
                environment.assign(&name.clone(), literal.clone())?;
            }
            Ok(value)
        },
    }
}
