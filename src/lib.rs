use std::{fs, path::Path, io::Write};

use parser::Parser;
use serde_json::Value;
use tokenizer::Tokenizer;
use wasm_bindgen::prelude::*;

use crate::interperter::Interperter;

pub mod tokenizer;
pub mod parser;
pub mod message_formatter;
pub mod statement;
pub mod expression;
pub mod interperter;

/*
Grammer rules in Extended Backus–Naur Form (EBNF)
program = { statement }
statement = template_literal
            | '{{' expression '}}'
            | for
            | if
expression = or
or = and { '||' and }
and = equality { '&&' equality }
equality = unary {( ('!=' | '==' ) unary )}
unary = ['!'] call 
call = ( identifier { '.' identifier } ) | literal
literal = string

for = '{{' 'for' identifier 'in' call '}}' statement '{{' 'end' '}}'
if = '{{' if expression '}}' { statement } [ '{{' else '}}'  { statement }] '{{' end '}}'
*/

#[wasm_bindgen]
pub fn render(source: &str, context_json: &str) -> String {
    let binding = Tokenizer::new(source.as_bytes());
    let tokens = binding.tokenize();
    let binding = Parser::new(&tokens);
    let statements = binding.parse();
    let value: Value = serde_json::from_str(context_json).unwrap();
    let interperter = Interperter::new(value);
    interperter.interpret(&statements)
}

pub fn render_file(source_path: &str, context_json_path: &str) {
    let source = fs::read_to_string(&source_path)
        .expect("Should have been able to read the file");
    let json = fs::read_to_string(&context_json_path)
        .expect("Should have been able to read the file");
    let output = render(&source, &json);
    println!("{}", &output);
    let path = Path::new(&source_path);
    let file_stem = path.file_stem().expect("Unable to parse source filename");
    let extension = path.extension().expect("Unable to parse source file extension");
    let mut file = fs::File::create([file_stem.to_str().unwrap(), "_yartle_out.", extension.to_str().unwrap()].join(""))
        .expect("Error writing file");
    file.write_all(output.as_bytes()).unwrap();
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum TokenType {
    DoubleLeftBrackets,
    DoubleRightBrackets,
    For,
    In,
    If,
    Else,
    When,
    Identifier,
    String,
    TempalteLiteral,
    Dot,
    End,
    DoubleEquals,
    ExclaimationEqual,
    Exclaimation,
    DoubleAmpersand,
    DoublePipe
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Token<'a> {
    token_type: TokenType,
    token_value: &'a [u8]
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
        
    // }
}
