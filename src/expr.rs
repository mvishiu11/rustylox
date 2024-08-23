use crate::token::Token;

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
        self.pretty_print_with_indent(0)
    }

    fn pretty_print_with_indent(&self, indent: usize) -> String {
        let indentation = " ".repeat(indent * 2);
        match self {
            Expr::Binary(expr) => format!(
                "{}BinaryExpression ({:?})\n{}├── {}\n{}└── {}",
                indentation,
                expr.operator.token_type, // Display the token type for the operator
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
                LiteralExpr::Nil => format!("{}Nil", indentation),
            },
            Expr::Unary(expr) => format!(
                "{}UnaryExpression ({:?})\n{}└── {}",
                indentation,
                expr.operator.token_type, // Display the token type for the operator
                indentation,
                expr.right.pretty_print_with_indent(indent + 1)
            ),
        }
    }
}