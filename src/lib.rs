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
expression = equality
eqaulity = unary {( ('!=' | '==' ) unary )}
unary = ['!'] call 
for = '{{' 'for' identifier 'in' call '}}' statement '{{' 'end' '}}'
if = '{{' if expression '}}' { statement } [ '{{' else '}}'  { statement }] '{{' end '}}'
call = ( identifier { '.' identifier } ) | literal
literal = string
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

#[derive(PartialEq, Debug, Copy, Clone)]
enum TokenType {
    LeftDoubleBrackets,
    RightDoubleBrackets,
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
    Exclaimation
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
