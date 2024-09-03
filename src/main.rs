mod lexer;
mod token;
mod expr;
mod parser;
mod error;
mod interpreter;
mod stmt;
mod environ;

use lexer::Lexer;
use std::env;
use std::fs;
use std::io::{self, Write};

const TOKENIZE: &str = "tokenize";
const PARSE: &str = "parse";
const INTERPRET: &str = "interpret";
const CLI: &str = "cli";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        writeln!(io::stderr(), "Commands: {TOKENIZE} {PARSE} {INTERPRET} {CLI}").unwrap();
        return;
    }

    let command = &args[1];
    let mut filename = "";
    if command == CLI {
        println!("🚀 Welcome to the Lox programming language REPL!");
    }
    else {
        filename = &args[2];
    }

    match command.as_str() {
        TOKENIZE => {
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
        },
        PARSE => {
            writeln!(io::stdout(), "✨ Program logs will be displayed here. Stay tuned!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut lexer = Lexer::new(file_contents);
                let tokens = lexer.tokenize();
                let mut parser = parser::Parser::new(tokens.to_vec());
                let (statements, errors) = parser.parse();
                
                if !errors.is_empty() {
                    for error in errors {
                        writeln!(io::stderr(), "{}", error).unwrap();
                    }
                } else {
                    println!("Parsed statements: {:?}", statements);
                }
            } else {
                println!("EOF  null");
            }
        },
        INTERPRET => {
            writeln!(io::stdout(), "✨ Program logs will be displayed here. Stay tuned!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut lexer = Lexer::new(file_contents);
                let tokens = lexer.tokenize();
                let mut parser = parser::Parser::new(tokens.to_vec());
                let (statements, errors) = parser.parse();

                if !errors.is_empty() {
                    for error in errors {
                        writeln!(io::stderr(), "{}", error).unwrap();
                    }
                } else {
                    match interpreter::interpret(&statements) {
                        Ok(()) => (),
                        Err(e) => writeln!(io::stderr(), "{}", e).unwrap(),
                    }
                }
            } else {
                println!("EOF  null");
            }
        }
        CLI => {
            writeln!(io::stdout(), "✨ Program logs will be displayed here. Stay tuned!").unwrap();

            let mut input = String::new();
            loop {
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "exit" {
                    break;
                }

                let mut lexer = Lexer::new(input.clone());
                let tokens = lexer.tokenize();
                let mut parser = parser::Parser::new(tokens.to_vec());
                let (statements, errors) = parser.parse();

                if !errors.is_empty() {
                    for error in errors {
                        writeln!(io::stderr(), "{}", error).unwrap();
                    }
                } else {
                    match interpreter::interpret(&statements) {
                        Ok(()) => (),
                        Err(e) => writeln!(io::stderr(), "{}", e).unwrap(),
                    }
                }

                input.clear();
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            writeln!(io::stderr(), "Commands: {TOKENIZE} {PARSE} {INTERPRET} {CLI}").unwrap();
            return;
        }
    }
}