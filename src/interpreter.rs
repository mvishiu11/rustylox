use std::cell::RefCell;
use std::{error::Error, rc::Rc};
use std::fmt::Write;
use crate::callable::LoxFunction;
use crate::resolver::Resolver;
use crate::{error::EvalError, expr::{Expr, LiteralExpr}, stmt::Stmt, token::TokenType};
use crate::error::ControlFlow;
use crate::environ::Environment;
use crate::natives::define_native_functions;

impl Error for EvalError {}

pub fn interpret(statements: &[Stmt], resolver: &Resolver) -> Result<String, EvalError> {
    let globals = Rc::new(RefCell::new(Environment::new()));
    define_native_functions(&mut globals.borrow_mut());
    let mut output = String::new();
    interpret_with_env(statements, Some(globals), resolver, &mut output)
}

pub fn interpret_with_env(
    statements: &[Stmt], 
    environ: Option<Rc<RefCell<Environment>>>, 
    resolver: &Resolver,
    output: &mut String
) -> Result<String, EvalError> {
    let environment = environ.unwrap_or_else(|| Rc::new(RefCell::new(Environment::new())));

    for statement in statements {
        execute(statement, environment.clone(), resolver, output)?;
    }

    Ok(output.clone())
}

fn execute(stmt: &Stmt, environment: Rc<RefCell<Environment>>, resolver: &Resolver, output: &mut String) -> Result<(), EvalError> {
    match stmt {
        Stmt::While(condition, body) => {
            while {
                let condition_value = evaluate(condition, environment.clone(), resolver, output)?;
                if let Expr::Literal(LiteralExpr::Boolean(b)) = condition_value {
                    b
                } else {
                    return Err(EvalError::TypeError("While condition must be a boolean".to_string()));
                }
            } {
                match execute(&*body, environment.clone(), resolver, output) {
                    Ok(()) => (),
                    Err(EvalError::ControlFlow(ControlFlow::Break)) => break,
                    Err(EvalError::ControlFlow(ControlFlow::Continue)) => continue,
                    Err(e) => return Err(e),
                }
            }
        }
        Stmt::Block(statements) => {
            let new_env = Rc::new(RefCell::new(Environment::new_enclosed(environment.clone())));
            for statement in statements {
                match execute(statement, new_env.clone(), resolver, output) {
                    Ok(()) => (),
                    Err(EvalError::ControlFlow(ControlFlow::Break)) => return Err(EvalError::ControlFlow(ControlFlow::Break)),
                    Err(EvalError::ControlFlow(ControlFlow::Continue)) => return Err(EvalError::ControlFlow(ControlFlow::Continue)),
                    Err(e) => return Err(e),
                }
            }
        }
        Stmt::Break => return Err(EvalError::ControlFlow(ControlFlow::Break)),
        Stmt::Continue => return Err(EvalError::ControlFlow(ControlFlow::Continue)),
        Stmt::Expression(expr) => {
            evaluate(expr, environment, resolver, output)?;
        }
        Stmt::If(condition, then_branch, else_branch) => {
            let condition_value = evaluate(condition, environment.clone(), resolver, output)?;
        
            if let Expr::Literal(LiteralExpr::Boolean(b)) = condition_value {
                if b {
                    execute(&*then_branch, environment.clone(), resolver, output)?;
                } else if let Some(else_branch) = else_branch {
                    execute(&*else_branch, environment.clone(), resolver, output)?;
                }
            } else {
                return Err(EvalError::TypeError("If condition must be a boolean".to_string()));
            }
        }
        Stmt::Function(name, params, body) => {
            let function = LoxFunction::new(name.clone(), params.clone(), body.clone(), environment.clone());
            environment.borrow_mut().define(name.clone(), LiteralExpr::Callable(Rc::new(function)));
        }
        Stmt::Return(Some(expr)) => {
            let value = evaluate(expr, environment.clone(), resolver, output)?;
            return Err(EvalError::ControlFlow(ControlFlow::Return(value)));
        },
        Stmt::Return(None) => {
            return Err(EvalError::ControlFlow(ControlFlow::Return(Expr::Literal(LiteralExpr::Nil))));
        },    
        Stmt::Print(expr) => {
            let value = evaluate(expr, environment, resolver, output)?;
            match value {
                Expr::Literal(literal) => {
                    match literal {
                        LiteralExpr::Number(n) => writeln!(output, "{}", n).unwrap(),
                        LiteralExpr::String(s) => writeln!(output, "{}", s).unwrap(),
                        LiteralExpr::Boolean(b) => writeln!(output, "{}", b).unwrap(),
                        LiteralExpr::Callable(callable) => writeln!(output, "{:?}", callable).unwrap(),
                        LiteralExpr::Nil => writeln!(output, "nil").unwrap(),
                    }
                },
                _ => return Err(EvalError::TypeError("Invalid expression type in print statement".to_string())),
            }
        }
        Stmt::Var(name, initializer) => {
            let value = if let Some(expr) = initializer {
                evaluate(expr, environment.clone(), resolver, output)?
            } else {
                Expr::Literal(LiteralExpr::Nil)
            };

            if let Expr::Literal(literal_value) = value {
                environment.borrow_mut().define(name.clone(), literal_value);
            }
        }    
    }
    Ok(())
}

