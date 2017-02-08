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
use self::Precedence::*;

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
        if let Identifier(name) = self.peek_token.clone() {
            self.next_token();
            if !self.expect_peek(Token::Assign) {
                None
            } else {
                self.next_token();
                let value = self.parse_expression(Lowest).unwrap();
                let stmt = LetStatement{ name: name.clone(), value: value };
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
        let value = self.parse_expression(Lowest).unwrap();
        let stmt = ReturnStatement{ value: value};

        if self.peek_token_is(Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Lowest);
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

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = match self.current_token {
            Identifier(_) => self.parse_identifier(),
            Integer(_) => self.parse_integer(),
            True => self.parse_true(),
            False => self.parse_false(),
            Bang => self.parse_prefix(),
            Minus => self.parse_prefix(),
            LeftParenthesis => self.parse_group(),
            If => self.parse_if(),
            _ => None,
        };

        while self.current_token != Semicolon && (precedence.clone() as i32) < (self.peek_precedence() as i32) {
            match self.peek_token {
                Plus => { left = self.parse_infix(left.unwrap()); },
                Minus => { left = self.parse_infix(left.unwrap()); },
                Asterisk => { left = self.parse_infix(left.unwrap()); },
                Slash => { left = self.parse_infix(left.unwrap()); },
                LowerThan => { left = self.parse_infix(left.unwrap()); },
                GreaterThan => { left = self.parse_infix(left.unwrap()); },
                Equal => { left = self.parse_infix(left.unwrap()); },
                NotEqual => { left = self.parse_infix(left.unwrap()); },
                _ => return left,
            }
        }

        left
    }

    fn parse_identifier(&self) -> Option<Expression> {
        match self.current_token {
            Identifier(ref value) => Some(IdentifierExpression { value: value.clone() }),
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

    fn parse_prefix(&mut self) -> Option<Expression> {
        let prefix = self.current_token.clone();
        self.next_token();
        match self.parse_expression(Prefix) {
            Some(right) => {
                let expression = PrefixExpression { prefix: prefix, right: Box::new(right) };
                Some(expression)
            },
            None => None,
        }
    }

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();
        let operator = self.current_token.clone();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);
        Some(InfixExpression{ left: Box::new(left), operator: operator, right: Box::new(right.unwrap()) } )
    }

    fn parse_group(&mut self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression(Lowest);

        if self.expect_peek(RightParenthesis) {
            expression
        } else {
            None
        }
    }

    fn parse_if(&mut self) -> Option<Expression> {

        if !self.expect_peek(LeftParenthesis) {
            return None
        }

        self.next_token();

        let condition = self.parse_expression(Lowest);

        if !self.expect_peek(RightParenthesis) {
            return None
        }

        if !self.expect_peek(LeftBrace) {
            return None
        }

        let consequence = self.parse_block_statement();

        if self.peek_token_is(Else) {
            self.next_token();
            if !self.expect_peek(LeftBrace) {
                return None
            } else {
                let alternative = self.parse_block_statement();
                return Some(IfExpression { condition: Box::new(condition.unwrap()), consequence: Box::new(consequence), alternative: Some(Box::new(alternative)) })
            }
        }

        Some(IfExpression { condition: Box::new(condition.unwrap()), consequence: Box::new(consequence), alternative: None })
    }

    fn parse_block_statement(&mut self) -> Statement {
        self.next_token();

        let mut stmts = Vec::new();

        while self.current_token != RightBrace && self.current_token != EndOfFile {
            if let Some(stmt) = self.parse_statement() {
                stmts.push(stmt);
            }
            self.next_token();
        }

        BlockStatement { statements: stmts }
    }

    fn peek_precedence(&self) -> Precedence {
        match self.peek_token {
            Equal | NotEqual => Equals,
            LowerThan | GreaterThan => LessGreater,
            Plus | Minus => Sum,
            Slash | Asterisk => Product,
            _ => Lowest,
        }
    }

    fn current_precedence(&self) -> Precedence {
        match self.current_token {
            Equal | NotEqual => Equals,
            LowerThan | GreaterThan => LessGreater,
            Plus | Minus => Sum,
            Slash | Asterisk => Product,
            _ => Lowest,
        }
    }

}

