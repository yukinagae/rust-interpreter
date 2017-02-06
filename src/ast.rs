#[allow(dead_code)]
#[allow(unused_imports)]

use lexer;
use lexer::Lexer;

use token::Token;

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
        let mut statements: Vec<LetStatement> = Vec::new();
        while self.current_token != Token::EndOfFile {
            let stmt = self.parse_let_statement();
            match stmt {
                Some(s) => {
                    statements.push(s);
                }
                None => {},
            }
            self.next_token();
        }
        Program { statements: statements }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        if let Token::Identifier(_) = self.peek_token {
            let name = self.peek_token.clone();
            self.next_token();
            if !self.expect_peek(Token::Assign) {
                None
            } else {
                self.next_token();
                let value = IdentifierExpression { token: self.current_token.clone() };
                let stmt = LetStatement{ token: Token::Let, name: name, value: value };
                if self.peek_token_is(Token::Semicolon) {
                    self.next_token();
                }
                Some(stmt)
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    token: Token
}

struct Statement {
}

#[derive(Debug, PartialEq)]
struct LetStatement {
    token: Token,
    name: Token,
    value: IdentifierExpression,
}

#[derive(Debug, PartialEq)]
struct IdentifierExpression {
    token: Token,
}

struct Expression {
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<LetStatement>
}

impl Program {

    fn token(&self) -> Token {
        if self.statements.len() > 0 {
            self.statements[0].token.clone()
        } else {
            Token::Illegal
        }
    }
}


#[test]
fn let_statement_test() {
    let lexer = Lexer::new("
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{:?}", program);
    assert_eq!(LetStatement{ token: Token::Let, name: Token::Identifier("x".to_string()), value: IdentifierExpression{ token: Token::Integer(5) } }, program.statements[0]);
    assert_eq!(LetStatement{ token: Token::Let, name: Token::Identifier("y".to_string()), value: IdentifierExpression{ token: Token::Integer(10) } }, program.statements[1]);
    assert_eq!(LetStatement{ token: Token::Let, name: Token::Identifier("foobar".to_string()), value: IdentifierExpression{ token: Token::Integer(838383) } }, program.statements[2]);
}
