#![allow(dead_code)]
#![allow(unused_imports)]

use std::io;
use std::io::BufRead;
use std::io::Write;

pub mod lexer;
use lexer::Lexer;

pub mod token;
use token::Token;

pub mod ast;

pub mod parser;
use parser::Parser;

fn main() {

    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        let lexer = Lexer::new(&line);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        println!("{}", program);
    }

}

