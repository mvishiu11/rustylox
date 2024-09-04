use std::cell::RefCell;
use std::rc::Rc;
use std::env;
use std::io::{self, Write};
use rustylox::environ::Environment;
use rustylox::{run_interpret, run_parse, run_tokenize, lexer::Lexer, parser};
use rustylox::interpreter;

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
    } else {
        filename = &args[2];
    }

    match command.as_str() {
        TOKENIZE => run_tokenize(filename),
        PARSE => run_parse(filename),
        INTERPRET => run_interpret(filename),
        CLI => {
            println!("✨ Program logs will be displayed here. Stay tuned!");

            let mut input = String::new();
            let cli_environ = Rc::new(RefCell::new(Environment::new()));
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

                let output = if !errors.is_empty() {
                    errors.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
                } else {
                    match interpreter::interpret_with_env(&statements, Some(cli_environ.clone())) {
                        Ok(output) => output,
                        Err(e) => e.to_string(),
                    }
                };

                if !output.is_empty() {
                    writeln!(io::stderr(), "{}", output).unwrap();
                }

                input.clear();
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            writeln!(io::stderr(), "Commands: {TOKENIZE} {PARSE} {INTERPRET} {CLI}").unwrap();
        }
    }
}
