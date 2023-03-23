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
program = { statement } 'End'
statement = template_literal
            | '{{' expression '}}'
            | for
            | if
expression = call
for = '{{' 'for' identifier 'in' call '}}' statement '{{' 'end' '}}'
if = '{{' if expression '}}' { statement } [ '{{' else '}}'  { statement }] '{{' end '}}'
call = identifier { "." identifier }
*/

#[wasm_bindgen]
pub fn render(source: &str, context_json: &str) -> String {
    let mut tokenizer = Tokenizer::new(source.as_bytes());
    let mut parser = Parser::new();
    parser.init(&mut tokenizer);
    let statements = parser.parse(&mut tokenizer);
    let value: Value = serde_json::from_str(context_json).unwrap();
    let mut interperter = Interperter::new(value);
    interperter.interpret(&statements)
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum TokenType {
    LeftDoubleBrackets,
    RightDoubleBrackets,
    For,
    If,
    In,
    When,
    Identifier,
    String,
    TempalteLiteral,
    Dot,
    End
}

#[derive(PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    token_value: String
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    // }
}
