use std::{fs, io::Read};

use clap::Parser;
use logos::{Lexer, Logos};

use crate::token::Token;

pub mod token;
pub mod parser;
pub mod nodes;
pub mod conversion;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let input = fs::File::open(args.input);
    if input.is_err() {
        panic!("Error Result");
    }
    let mut code = String::new();
    let res = input.unwrap().read_to_string(&mut code);
    if res.is_err() {
        panic!("Error Result: {}", res.err().unwrap());
    }
    let lexer: Lexer<'_, Token> = Token::lexer(code.as_str());
    println!("{:#?}", lexer);
    let mut parser = parser::Parser::new();
    let document = parser.parse(lexer);
    if document.is_none() {
        panic!("Failed to parse document");
    }
    println!("{:#?}", document);
    let bytecode = conversion::convert_to_bytes(&mut document.unwrap());
    let res = fs::write(args.output, bytecode);
}