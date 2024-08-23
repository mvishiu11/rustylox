use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<Expr>),
    Literal(LiteralExpr),
    Unary(Box<UnaryExpr>),
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
    Nil
}

impl Expr {
    pub fn pretty_print(&self) -> String {
        match self {
            Expr::Binary(expr) => format!(
                "({} {} {})",
                expr.operator.lexeme,
                expr.left.pretty_print(),
                expr.right.pretty_print()
            ),
            Expr::Grouping(expr) => format!("(group {})", expr.pretty_print()),
            Expr::Literal(expr) => match expr {
                LiteralExpr::Number(n) => n.to_string(),
                LiteralExpr::String(s) => s.clone(),
                LiteralExpr::Boolean(b) => b.to_string(),
                LiteralExpr::Nil => "nil".to_string(),
            },
            Expr::Unary(expr) => format!(
                "({} {})",
                expr.operator.lexeme,
                expr.right.pretty_print()
            ),
        }
    }
}