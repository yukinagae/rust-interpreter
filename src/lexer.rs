use token;
use token::Token;

use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {

    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&c) => c == ch,
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut identifier = String::new();
        identifier.push(first);
        while self.peek_is_letter() {
            let ch = self.read_char().unwrap();
            identifier.push(ch);
        }
        identifier
    }

    fn read_number(&mut self, first: char) -> u32 {
        let mut number = String::new();
        number.push(first);
        while let Some(&c) = self.peek_char() {
            if c.is_numeric() {
                let n = self.read_char().unwrap();
                number.push(n);
            } else {
                break;
            }
        }
        number.parse::<u32>().unwrap()
    }

    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();

        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            },
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LowerThan,
            Some('>') => Token::GreaterThan,
            Some('(') => Token::LeftParenthesis,
            Some(')') => Token::RightParenthesis,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some(ch @ _) => {
                if is_letter(ch) {
                    let literal = self.read_identifier(ch);
                    token::lookup_identifier(&literal)
                } else if ch.is_numeric() {
                    let number = self.read_number(ch);
                    Token::Integer(number)
                } else {
                    Token::Illegal
                }
            },
            None => Token::EndOfFile,
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() // || ch == '_'
}

#[test]
#[ignore]
fn next_token_test() {
    let input = "let five = 5;
                 let ten = 10;
                 let add = fn(x, y) {
                    x + y;
                 };

                 let result = add(five, ten);
                 !-/*5;
                 5 < 10 > 5;

                 if (5 < 10) {
                    return true;
                 } else {
                    return false;
                 }

                 10 == 10;
                 10 != 9;
                ";
    let mut lexer = Lexer::new(input);
    assert_eq!(Token::Let, lexer.next_token());
    assert_eq!(Token::Identifier("five".to_string()), lexer.next_token());
    assert_eq!(Token::Assign, lexer.next_token());
    assert_eq!(Token::Integer(5), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::Let, lexer.next_token());
    assert_eq!(Token::Identifier("ten".to_string()), lexer.next_token());
    assert_eq!(Token::Assign, lexer.next_token());
    assert_eq!(Token::Integer(10), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::Let, lexer.next_token());
    assert_eq!(Token::Identifier("add".to_string()), lexer.next_token());
    assert_eq!(Token::Assign, lexer.next_token());
    assert_eq!(Token::Function, lexer.next_token());
    assert_eq!(Token::LeftParenthesis, lexer.next_token());
    assert_eq!(Token::Identifier("x".to_string()), lexer.next_token());
    assert_eq!(Token::Comma, lexer.next_token());
    assert_eq!(Token::Identifier("y".to_string()), lexer.next_token());
    assert_eq!(Token::RightParenthesis, lexer.next_token());
    assert_eq!(Token::LeftBrace, lexer.next_token());
    assert_eq!(Token::Identifier("x".to_string()), lexer.next_token());
    assert_eq!(Token::Plus, lexer.next_token());
    assert_eq!(Token::Identifier("y".to_string()), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::RightBrace, lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::Let, lexer.next_token());
    assert_eq!(Token::Identifier("result".to_string()), lexer.next_token());
    assert_eq!(Token::Assign, lexer.next_token());
    assert_eq!(Token::Identifier("add".to_string()), lexer.next_token());
    assert_eq!(Token::LeftParenthesis, lexer.next_token());
    assert_eq!(Token::Identifier("five".to_string()), lexer.next_token());
    assert_eq!(Token::Comma, lexer.next_token());
    assert_eq!(Token::Identifier("ten".to_string()), lexer.next_token());
    assert_eq!(Token::RightParenthesis, lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::Bang, lexer.next_token());
    assert_eq!(Token::Minus, lexer.next_token());
    assert_eq!(Token::Slash, lexer.next_token());
    assert_eq!(Token::Asterisk, lexer.next_token());
    assert_eq!(Token::Integer(5), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::Integer(5), lexer.next_token());
    assert_eq!(Token::LowerThan, lexer.next_token());
    assert_eq!(Token::Integer(10), lexer.next_token());
    assert_eq!(Token::GreaterThan, lexer.next_token());
    assert_eq!(Token::Integer(5), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());

    assert_eq!(Token::If, lexer.next_token());
    assert_eq!(Token::LeftParenthesis, lexer.next_token());
    assert_eq!(Token::Integer(5), lexer.next_token());
    assert_eq!(Token::LowerThan, lexer.next_token());
    assert_eq!(Token::Integer(10), lexer.next_token());
    assert_eq!(Token::RightParenthesis, lexer.next_token());
    assert_eq!(Token::LeftBrace, lexer.next_token());
    assert_eq!(Token::Return, lexer.next_token());
    assert_eq!(Token::True, lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::RightBrace, lexer.next_token());
    assert_eq!(Token::Else, lexer.next_token());
    assert_eq!(Token::LeftBrace, lexer.next_token());
    assert_eq!(Token::Return, lexer.next_token());
    assert_eq!(Token::False, lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
    assert_eq!(Token::RightBrace, lexer.next_token());

    assert_eq!(Token::Integer(10), lexer.next_token());
    assert_eq!(Token::Equal, lexer.next_token());
    assert_eq!(Token::Integer(10), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());

    assert_eq!(Token::Integer(10), lexer.next_token());
    assert_eq!(Token::NotEqual, lexer.next_token());
    assert_eq!(Token::Integer(9), lexer.next_token());
    assert_eq!(Token::Semicolon, lexer.next_token());
}
