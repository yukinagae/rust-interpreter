#[allow(dead_code)]
#[allow(unused_imports)]

use std::io;
use std::io::BufRead;
use std::io::Write;

pub mod lexer;
use lexer::Lexer;

pub mod token;
use token::Token;

pub mod ast;
use ast::Parser;

fn main() {

    let lexer = Lexer::new("
        let x = 5;
        let y = 10;
        let foobar = 838383;
        return 5;
    ");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{:?}", program);

    // let stdin = io::stdin();

    // loop {
    //     print!(">> ");
    //     io::stdout().flush().expect("Error flushing stdout");

    //     let mut line = String::new();
    //     stdin.lock().read_line(&mut line).expect("Error reading from stdin");

    //     let mut lexer = Lexer::new(&line);

    //     loop {
    //         let token = lexer.next_token();
    //         println!("{:?}", token);
    //         if token == Token::EndOfFile {
    //             break;
    //         }
    //     }

    // }

}

