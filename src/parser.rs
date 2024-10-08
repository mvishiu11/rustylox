use crate::token::{Token, TokenType};
use crate::expr::{BinaryExpr, CallExpr, Expr, LiteralExpr, LogicalExpr, UnaryExpr};
use crate::error::ParserError;
use crate::stmt::Stmt;

// The Parser struct, responsible for converting a series of tokens into an AST.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParserError>, // Collects all parsing errors
}

impl Parser {
    /// Create a new Parser instance.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Parse the input tokens and produce an AST. Returns a vector of statements and any errors found.
    pub fn parse(&mut self) -> (Vec<Stmt>, Vec<ParserError>) {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        (statements, self.errors.clone())
    }

    /// Parse a declaration, catching errors and continuing to parse.
    fn declaration(&mut self) -> Option<Stmt> {
        match self.try_declaration() {
            Ok(stmt) => Some(stmt),
            Err(err) => {
                self.errors.push(err);
                self.synchronize();
                None
            }
        }
    }

    fn try_declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else if self.match_token(&[TokenType::Fun]) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }
    

    /// Parse a single statement.
    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_token(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_token(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_token(&[TokenType::Return]) {
            self.return_statement()
        } else if self.match_token(&[TokenType::Fun]) {
            self.function_declaration()
        } else if self.match_token(&[TokenType::While]) { 
            self.while_statement()
        } else if self.match_token(&[TokenType::LeftBrace]) {
            self.block()
        } else if self.match_token(&[TokenType::Var]) {
            self.var_declaration()
        } else if self.match_token(&[TokenType::Break]) {
            self.consume(TokenType::Semicolon, "Expect ';' after 'break'.")?;
            Ok(Stmt::Break)
        } else if self.match_token(&[TokenType::Continue]) {
            self.consume(TokenType::Semicolon, "Expect ';' after 'continue'.")?;
            Ok(Stmt::Continue)
        } else {
            self.expression_statement()
        }
    }

    /// Parse a for statement.
    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
    
        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
    
        let condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(LiteralExpr::Boolean(true))
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
    
        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
    
        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(vec![
                body,
                Stmt::Expression(increment.clone()),
            ]);
        }

        body = Stmt::While(condition, Box::new(body));
    
        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![initializer, body]);
        }
    
        Ok(body)
    }    

    /// Parse an if statement.
    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;
    
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
    
        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    /// Parse a print statement.
    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn return_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
    
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(value))
    }    

    fn function_declaration(&mut self) -> Result<Stmt, ParserError> {
        // Expect function name
        let name_token = self.consume(TokenType::Identifier, "Expect function name.")?;
        let name = name_token.lexeme.clone();
    
        // Parse the parameter list
        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
        let mut parameters = Vec::new();
    
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(self.error(self.peek(), "Cannot have more than 255 parameters."));
                }
    
                let param = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                parameters.push(param.lexeme.clone());
    
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
    
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
    
        // Parse the function body
        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
        let body = self.block()?; // Parses the block of statements
    
        // Return the function statement
        Ok(Stmt::Function(name, parameters, match body {
            Stmt::Block(statements) => statements,
            _ => vec![body],  // Should be a block, but safeguard just in case
        }))
    }   

    /// Parse a while statement.
    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
    
        let body = Box::new(self.statement()?);
    
        Ok(Stmt::While(condition, body))
    }

    /// Parse a block of statements.
    fn block(&mut self) -> Result<Stmt, ParserError> {
        let mut statements = Vec::new();
    
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
    
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        return Ok(Stmt::Block(statements));
    }

    /// Parse a variable declaration.
    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name_token = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let name = name_token.lexeme.clone();
    
        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
    
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
    
        Ok(Stmt::Var(name, initializer))
    }
    

    /// Parse an expression statement.
    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    /// Check if the parser has reached the end of the input tokens.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Get the current token without consuming it.
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Advance the current position and return the previous token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    /// Check if the current token matches any of the given types and consume it if it does.
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types.iter() {
            if self.check(token_type.to_owned()) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Check if the current token matches the given type.
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    /// Handle errors when an unexpected token is encountered.
    fn error(&self, token: &Token, message: &str) -> ParserError {
        ParserError::new(token.line, format!("Error at '{}': {}", token.lexeme, message))
    }

    /// Helper function to synchronize the parser after an error.
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }

    /// Get the previous token.
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Parse an expression. Currently handles equality expressions.
    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    /// Parse assignment expressions.
    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.or()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }

            return Err(self.error(&equals, "Invalid assignment target."));
        }

        Ok(expr)
    }

    /// Parse logical OR expressions.
    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(LogicalExpr { left: expr, operator, right }));
        }

        Ok(expr)
    }

    /// Parse logical AND expressions.
    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(LogicalExpr { left: expr, operator, right }));
        }

        Ok(expr)
    }

    /// Parse equality expressions, handling `==` and `!=` operators.
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(BinaryExpr { left: expr, operator, right }));
        }

        Ok(expr)
    }

    /// Parse comparison expressions, handling `<`, `<=`, `>`, and `>=` operators.
    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(BinaryExpr { left: expr, operator, right }));
        }

        Ok(expr)
    }

    /// Parse terms, handling `+` and `-` operators.
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(BinaryExpr { left: expr, operator, right }));
        }

        Ok(expr)
    }

    /// Parse factors, handling `*` and `/` operators.
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(BinaryExpr { left: expr, operator, right }));
        }

        Ok(expr)
    }

    /// Parse unary expressions, handling `!` and `-` operators.
    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Box::new(UnaryExpr { operator, right })));
        }

        self.call()
    }

    /// Parse function calls.
    fn call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Finish parsing a function call.
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParserError> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(self.error(self.peek(), "Cannot have more than 255 arguments."));
                }
                arguments.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?.clone();

        Ok(Expr::Call(Box::new(CallExpr { callee, paren, arguments })))
    }

    /// Parse primary expressions, handling literals, grouping, etc.
    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr::Boolean(false)));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr::Boolean(true)));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr::Nil));
        }

        if self.match_token(&[TokenType::Number]) {
            return Ok(Expr::Literal(LiteralExpr::Number(
                self.previous().lexeme.parse().unwrap(),
            )));
        }

        if self.match_token(&[TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr::String(
                self.previous().lexeme.clone(),
            )));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    /// Consume a token if it matches the expected type, otherwise return an error.
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }
}