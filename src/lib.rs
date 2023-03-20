// use serde_json::{Result, Value};
pub mod tokenizer;
pub mod parser;
pub mod message_formatter;
/*
    Grammer rules in Extended Backus–Naur Form (EBNF)
    statement = template_literal
                | expression
                | compound_expression
    expression = '{{' identifier '}}'
    compound_expression = '{{' 'for' identifier 'in' identifier '}}' statement '{{' 'end' '}}'
                        | '{{' 'if' identifier '}}' statement [ '{{' 'else' '}}' ] '{{' 'end' '}}'
                        | '{{' 
*/
#[derive(PartialEq, Debug, Copy, Clone)]
enum TokenType {
    LeftDoubleBrackets,
    RightDoubleBrackets,
    For,
    If,
    In,
    When,
    Identifier,
    StringLiteral,
    TempalteLiteral,
    END
}

#[derive(PartialEq, Debug)]
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