#[derive(Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
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
    assert_eq!(LetStatement{ name: "x".to_string(), value: IntegerExpression{ value: 5 } }, program.statements()[0]);
    assert_eq!(LetStatement{ name: "y".to_string(), value: IntegerExpression{ value: 10 } }, program.statements()[1]);
    assert_eq!(LetStatement{ name: "foobar".to_string(), value: IntegerExpression{ value: 838383 } }, program.statements()[2]);
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
#[ignore]
fn expression_statement_test() {
    let lexer = Lexer::new("
        foobar;
        5;
        !5;
        -15;
        5 + 6;
        5 - 6;
        5 * 6;
        5 / 6;
        5 < 6;
        5 > 6;
        5 == 6;
        5 != 6;
    ");

    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{:?}", program);
    assert_eq!(ExpressionStatement{ expression: IdentifierExpression{ value: "foobar".to_string() } }, program.statements()[0]);
    assert_eq!(ExpressionStatement{ expression: IntegerExpression{ value: 5 } }, program.statements()[1]);
    assert_eq!(ExpressionStatement{ expression: PrefixExpression{ prefix: Bang, right: Box::new(IntegerExpression{ value: 5 }) } }, program.statements()[2]);
    assert_eq!(ExpressionStatement{ expression: PrefixExpression{ prefix: Minus, right: Box::new(IntegerExpression{ value: 15 }) } }, program.statements()[3]);
    assert_eq!(ExpressionStatement{ expression: InfixExpression{ left: Box::new(IntegerExpression{ value: 5 }), operator: Plus, right: Box::new(IntegerExpression{ value: 6 }) } }, program.statements()[4]);
    assert_eq!(ExpressionStatement{ expression: InfixExpression{ left: Box::new(IntegerExpression{ value: 5 }), operator: Minus, right: Box::new(IntegerExpression{ value: 6 }) } }, program.statements()[5]);
    assert_eq!(ExpressionStatement{ expression: InfixExpression{ left: Box::new(IntegerExpression{ value: 5 }), operator: Asterisk, right: Box::new(IntegerExpression{ value: 6 }) } }, program.statements()[6]);
    assert_eq!(ExpressionStatement{ expression: InfixExpression{ left: Box::new(IntegerExpression{ value: 5 }), operator: Slash, right: Box::new(IntegerExpression{ value: 6 }) } }, program.statements()[7]);
    assert_eq!(ExpressionStatement{ expression: InfixExpression{ left: Box::new(IntegerExpression{ value: 5 }), operator: LowerThan, right: Box::new(IntegerExpression{ value: 6 }) } }, program.statements()[8]);
    assert_eq!(ExpressionStatement{ expression: InfixExpression{ left: Box::new(IntegerExpression{ value: 5 }), operator: GreaterThan, right: Box::new(IntegerExpression{ value: 6 }) } }, program.statements()[9]);
    assert_eq!(ExpressionStatement{ expression: InfixExpression{ left: Box::new(IntegerExpression{ value: 5 }), operator: Equal, right: Box::new(IntegerExpression{ value: 6 }) } }, program.statements()[10]);
    assert_eq!(ExpressionStatement{ expression: InfixExpression{ left: Box::new(IntegerExpression{ value: 5 }), operator: NotEqual, right: Box::new(IntegerExpression{ value: 6 }) } }, program.statements()[11]);
}

#[test]
#[ignore]
fn parse_operator_precedence_test() {
    let lexer = Lexer::new("
        -a * b;
        !-a;
        a + b + c;
        a + b - c;
        a * b * c;
        a * b / c;
        a + b / c;
        a + b * c + d / e - f;
        5 > 4 == 3 < 4;
        5 < 4 != 3 > 4;
        3 + 4 * 5 == 3 * 1 + 4 * 5;
        3 + 4 * 5 == 3 * 1 + 4 * 5;
        3 + 4; -5 * 5;
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!("((-a) * b)", program.statements()[0].to_string());
    assert_eq!("(!(-a))", program.statements()[1].to_string());
    assert_eq!("((a + b) + c)", program.statements()[2].to_string());
    assert_eq!("((a + b) - c)", program.statements()[3].to_string());
    assert_eq!("((a * b) * c)", program.statements()[4].to_string());
    assert_eq!("((a * b) / c)", program.statements()[5].to_string());
    assert_eq!("(a + (b / c))", program.statements()[6].to_string());

    assert_eq!("(((a + (b * c)) + (d / e)) - f)", program.statements()[7].to_string());
    assert_eq!("((5 > 4) == (3 < 4))", program.statements()[8].to_string());
    assert_eq!("((5 < 4) != (3 > 4))", program.statements()[9].to_string());
    assert_eq!("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))", program.statements()[10].to_string());
    assert_eq!("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))", program.statements()[11].to_string());
    assert_eq!("(3 + 4)", program.statements()[12].to_string());
    assert_eq!("((-5) * 5)", program.statements()[13].to_string());
}

#[test]
#[ignore]
fn parse_operator_boolean_test() {
    let lexer = Lexer::new("
        true;
        false;
        3 > 5 == false;
        3 < 5 == true;
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!("true", program.statements()[0].to_string());
    assert_eq!("false", program.statements()[1].to_string());
    assert_eq!("((3 > 5) == false)", program.statements()[2].to_string());
    assert_eq!("((3 < 5) == true)", program.statements()[3].to_string());
}

#[test]
#[ignore]
fn parse_grouped_expressions_test() {
    let lexer = Lexer::new("
        1 + (2 + 3) + 4;
        (5 + 5) * 2;
        2 / (5 + 5);
        -(5 + 5);
        !(true == true);
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!("((1 + (2 + 3)) + 4)", program.statements()[0].to_string());
    assert_eq!("((5 + 5) * 2)", program.statements()[1].to_string());
    assert_eq!("(2 / (5 + 5))", program.statements()[2].to_string());
    assert_eq!("(-(5 + 5))", program.statements()[3].to_string());
    assert_eq!("(!(true == true))", program.statements()[4].to_string());
}

#[test]
// #[ignore]
fn parse_if_test() {
    let lexer = Lexer::new("
        if (x < y) { x } else { y }
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    // println!("{:?}", program.statements()[0]);

    if let ExpressionStatement {ref expression} = program.statements()[0] {
        match *expression {
            IfExpression{ ref condition, ref consequence, ref alternative } => {
                assert_eq!(Box::new(InfixExpression{ left: Box::new(IdentifierExpression{value: "x".to_string()}), operator: LowerThan, right: Box::new(IdentifierExpression{value: "y".to_string()})}), *condition);
                match *consequence {
                    _ => {},
                }
                assert_eq!( Box::new(BlockStatement{ statements: vec![ExpressionStatement{ expression: IdentifierExpression{ value: "x".to_string() }}] }), *consequence);
                match *alternative {
                    Some(ref alt) => {
                        assert_eq!( Box::new(BlockStatement{ statements: vec![ExpressionStatement{ expression: IdentifierExpression{ value: "y".to_string() }}] }), *alt);
                    },
                    None => assert!(false),
                }
            },
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}