/// Main evaluation function for expressions
pub fn evaluate(expr: &Expr, environment: Rc<RefCell<Environment>>, resolver: &Resolver, output: &mut String) -> Result<Expr, EvalError> {
    match expr {
        Expr::Literal(literal) => Ok(Expr::Literal(literal.clone())),
        Expr::Unary(unary) => {
            let right = evaluate(&unary.right, environment.clone(), resolver, output)?;
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
            let left = evaluate(&binary.left, environment.clone(), resolver, output)?;
            let right = evaluate(&binary.right, environment.clone(), resolver, output)?;
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
                    TokenType::Percent => if r == 0.0 {
                        Err(EvalError::DivisionByZero)
                    } else {
                        Ok(Expr::Literal(LiteralExpr::Number(l % r)))
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
        Expr::Grouping(grouping) => evaluate(&**grouping, environment.clone(), resolver, output),
        Expr::Variable(name) => {
            if let Some(scope_depth) = resolver.resolve_local(&name.lexeme) {
                // If we have a scope depth, fetch from the local environment
                match environment.borrow().get_at_depth(&name, scope_depth) {
                    Ok(literal) => Ok(Expr::Literal(literal)),
                    Err(_) => Err(EvalError::UndefinedVariable(name.lexeme.clone())),
                }
            } else {
                // Otherwise, fetch from the global environment
                match environment.borrow().get(&name) {
                    Ok(literal) => Ok(Expr::Literal(literal)),
                    Err(_) => Err(EvalError::UndefinedVariable(name.lexeme.clone())),
                }
            }
        }
        Expr::Assign(name, expr) => {
            let value = evaluate(&expr, environment.clone(), resolver, output)?;
            if let Expr::Literal(ref literal) = value {
                environment.borrow_mut().assign(name, literal.clone())?;
            }
            Ok(value)
        },
        Expr::Logical(logical) => {
            let left = evaluate(&logical.left, environment.clone(), resolver, output)?;
            if logical.operator.token_type == TokenType::Or {
                if is_truthy(&left) {
                    return Ok(left);
                }
            } else {
                if !is_truthy(&left) {
                    return Ok(left);
                }
            }
            evaluate(&logical.right, environment.clone(), resolver, output)
        },
        Expr::Call(call_expr) => {
            let callee = evaluate(&call_expr.callee, environment.clone(), resolver, output)?;
            let mut arguments = Vec::new();
        
            for arg in &call_expr.arguments {
                let value = match evaluate(arg, environment.clone(), resolver, output)? {
                    Expr::Literal(literal) => literal,
                    _ => return Err(EvalError::TypeError("Invalid argument type".to_string())),
                };
                arguments.push(value);
            }
        
            match callee {
                Expr::Literal(LiteralExpr::Callable(callable)) => {
                    if arguments.len() != callable.arity() {
                        return Err(EvalError::ArityError(callable.arity(), arguments.len()));
                    }
                    callable.call(arguments, environment.clone(), resolver, output)
                },
                _ => Err(EvalError::TypeError("Can only call functions and classes".to_string())),
            }
        }        
    }
}

fn is_truthy(expr: &Expr) -> bool {
    match expr {
        Expr::Literal(LiteralExpr::Nil) => false,
        Expr::Literal(LiteralExpr::Boolean(b)) => *b,
        _ => true,
    }
}