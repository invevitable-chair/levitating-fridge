mod ast;
mod lexer;
mod parser;
mod compiler;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: hals-compiler <input.hal>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let source = fs::read_to_string(input_path)
        .expect("Failed to read .hal file");

    let tokens = lexer::lex(&source);
    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse_program();

    let c_code = compiler::compile_to_c(&program);
    fs::write("out.c", c_code).expect("Failed to write out.c");

    println!("Compiled {} -> out.c", input_path);
}
