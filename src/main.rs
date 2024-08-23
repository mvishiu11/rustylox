mod lexer;
mod token;
mod expr;
mod parser;
mod error;

use lexer::Lexer;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stdout(), "✨ Program logs will be displayed here. Stay tuned!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut lexer = Lexer::new(file_contents);
                let tokens = lexer.tokenize();
                for token in tokens {
                    println!("{:?}", token);
                }
            } else {
                println!("EOF  null");
            }
        }
        "parse" => {
            writeln!(io::stdout(), "✨ Program logs will be displayed here. Stay tuned!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut lexer = Lexer::new(file_contents);
                let tokens = lexer.tokenize();
                let mut parser = parser::Parser::new(tokens.to_vec());
                match parser.parse() {
                    Ok(expr) => println!("{}", expr.pretty_print()),
                    Err(e) => writeln!(io::stderr(), "{}", e).unwrap(),
                }
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}