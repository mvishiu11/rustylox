use crate::token::{Token, TokenType};
use crate::expr::{Expr, BinaryExpr, UnaryExpr, LiteralExpr};
use crate::error::ParserError;

/// The Parser struct, responsible for converting a series of tokens into an AST.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Create a new Parser instance.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parse the input tokens and produce an AST.
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
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
        self.equality()
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

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
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

        self.primary()
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