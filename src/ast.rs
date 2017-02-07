use token::Token;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Statement {
    LetStatement {
        name: Token,
        value: Expression,
    },
    ReturnStatement {
        value: Expression,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IdentifierExpression {
        token: Token
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Program {
    statements: Vec<Statement>
}

impl Program {

    pub fn new(stmts: Vec<Statement>) -> Self {
        Program { statements: stmts }
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
    assert_eq!(LetStatement{ name: Token::Identifier("x".to_string()), value: IdentifierExpression{ token: Token::Integer(5) } }, program.statements[0]);
    assert_eq!(LetStatement{ name: Token::Identifier("y".to_string()), value: IdentifierExpression{ token: Token::Integer(10) } }, program.statements[1]);
    assert_eq!(LetStatement{ name: Token::Identifier("foobar".to_string()), value: IdentifierExpression{ token: Token::Integer(838383) } }, program.statements[2]);
}

#[test]
fn return_statement_test() {
    let lexer = Lexer::new("
        return 5;
        return 10;
        return 993322;
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{:?}", program);
    assert_eq!(ReturnStatement{ value: IdentifierExpression{ token: Token::Integer(5) } }, program.statements[0]);
    assert_eq!(ReturnStatement{ value: IdentifierExpression{ token: Token::Integer(10) } }, program.statements[1]);
    assert_eq!(ReturnStatement{ value: IdentifierExpression{ token: Token::Integer(993322) } }, program.statements[2]);
}
