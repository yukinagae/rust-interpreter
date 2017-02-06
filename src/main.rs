use std::io;
use std::io::BufRead;
use std::io::Write;

pub mod lexer;
use lexer::Lexer;

pub mod token;
use token::Token;

fn main() {

    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        let mut lexer = Lexer::new(&line);

        loop {
            let token = lexer.next_token();
            println!("{:?}", token);
            if token == Token::EndOfFile {
                break;
            }
        }

    }

}

