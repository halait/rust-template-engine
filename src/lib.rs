use parser::Parser;
use serde_json::Value;
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
program = { declaration } 'End'
declarations = statement
statement = expression
expression = '{{' call '}}' | template_literal
call = identifier { "." identifier }




expression = '{{' identifier '}}'
compound_expression = '{{' 'for' identifier 'in' identifier '}}' statement '{{' 'end' '}}'
                    | '{{' 'if' identifier '}}' statement [ '{{' 'else' '}}' ] '{{' 'end' '}}'
                    | '{{' 
*/

#[wasm_bindgen]
pub fn render(source: &str, context_json: &str) -> String {
    let mut statements = Parser::new(source.as_bytes()).parse();
    let value: Value = serde_json::from_str(context_json).unwrap();
    let mut interperter = Interperter {context: value, result: String::new()};
    interperter.interpret(statements);
    interperter.result
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum TokenType {
    Begin,
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
