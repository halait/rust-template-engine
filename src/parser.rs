use crate::{Token, TokenType, tokenizer::{Tokenizer}, statement::{Statement, self}, expression::{Expression, self}};

pub struct Parser {
    current_token: Option<Token>
}
impl Parser {
    pub fn new() -> Self {
        Parser { current_token: None }
    }
    
    pub fn init(&mut self, tokenizer: &mut Tokenizer) {
        self.next_token(tokenizer);
    }

    fn next_token(&mut self, tokenizer: &mut Tokenizer) -> &Option<Token> {
        self.current_token = tokenizer.next();
        println!("Next token: {:?}", self.current_token);
        &self.current_token
    }

    fn expect(&mut self, token_type: TokenType, tokenizer: &mut Tokenizer) {
        self.on(token_type);
        self.next_token(tokenizer);
    }

    fn on(&mut self, token_type: TokenType) {
        if self.current_token == None {
            panic!("Unexpected end of input, expected token: {:?}, ", token_type);
        }
        if self.current_token.as_ref().unwrap().token_type != token_type {
            panic!("Unexpected token: {:?}, expected: {:?}", self.current_token.as_ref().unwrap().token_type, token_type);
        }
    }

    pub fn parse(&mut self, tokenizer: &mut Tokenizer) -> Vec::<Statement> {
        let mut statements: Vec::<Statement> = Vec::new();
        loop {
            if self.current_token == None {
                return statements;
            }
            match self.parse_statement(tokenizer) {
                Some(statement) => {
                    statements.push(statement);
                }
                None => {
                    return statements
                }
            }
        }
    }

    fn parse_statement(&mut self, tokenizer: &mut Tokenizer) -> Option<Statement> {
        match self.current_token.as_ref()?.token_type {
            TokenType::LeftDoubleBrackets => {
                match self.next_token(tokenizer).as_ref().expect("Unexpected end of input").token_type {
                    TokenType::Identifier => {
                        Some(Statement::Expression(self.parse_expression(tokenizer)))
                    }
                    TokenType::For => {
                        Some(self.parse_for(tokenizer))
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
                    value: self.current_token.as_ref().expect("Unexpected end of input").token_value.clone()
                });
                self.next_token(tokenizer);
                Some(Statement::Expression(expression))
            }
            _ => {
                todo!("{:?}", self.current_token);
            }
        }
    }

    fn parse_expression(&mut self, tokenizer: &mut Tokenizer) -> Expression {
        self.parse_call(tokenizer)
    }

    fn parse_call(&mut self, tokenizer: &mut Tokenizer) -> Expression {
        let mut expression = self.parse_identifier(tokenizer);
        
        while self.current_token.as_ref().expect("Unexpected end of input").token_type == TokenType::Dot {
            self.next_token(tokenizer);
            self.on(TokenType::Identifier);
            expression = Expression::Call(expression::CallExpression {
                callee: Box::new(Statement::Expression(expression)),
                name: self.current_token.as_ref().unwrap().token_value.clone()
            });
            self.next_token(tokenizer);
        }
        self.expect(TokenType::RightDoubleBrackets, tokenizer);
        expression
    }

    fn parse_identifier(&mut self, tokenizer: &mut Tokenizer) -> Expression {
        let expression = Expression::Variable(expression::VariableExpression {
            name: self.current_token.as_ref().unwrap().token_value.clone()
        });
        self.next_token(tokenizer);
        expression
    }

    fn parse_for(&mut self, tokenizer: &mut Tokenizer) -> Statement {
        self.expect(TokenType::For, tokenizer);
        self.on(TokenType::Identifier);
        let instance_identifier = self.current_token.as_ref().unwrap().token_value.clone();
        self.next_token(tokenizer);
        self.expect(TokenType::In, tokenizer);
        self.on(TokenType::Identifier);
        let array_variable = self.parse_call(tokenizer);
        let statements = self.parse(tokenizer);
        self.expect(TokenType::End, tokenizer);
        self.expect(TokenType::RightDoubleBrackets, tokenizer);
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