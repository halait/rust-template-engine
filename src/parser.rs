use std::{cell::RefCell, ops::Deref};

use crate::{Token, TokenType, statement::{Statement, self}, expression::{Expression, self}};

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    i: RefCell<usize>
}

impl<'a, 'b> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, i: RefCell::new(0) }
    }
    
    // pub fn init(&mut self) {
    //     self.next_token();
    // }

    fn next_token(&self) -> Option<&Token> {
        let old = *self.i.borrow();
        self.i.replace(old + 1);
        // self.i += 1;
        self.current_token()
    }

    fn current_token(&self) -> Option<&Token> {
        let i = self.i.borrow();
        if self.tokens.len() <= *i { None } else { Some(&self.tokens[*i]) }
    }

    fn expect(&self, token_type: TokenType) {
        self.on(token_type);
        self.next_token();
    }

    fn on(&self, token_type: TokenType) {
        let token = self.current_token().expect(&format!("Unexpected end of input, expected token: {:?}", token_type));
        if token.token_type != token_type {
            panic!("Unexpected token: {:?}, expected: {:?}", token.token_type, token_type);
        }
    }

    pub fn parse(&self) -> Vec::<Statement> {
        let mut statements: Vec::<Statement> = Vec::new();
        loop {
            if self.current_token() == None {
                return statements;
            }
            match self.parse_statement() {
                Some(statement) => {
                    statements.push(statement);
                }
                None => {
                    return statements
                }
            }
        }
    }

    fn parse_statement(&self) -> Option<Statement> {
        match self.current_token()?.token_type {
            TokenType::LeftDoubleBrackets => {
                match self.next_token()?.token_type {
                    TokenType::Identifier => {
                        Some(Statement::Expression(self.parse_expression()))
                    }
                    TokenType::For => {
                        Some(self.parse_for())
                    }
                    TokenType::LeftDoubleBrackets => todo!(),
                    TokenType::RightDoubleBrackets => todo!(),
                    TokenType::If => todo!(),
                    TokenType::In => todo!(),
                    TokenType::When => todo!(),
                    TokenType::String => todo!(),
                    TokenType::TempalteLiteral => todo!(),
                    TokenType::Dot => todo!(),
                    TokenType::End => {
                        None
                    }
                }
            }
            TokenType::TempalteLiteral => {
                let expression = Expression::TemplateLiteral(expression::TemplateLiteralExpression {
                    value: self.current_token()?.token_value
                });
                self.next_token();
                Some(Statement::Expression(expression))
            }
            _ => {
                todo!("{:?}", self.current_token()?);
            }
        }
    }

    fn parse_expression(&self) -> Expression {
        self.parse_call()
    }

    fn parse_call(&self) -> Expression {
        let mut expression = self.parse_identifier();
        
        while self.current_token().expect("Unexpected end of input").token_type == TokenType::Dot {
            self.next_token();
            self.on(TokenType::Identifier);
            expression = Expression::Call(expression::CallExpression {
                callee: Box::new(Statement::Expression(expression)),
                name: self.current_token().unwrap().token_value
            });
            self.next_token();
        }
        self.expect(TokenType::RightDoubleBrackets);
        expression
    }

    fn parse_identifier(&self) -> Expression {
        let expression = Expression::Variable(expression::VariableExpression {
            name: self.current_token().unwrap().token_value
        });
        self.next_token();
        expression
    }

    fn parse_for(&self) -> Statement {
        self.expect(TokenType::For);
        self.on(TokenType::Identifier);
        let instance_identifier = self.current_token().unwrap().token_value;
        self.next_token();
        self.expect(TokenType::In);
        self.on(TokenType::Identifier);
        let array_variable = self.parse_call();
        let statements = self.parse();
        self.expect(TokenType::End);
        self.expect(TokenType::RightDoubleBrackets);
        Statement::For(statement::ForStatement{
            instance_identifier: instance_identifier,
            array_variable: Box::new(Statement::Expression(array_variable)),
            statements
        })
    }
}


#[cfg(test)]
mod tests {
    // use crate::expression::VariableExpression;

    // use super::*;

    // #[test]
    // fn it_works() {
    //     let mut parser = Parser::new(r#"Yoo, {{ person.name }} man{{fastboi}}{{slow}}"#.as_bytes());
    //     let statements = parser.parse();
    //     for statement in statements {
    //         match statement {
    //             Statement::Expression(expression) => {
    //                 match expression {
    //                     Expression::Variable(variable_expression) => {
    //                         println!("{}", variable_expression.name);
    //                     }
    //                     Expression::TemplateLiteral(template_literal_expression) => {
    //                         println!("{}", template_literal_expression.value);
    //                     }
    //                     Expression::Call(call_expression) => {
    //                         println!("{}", call_expression.name);
    //                     },
    //                     Expression::TemplateLiteral(_) => todo!(),
    //                 }
    //             }
    //         }
    //     }
    //     assert!(false);
    // }
}