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
        token: Token
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
}
