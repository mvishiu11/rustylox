use std::cell::RefCell;
use std::rc::Rc;
use std::{env, string};
use std::io::{self, Write};
use rustylox::environ::Environment;
use rustylox::resolver::Resolver;
use rustylox::stmt::pretty_print_program;
use rustylox::{run_interpret, read_file, run_tokenize, lexer::Lexer, parser, parser::Parser};
use rustylox::interpreter;
use rustylox::natives::define_native_functions;

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
        PARSE => {
            let file_contents = read_file(filename);
            let mut lexer = Lexer::new(file_contents.to_string());
            let tokens = lexer.tokenize();
            let mut parser = Parser::new(tokens.to_vec());
            let (statements, errors) = parser.parse();
            let parsed = pretty_print_program((statements, errors));
            print!("{}", parsed);
        }
        INTERPRET => run_interpret(filename),
        CLI => {
            println!("✨ Program logs will be displayed here. Stay tuned!");

            let mut input = String::new();
            let cli_environ = Rc::new(RefCell::new(Environment::new()));
            define_native_functions(&mut cli_environ.borrow_mut());
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
                let mut resolver = Resolver::new();
                resolver.resolve(&statements);

                let output = if !errors.is_empty() {
                    errors.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
                } else {
                    match interpreter::interpret_with_env(&statements, Some(cli_environ.clone()), &resolver, &mut string::String::new()) {
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
