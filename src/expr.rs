use std::rc::Rc;

use crate::{callable::LoxFunction, token::Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<Expr>),
    Literal(LiteralExpr),
    Unary(Box<UnaryExpr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(Box<LogicalExpr>),
    Call(Box<CallExpr>),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr,
}

#[derive(Debug, Clone)]
pub enum LiteralExpr {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Rc<LoxFunction>),
    Nil
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Expr,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}


impl Expr {
    pub fn pretty_print(&self) -> String {
        self.pretty_print_with_indent(0)
    }

    pub fn pretty_print_with_indent(&self, indent: usize) -> String {
        let indentation = " ".repeat(indent * 2);
        match self {
            Expr::Binary(expr) => format!(
                "{}BinaryExpression ({:?})\n{}├── {}\n{}└── {}",
                indentation,
                expr.operator.token_type,
                indentation,
                expr.left.pretty_print_with_indent(indent + 1),
                indentation,
                expr.right.pretty_print_with_indent(indent + 1)
            ),
            Expr::Grouping(expr) => format!(
                "{}Grouping\n{}└── {}",
                indentation,
                indentation,
                expr.pretty_print_with_indent(indent + 1)
            ),
            Expr::Literal(expr) => match expr {
                LiteralExpr::Number(n) => format!("{}Number ({})", indentation, n),
                LiteralExpr::String(s) => format!("{}String ({})", indentation, s),
                LiteralExpr::Boolean(b) => format!("{}Boolean ({})", indentation, b),
                LiteralExpr::Callable(func) => format!("{}Callable ({})", indentation, func.name),
                LiteralExpr::Nil => format!("{}Nil", indentation),
            },
            Expr::Unary(expr) => format!(
                "{}UnaryExpression ({:?})\n{}└── {}",
                indentation,
                expr.operator.token_type,
                indentation,
                expr.right.pretty_print_with_indent(indent + 1)
            ),
            Expr::Variable(token) => format!("{}Variable ({})", indentation, token.lexeme),
            Expr::Assign(token, expr) => format!(
                "{}Assign ({})\n{}└── {}",
                indentation,
                token.lexeme,
                indentation,
                expr.pretty_print_with_indent(indent + 1)
            ),
            Expr::Logical(expr) => format!(
                "{}LogicalExpression ({:?})\n{}├── {}\n{}└── {}",
                indentation,
                expr.operator.token_type,
                indentation,
                expr.left.pretty_print_with_indent(indent + 1),
                indentation,
                expr.right.pretty_print_with_indent(indent + 1)
            ),
            Expr::Call(expr) => {
                let mut pretty_arguments = String::new();
                for argument in &expr.arguments {
                    pretty_arguments.push_str(&argument.pretty_print_with_indent(indent + 1));
                }
                format!(
                    "{}CallExpression\n{}├── {}\n{}└── Arguments\n{}{}",
                    indentation,
                    indentation,
                    expr.callee.pretty_print_with_indent(indent + 1),
                    indentation,
                    pretty_arguments,
                    indentation
                )
            }
        }
    }
}