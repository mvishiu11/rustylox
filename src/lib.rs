use wasm_bindgen::prelude::*;
use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::io::{self, Write};

pub mod lexer;
pub mod token;
pub mod expr;
pub mod parser;
pub mod error;
pub mod stmt;
pub mod environ;
pub mod interpreter;
mod callable;
mod natives;

#[wasm_bindgen]
pub fn tokenize(file_contents: &str) -> String {
    let mut lexer = Lexer::new(file_contents.to_string());
    let tokens = lexer.tokenize();
    tokens.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>().join("\n")
}

#[wasm_bindgen]
pub fn parse(file_contents: &str) -> String {
    let mut lexer = Lexer::new(file_contents.to_string());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens.to_vec());
    let (statements, errors) = parser.parse();

    if !errors.is_empty() {
        errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
    } else {
        format!("Parsed statements: {:?}", statements)
    }
}

#[wasm_bindgen]
pub fn interpret(file_contents: &str) -> String {
    let mut lexer = Lexer::new(file_contents.to_string());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens.to_vec());
    let (statements, errors) = parser.parse();

    if !errors.is_empty() {
        errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
    } else {
        match interpreter::interpret(&statements) {
            Ok(output) => output,
            Err(e) => e.to_string(),
        }
    }
}

// CLI functions, which call the above functions
pub fn run_tokenize(filename: &str) {
    let file_contents = read_file(filename);
    println!("{}", tokenize(&file_contents));
}

pub fn run_parse(filename: &str) {
    let file_contents = read_file(filename);
    println!("{}", parse(&file_contents));
}

pub fn run_interpret(filename: &str) {
    let file_contents = read_file(filename);
    println!("{}", interpret(&file_contents));
}

pub fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    })
}
