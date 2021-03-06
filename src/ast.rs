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
            BlockStatement{ ref statements } => {
                let mut stmts = String::new();
                for s in statements {
                    stmts.push_str(&s.to_string());
                    stmts.push_str("; ");
                }
                write!(f, "{{ {} }}", stmts)
            },
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
            IfExpression { ref condition, ref consequence, ref alternative } => write!(f, "(if {} {{ {} }} else {{ {:?} }})", condition.to_string(), consequence.to_string(), alternative),
            FunctionExpression { ref parameters, ref body } => write!(f, "fn({}) {}", parameters.join(", "), body.to_string()),
            CallExpression { ref name, ref arguments } => {
                let mut exprs = Vec::new();
                for a in arguments {
                    exprs.push(a.to_string());
                }
                write!(f, "{}({})", name, exprs.join(", "))
            },
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
    },
    IfExpression {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>
    },
    FunctionExpression {
        parameters: Vec<String>,
        body: Box<Statement>
    },
    CallExpression {
        name: String,
        arguments: Vec<Expression>
    },
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

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stmts = String::new();
        let ref statements = self.statements;
        for s in statements {
            stmts.push_str(&s.to_string());
            // stmts.push_str("; ");
        }
        write!(f, "{}", stmts)
    }
}
