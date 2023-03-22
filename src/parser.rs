use crate::{Token, TokenType, tokenizer::{Tokenizer}, statement::Statement, expression::{Expression, self}};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token
}
impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            tokenizer: Tokenizer::new(source),
            current_token: Token {
                token_type: TokenType::Begin,
                token_value: "".to_string()
            }
        }
    }

    // pub fn init(&mut self) {
    //     self.current_token = self.tokenizer.get_token();
    // }

    fn next_token(&mut self) -> &Token {
        self.current_token = self.tokenizer.get_token();
        println!("Next token: {:?}", self.current_token);
        &self.current_token
    }

    fn expect(&mut self, token_type: TokenType) {
        self.on(token_type);
        self.next_token();
    }

    fn on(&mut self, token_type: TokenType) {
        if self.current_token.token_type != token_type {
            panic!("Unexpected token: {:?}, is: {:?}", token_type, self.current_token.token_type);
        }
    }

    pub fn parse(&mut self) -> Vec::<Statement> {
        let mut statements: Vec::<Statement> = Vec::new();
        self.next_token();
        loop {
            if self.current_token.token_type == TokenType::End {
                break
            }
            statements.push(self.parse_statement());
        }
        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token.token_type {
            TokenType::LeftDoubleBrackets => {
                match self.next_token().token_type {
                    TokenType::Identifier => {
                        Statement::Expression(self.parse_expression())
                    }
                    TokenType::For => {
                        self.parse_for()
                    }
                    TokenType::Begin => todo!(),
                    TokenType::LeftDoubleBrackets => todo!(),
                    TokenType::RightDoubleBrackets => todo!(),
                    TokenType::For => todo!(),
                    TokenType::If => todo!(),
                    TokenType::In => todo!(),
                    TokenType::When => todo!(),
                    TokenType::String => todo!(),
                    TokenType::TempalteLiteral => todo!(),
                    TokenType::Dot => todo!(),
                    TokenType::End => todo!(),
                }
            }
            TokenType::TempalteLiteral => {
                let expression = Expression::TemplateLiteral(expression::TemplateLiteralExpression {
                    value: self.current_token.token_value.clone()
                });
                self.next_token();
                Statement::Expression(expression)
            }
            _ => {
                todo!()
            }
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_call()
    }

    fn parse_call(&mut self) -> Expression {
        let mut expression = self.parse_identifier();
        
        while self.current_token.token_type == TokenType::Dot {
            self.next_token();
            self.on(TokenType::Identifier);
            expression = Expression::Call(expression::CallExpression {
                callee: Box::new(expression),
                name: self.current_token.token_value.clone()
            });
            self.next_token();
        }

        self.expect(TokenType::RightDoubleBrackets);

        expression
    }

    fn parse_identifier(&mut self) -> Expression {
        let expression = Expression::Variable(expression::VariableExpression {
            name: self.current_token.token_value.clone()
        });
        self.next_token();
        expression
    }

    fn parse_for(&mut self) -> Statement {
        self.on(TokenType::Identifier);
        let instance_identifier = self.current_token.token_value;
        self.expect(TokenType::In);
        self.on(TokenType::Identifier);
        let array_variable = self.parse_call();
        let statements = self.parse
        

    }
}


#[cfg(test)]
mod tests {
    use crate::expression::VariableExpression;

    use super::*;

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