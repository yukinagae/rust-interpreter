use lexer::Lexer;
use token::Token;
use ast;
#[allow(unused_imports)]
use ast::Statement;
use ast::Statement::*;
use ast::Expression;
use ast::Expression::IdentifierExpression;
use ast::Program;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser { lexer: lexer, current_token: Token::Illegal, peek_token: Token::Illegal };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&mut self, t: Token) -> bool {
        self.current_token == t
    }

    fn peek_token_is(&mut self, t: Token) -> bool {
        self.peek_token == t
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            // TODO error!
            false
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();
        while self.current_token != Token::EndOfFile {
            let stmt = self.parse_statement();
            match stmt {
                Some(s) => statements.push(s),
                None => {},
            }
            self.next_token();
        }
        Program::new(statements)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if let Token::Identifier(_) = self.peek_token {
            let name = self.peek_token.clone();
            self.next_token();
            if !self.expect_peek(Token::Assign) {
                None
            } else {
                self.next_token();
                let value = IdentifierExpression { token: self.current_token.clone() };
                let stmt = LetStatement{ name: name, value: value };
                if self.peek_token_is(Token::Semicolon) {
                    self.next_token();
                }
                Some(stmt)
            }
        } else {
            None
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let value = IdentifierExpression { token: self.current_token.clone() };
        let stmt = ReturnStatement{ value: value};

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }
}
