#![allow(unused_imports)]
#![allow(dead_code)]

use token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetStatement {
        name: Token,
        value: Expression,
    },
    ReturnStatement {
        value: Expression
    },
    ExpressionStatement {
        expression: Expression,
    },
    BlockStatement {
        statements: Vec<Statement>
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IdentifierExpression {
        value: Token
    },
    IntegerExpression {
        value: u32
    },
    BooleanExpression {
        value: bool
    },
    PrefixExpression {
        prefix: Token,
        right: Box<Expression>
    },
    InfixExpression {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    statements: Vec<Statement>
}

impl Program {

    pub fn new(stmts: Vec<Statement>) -> Self {
        Program { statements: stmts }
    }

    // TODO for testing purpose only
    pub fn statements(&self) -> Vec<Statement> {
        self.statements.clone()
    }
}
