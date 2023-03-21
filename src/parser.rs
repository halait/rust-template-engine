use crate::{Token, TokenType, tokenizer::{Tokenizer}, statement::Statement, expression::{Expression, self, CallExpression}};

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

    fn next_token(&mut self) {
        // TODO return last token?
        self.current_token = self.tokenizer.get_token();
        // println!("Next token: {:?}", self.current_token);
        // self.current_token = self.tokenizer.get_token();
        // &self.current_token
    }
    
    // fn current_token(&self) -> &Token {
    //     &self.current_token
    // }

    fn expect(&mut self, token_type: TokenType) {
        if self.current_token.token_type != token_type {
            panic!("Unexpected token: {:?}, is: {:?}", token_type, self.current_token.token_type);
        }
        // &self.next_token()
    }

    pub fn parse(&mut self) -> Vec::<Statement> {
        let mut statements: Vec::<Statement> = Vec::new();
        self.next_token();
        loop {
            if self.current_token.token_type == TokenType::End {
                break
            }
            statements.push(self.statement());
        }
        statements
    }

    fn statement(&mut self) -> Statement {
        Statement::Expression(self.expression())
    }

    fn expression(&mut self) -> Expression {
        match self.current_token.token_type {
            TokenType::LeftDoubleBrackets => {
                self.next_token();
                self.call()
            }
            TokenType::TempalteLiteral => {
                
                let expression = Expression::TemplateLiteral(expression::TemplateLiteralExpression {
                    value: self.current_token.token_value.clone()
                });
                self.next_token();
                expression
            }
            _ => {
                panic!("Not implemented {:?}", self.current_token.token_type);
            }
        }
    }

    fn call(&mut self) -> Expression {
        let mut expression = self.identifier();
        
        while self.current_token.token_type == TokenType::Dot {
            self.next_token();
            self.expect(TokenType::Identifier);
            expression = Expression::Call(expression::CallExpression {
                callee: Box::new(expression),
                name: self.current_token.token_value.clone()
            });
            self.next_token();
        }

        self.expect(TokenType::RightDoubleBrackets);
        self.next_token();

        expression
    }

    fn identifier(&mut self) -> Expression {
        let expression = Expression::Variable(expression::VariableExpression {
            name: self.current_token.token_value.clone()
        });
        self.next_token();
        expression
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