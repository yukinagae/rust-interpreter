#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt;
use token::Token;
use self::Statement::*;
use self::Expression::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetStatement {
        name: String,
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

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LetStatement{ ref name, ref value } => write!(f, "let {} = {}", name, value.to_string()),
            ReturnStatement{ ref value } => write!(f, "return {}", value.to_string()),
            ExpressionStatement{ ref expression } => write!(f, "{}", expression.to_string()),
            BlockStatement{ statements: _ } => write!(f, "{:?}", self),
        }

    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IdentifierExpression{ ref value } => write!(f, "{}", value),
            IntegerExpression{ value } => write!(f, "{}", value),
            BooleanExpression{ value } => write!(f, "{}", value),
            PrefixExpression{ ref prefix, ref right } => write!(f, "({}{})", prefix, right.to_string()),
            InfixExpression{ ref left, ref operator, ref right } => write!(f, "({} {} {})", left.to_string(), operator.to_string(), right.to_string()),
        }

    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IdentifierExpression {
        value: String
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
