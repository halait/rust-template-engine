use std::{cell::RefCell};

use crate::{Token, TokenType, statement::{Statement, self}, expression::{Expression, self, UnaryExpression, BinaryExpression}};

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    i: RefCell<usize>
}

impl<'a, 'b> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, i: RefCell::new(0) }
    }

    fn next_token(&self) -> Option<&Token> {
        let old = *self.i.borrow();
        self.i.replace(old + 1);
        println!("Next token: {:?}", self.current_token());
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

    fn is_on(&self, token_type: TokenType) -> bool {
        let token = self.current_token().expect(&format!("Unexpected end of input, expected more tokens"));
        if token.token_type != token_type {
            return false;
        }
        true
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
                    TokenType::For => Some(self.parse_for()),
                    TokenType::If => Some(self.parse_if()),
                    // can this be more elegant? does not fit in grammar rules
                    TokenType::End => None,
                    TokenType::Else => None,
                    _ => {
                        let statement = Some(Statement::Expression(self.parse_expression()));
                        self.expect(TokenType::RightDoubleBrackets);
                        statement
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
        self.parse_equality()
    }

    fn parse_equality(&self) -> Expression {
        let mut left = self.parse_unary();
        while self.is_on(TokenType::DoubleEquals) || self.is_on(TokenType::ExclaimationEqual) {
            let operator = self.current_token().unwrap();
            self.next_token();
            let right = Box::new(Statement::Expression(self.parse_unary()));
            left = Expression::Binary(BinaryExpression {
                left: Box::new(Statement::Expression(left)),
                operator,
                right
            });
        }
        left
    }

    fn parse_unary(&self) -> Expression {
        if self.is_on(TokenType::Exclaimation) {
            let operator = self.current_token().unwrap().clone();
            self.next_token();
            return Expression::Unary(UnaryExpression {
                operator,
                right: Box::new(Statement::Expression(self.parse_call()))
            });
        }
        self.parse_call()
    }

    fn parse_call(&self) -> Expression {
        if self.is_on(TokenType::Identifier) {
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
            return expression;
        }
        self.parse_literal()
    }

    fn parse_identifier(&self) -> Expression {
        self.on(TokenType::Identifier);
        let expression = Expression::Variable(expression::VariableExpression {
            name: self.current_token().unwrap().token_value
        });
        self.next_token();
        expression
    }

    fn parse_literal(&self) -> Expression {
        self.on(TokenType::String);
        let token = self.current_token().unwrap().clone();
        self.next_token();
        Expression::Literal(expression::LiteralExpression { token })
    }

    fn parse_for(&self) -> Statement {
        self.expect(TokenType::For);
        self.on(TokenType::Identifier);
        let instance_identifier = self.current_token().unwrap().token_value;
        self.next_token();
        self.expect(TokenType::In);
        self.on(TokenType::Identifier);
        let array_variable = self.parse_call();
        self.expect(TokenType::RightDoubleBrackets);
        let statements = self.parse();
        self.expect(TokenType::End);
        self.expect(TokenType::RightDoubleBrackets);
        Statement::For(statement::ForStatement{
            instance_identifier: instance_identifier,
            array_variable: Box::new(Statement::Expression(array_variable)),
            statements
        })
    }

    fn parse_if(&self) -> Statement {
        self.expect(TokenType::If);
        let condition = Box::new(Statement::Expression(self.parse_expression()));
        self.expect(TokenType::RightDoubleBrackets);
        let if_statements = self.parse();
        let mut else_statements: Vec<Statement> = Vec::new();
        if self.is_on(TokenType::Else) {
            self.next_token();
            self.expect(TokenType::RightDoubleBrackets);
            else_statements = self.parse();
        }
        self.expect(TokenType::End);
        self.expect(TokenType::RightDoubleBrackets);
        Statement::If(statement::IfStatement {
            condition,
            if_statements,
            else_statements
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