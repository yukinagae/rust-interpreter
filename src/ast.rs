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
            LetStatement{ name: ref name, value: ref value } => write!(f, "let {} = {};", name, value.to_string()),
            ReturnStatement{ value: ref value } => write!(f, "return {};", value.to_string()),
            ExpressionStatement{ expression: _ } => write!(f, "{:?}", self),
            BlockStatement{ statements: _ } => write!(f, "{:?}", self),
        }

    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IdentifierExpression{ value: ref value } => write!(f, "{}", value),
            IntegerExpression{ value: value } => write!(f, "{}", value),
            BooleanExpression{ value: value } => write!(f, "{}", value),
            PrefixExpression{ prefix: ref prefix, right: ref expression } => write!(f, "({}{})", prefix, expression.to_string()),
            InfixExpression{ left: ref left, operator: ref operator, right: ref right } => write!(f, "({} {} {})", left.to_string(), operator.to_string(), right.to_string()),
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
