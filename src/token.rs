#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfFile,

    // literals
    Identifier(String),
    Integer(u32),

    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LowerThan,
    GreaterThan,
    Equal,
    NotEqual,

    // delimiters
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn lookup_identifier(key: &str) -> Token {
    match key {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Identifier(key.to_string()),
    }
}

#[test]
fn token_default_test() {
    let token: Token = Default::default();
    assert_eq!(token, Token::Illegal);
}

#[test]
fn lookup_identifier_test() {
    assert_eq!(lookup_identifier("fn"), Token::Function);
}
