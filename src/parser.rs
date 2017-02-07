#![allow(unused_imports)]
#![allow(dead_code)]

use lexer::Lexer;
use token::Token;
use token::Token::*;
use ast;
use ast::Statement;
use ast::Statement::*;
use ast::Expression;
use ast::Expression::*;
use ast::Program;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

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
        while self.current_token != EndOfFile {
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
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if let Identifier(_) = self.peek_token {
            let name = self.peek_token.clone();
            self.next_token();
            if !self.expect_peek(Token::Assign) {
                None
            } else {
                self.next_token();
                let value = IdentifierExpression { value: self.current_token.clone() };
                let stmt = LetStatement{ name: name, value: value };
                if self.peek_token_is(Semicolon) {
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
        let value = IdentifierExpression { value: self.current_token.clone() };
        let stmt = ReturnStatement{ value: value};

        if self.peek_token_is(Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression();
        match expression {
            None => return None,
            Some(_) => {},
        }
        let stmt = ExpressionStatement{ expression: expression.unwrap()};

        if self.peek_token_is(Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match self.current_token {
            Identifier(_) => self.parse_identifier(),
            Integer(_) => self.parse_integer(),
            True => self.parse_true(),
            False => self.parse_false(),
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Option<Expression> {
        match self.current_token {
            ref token@Identifier(_) => Some(IdentifierExpression { value: token.clone() }),
            _ => None,
        }
    }

    fn parse_integer(&self) -> Option<Expression> {
        match self.current_token {
            Integer(value) => Some(IntegerExpression { value: value }),
            _ => None,
        }
    }

    fn parse_true(&self) -> Option<Expression> {
        Some(BooleanExpression { value: true })
    }

    fn parse_false(&self) -> Option<Expression> {
        Some(BooleanExpression { value: false })
    }
}

#[test]
#[ignore]
fn let_statement_test() {
    let lexer = Lexer::new("
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{:?}", program);
    assert_eq!(LetStatement{ name: Identifier("x".to_string()), value: IntegerExpression{ value: 5 } }, program.statements()[0]);
    assert_eq!(LetStatement{ name: Identifier("y".to_string()), value: IntegerExpression{ value: 10 } }, program.statements()[1]);
    assert_eq!(LetStatement{ name: Identifier("foobar".to_string()), value: IntegerExpression{ value: 838383 } }, program.statements()[2]);
}

#[test]
#[ignore]
fn return_statement_test() {
    let lexer = Lexer::new("
        return 5;
        return 10;
        return 993322;
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{:?}", program);
    assert_eq!(ReturnStatement{ value: IntegerExpression{ value: 5 } }, program.statements()[0]);
    assert_eq!(ReturnStatement{ value: IntegerExpression{ value: 10 } }, program.statements()[1]);
    assert_eq!(ReturnStatement{ value: IntegerExpression{ value: 993322 } }, program.statements()[2]);
}

#[test]
fn expression_statement_test() {
    let lexer = Lexer::new("
        foobar;
        5;
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{:?}", program);
    assert_eq!(ExpressionStatement{ expression: IdentifierExpression{ value: Identifier("foobar".to_string()) } }, program.statements()[0]);
    assert_eq!(ExpressionStatement{ expression: IntegerExpression{ value: 5 } }, program.statements()[1]);
}

