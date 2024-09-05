use std::collections::HashMap;

use crate::{expr::Expr, stmt::Stmt, token::Token};

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    /// Begin a new block scope
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// End a block scope
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    /// Declare a variable in the current scope
    fn declare(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), false);
        }
    }

    /// Define a variable (i.e., mark it as initialized)
    fn define(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), true);
        }
    }

    /// Resolve a variable, determining whether it is in the current scope or a global
    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(name) {
                return Some(i);
            }
        }
        None
    }

    fn resolve_var_declaration(&mut self, name: &str, initializer: Option<&Expr>) {
        self.declare(name);
        if let Some(init_expr) = initializer {
            self.resolve_expr(init_expr);
        }
        self.define(name);
    }    

    fn resolve_block(&mut self, statements: &[Stmt]) {
        self.begin_scope();
        for statement in statements {
            self.resolve_stmt(statement);
        }
        self.end_scope();
    }

    fn resolve_variable(&mut self, name: &Token) {
        if let Some(depth) = self.resolve_local(&name.lexeme) {
            self.mark_variable(name, depth);
        } else {
            self.mark_variable(name, 0);
        }
    }

    fn mark_variable(&mut self, name: &Token, depth: usize) {
        if let Some(scope) = self.scopes.get_mut(depth) {
            scope.insert(name.lexeme.clone(), true);
        }
    }
    
    fn resolve_function(&mut self, name: &str, params: &[String], body: &[Stmt]) {
        self.declare(name);
        self.define(name);
    
        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_block(body);
        self.end_scope();
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Variable(name) => {
                // Resolve the variable
                self.resolve_variable(name);
            }
            Expr::Assign(name, value) => {
                // Resolve the value being assigned to the variable
                self.resolve_expr(value);
                // Resolve the variable itself (find its depth)
                self.resolve_variable(name);
            }
            Expr::Binary(binary_expr) => {
                self.resolve_expr(&binary_expr.left);
                self.resolve_expr(&binary_expr.right);
            }
            Expr::Call(call_expr) => {
                self.resolve_expr(&call_expr.callee);
                for arg in &call_expr.arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::Grouping(expr) => {
                self.resolve_expr(expr);
            }
            Expr::Literal(_) => {}
            Expr::Logical(logical_expr) => {
                self.resolve_expr(&logical_expr.left);
                self.resolve_expr(&logical_expr.right);
            }
            Expr::Unary(unary_expr) => {
                self.resolve_expr(&unary_expr.right);
            }
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(statements) => {
                self.resolve_block(statements);
            }
            Stmt::Expression(expr) => {
                self.resolve_expr(expr);
            }
            Stmt::Function(name, params, body) => {
                self.resolve_function(name, params, body);
            }
            Stmt::If(condition, then_branch, else_branch) => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);
                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(else_branch);
                }
            }
            Stmt::Print(expr) => {
                self.resolve_expr(expr);
            }
            Stmt::Return(value) => {
                if let Some(value) = value {
                    self.resolve_expr(value);
                }
            }
            Stmt::Var(name, initializer) => {
                self.resolve_var_declaration(name, initializer.as_ref());
            }
            Stmt::While(condition, body) => {
                self.resolve_expr(condition);
                self.resolve_stmt(body);
            }
            _ => {
                // Do nothing
            }
        }
    }    
}
