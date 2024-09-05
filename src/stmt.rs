use crate::expr::Expr;
use crate::error::ParserError;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(String, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Break,
    Continue,
    Function(String, Vec<String>, Vec<Stmt>),
    Return(Option<Expr>),
}

pub fn pretty_print_program(program: (Vec<Stmt>, Vec<ParserError>)) -> String {
    let (statements, errors) = program;

    let mut result = String::new();

    // Print each statement
    for stmt in statements {
        result.push_str(&stmt.pretty_print());
        result.push('\n');
    }

    // Print each error
    if !errors.is_empty() {
        result.push_str("\nErrors:\n");
        for error in errors {
            result.push_str(&format!(
                "Line {}: {}\n",
                error.line, error.message
            ));
        }
    }

    result
}


impl Stmt {
    pub fn pretty_print(&self) -> String {
        self.pretty_print_with_indent(0)
    }

    pub fn pretty_print_with_indent(&self, indent: usize) -> String {
        let indentation = " ".repeat(indent * 2);
        match self {
            Stmt::Expression(expr) => format!(
                "{}Expression\n{}└── {}",
                indentation,
                indentation,
                expr.pretty_print_with_indent(indent + 1)
            ),
            Stmt::Print(expr) => format!(
                "{}Print\n{}└── {}",
                indentation,
                indentation,
                expr.pretty_print_with_indent(indent + 1)
            ),
            Stmt::Var(name, initializer) => {
                let initializer_str = if let Some(expr) = initializer {
                    expr.pretty_print_with_indent(indent + 1)
                } else {
                    format!("{}None", indentation)
                };
                format!(
                    "{}Var ({})\n{}└── {}",
                    indentation,
                    name,
                    indentation,
                    initializer_str
                )
            }
            Stmt::Block(statements) => {
                let mut result = format!("{}Block", indentation);
                for statement in statements {
                    result.push_str(&format!(
                        "\n{}├── {}",
                        indentation,
                        statement.pretty_print_with_indent(indent + 1)
                    ));
                }
                result
            }
            Stmt::If(condition, then_branch, else_branch) => {
                let mut result = format!(
                    "{}If\n{}├── Condition: {}\n{}├── Then: {}",
                    indentation,
                    indentation,
                    condition.pretty_print_with_indent(indent + 1),
                    indentation,
                    then_branch.pretty_print_with_indent(indent + 1)
                );
                if let Some(else_stmt) = else_branch {
                    result.push_str(&format!(
                        "\n{}└── Else: {}",
                        indentation,
                        else_stmt.pretty_print_with_indent(indent + 1)
                    ));
                }
                result
            }
            Stmt::While(condition, body) => format!(
                "{}While\n{}├── Condition: {}\n{}└── Body: {}",
                indentation,
                indentation,
                condition.pretty_print_with_indent(indent + 1),
                indentation,
                body.pretty_print_with_indent(indent + 1)
            ),
            Stmt::Function(name, params, body) => {
                let mut result = format!(
                    "{}Function ({})\n{}├── Parameters: {}",
                    indentation,
                    name,
                    indentation,
                    params.join(", ")
                );
                for statement in body {
                    result.push_str(&format!(
                        "\n{}├── {}",
                        indentation,
                        statement.pretty_print_with_indent(indent + 1)
                    ));
                }
                result
            }
            Stmt::Return(expr) => {
                let expr_str = if let Some(expr) = expr {
                    expr.pretty_print_with_indent(indent + 1)
                } else {
                    format!("{}None", indentation)
                };
                format!(
                    "Return\n{}└── {}",
                    indentation,
                    expr_str
                )
            }
            Stmt::Break => format!("{}Break", indentation),
            Stmt::Continue => format!("{}Continue", indentation),
        }
    }
}
