use std::{cell::RefCell};

use crate::{Token, TokenType, statement::{Statement, self}, expression::{Expression, self, UnaryExpression, BinaryExpression}};

/// Creates AST with given tokens
pub struct Parser<'a> {
    // TODO: do not hold refrence, take as argument?
    tokens: &'a Vec<Token<'a>>,
    i: RefCell<usize>
}

impl<'a, 'b> Parser<'a> {
    /// Returns parser, to be used to create AST from `tokens`
    /// 
    /// # Arguments
    /// `tokens` - tokens to be parsed
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, i: RefCell::new(0) }
    }

    /// Increments parser to next token and returns it
    fn next_token(&self) -> Option<&Token> {
        let old = *self.i.borrow();
        self.i.replace(old + 1);
        // println!("Next token: {:?}", self.current_token());
        self.current_token()
    }

    fn current_token(&self) -> Option<&Token> {
        let i = self.i.borrow();
        if self.tokens.len() <= *i { None } else { Some(&self.tokens[*i]) }
    }

    /// Checks if currently on given `TokenType`, increments parser if yes, panics if not
    /// 
    /// # Arguments
    /// 
    /// * `token_type` - expected current `TokenType`
    fn expect(&self, token_type: TokenType) {
        self.on(token_type);
        self.next_token();
    }

    /// Checks if currently on given `TokenType`, panics if not
    /// 
    /// # Arguments
    /// 
    /// * `token_type` - expected current `TokenType`
    fn on(&self, token_type: TokenType) {
        let token = self.current_token().expect(&format!("Unexpected end of input, expected token: {:?}", token_type));
        if token.token_type != token_type {
            panic!("Unexpected token: {:?}, expected: {:?}", token.token_type, token_type);
        }
    }

    /// Returns true if parser is currently on given `TokenType`, else false
    /// 
    /// # Arguments
    /// 
    /// * `token_type` - the `TokenType` to compare to
    fn is_on(&self, token_type: TokenType) -> bool {
        let token = self.current_token().expect(&format!("Unexpected end of input, expected more tokens"));
        if token.token_type != token_type {
            return false;
        }
        true
    }

    /// Parses tokens refrenceced by instance, returns `Vec::<Statement>` that represent a series of ASTs
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

    /// Parse starting at current token, convert to AST, Option::None if at end of lexical statement  
    fn parse_statement(&self) -> Option<Statement> {
        match self.current_token()?.token_type {
            TokenType::DoubleLeftBrackets => {
                match self.next_token()?.token_type {
                    TokenType::For => Some(self.parse_for()),
                    TokenType::If => Some(self.parse_if()),
                    // can this be more elegant? does not fit in grammar rules
                    TokenType::End => None,
                    TokenType::Else => None,
                    _ => {
                        let statement = Some(Statement::Expression(self.parse_expression()));
                        self.expect(TokenType::DoubleRightBrackets);
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

    /// Parse expression starting at current token
    fn parse_expression(&self) -> Expression {
        self.parse_or()
    }

    /// Parse or expression starting at current token
    fn parse_or(&self) -> Expression {
        let mut left = self.parse_and();
        while self.is_on(TokenType::DoublePipe) {
            let operator = self.current_token().unwrap();
            self.next_token();
            let right = Box::new(Statement::Expression(self.parse_and()));
            left = Expression::Binary(BinaryExpression {
                left: Box::new(Statement::Expression(left)),
                operator,
                right
            });
        }
        left
    }

    /// Parse and expression starting at current token
    fn parse_and(&self) -> Expression {
        let mut left = self.parse_equality();
        while self.is_on(TokenType::DoubleAmpersand) {
            let operator = self.current_token().unwrap();
            self.next_token();
            let right = Box::new(Statement::Expression(self.parse_equality()));
            left = Expression::Binary(BinaryExpression {
                left: Box::new(Statement::Expression(left)),
                operator,
                right
            });
        }
        left
    }

    /// Parse equality expression starting at current token
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

    /// Parse unary expression starting at current token
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

    /// Parse call expression starting at current token
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

    /// Parse identifier expression starting at current token
    fn parse_identifier(&self) -> Expression {
        self.on(TokenType::Identifier);
        let expression = Expression::Variable(expression::VariableExpression {
            name: self.current_token().unwrap().token_value
        });
        self.next_token();
        expression
    }

    /// Parse literal expression starting at current token, only works for string literal currently
    fn parse_literal(&self) -> Expression {
        self.on(TokenType::String);
        let token = self.current_token().unwrap().clone();
        self.next_token();
        Expression::Literal(expression::LiteralExpression { token })
    }

    /// Parse for statement starting at current token, only works for string literal currently
    fn parse_for(&self) -> Statement {
        self.expect(TokenType::For);
        self.on(TokenType::Identifier);
        let instance_identifier = self.current_token().unwrap().token_value;
        self.next_token();
        self.expect(TokenType::In);
        self.on(TokenType::Identifier);
        let array_variable = self.parse_call();
        self.expect(TokenType::DoubleRightBrackets);
        let statements = self.parse();
        self.expect(TokenType::End);
        self.expect(TokenType::DoubleRightBrackets);
        Statement::For(statement::ForStatement{
            instance_identifier: instance_identifier,
            array_variable: Box::new(Statement::Expression(array_variable)),
            statements
        })
    }

    /// Parse if statement starting at current token, only works for string literal currently
    fn parse_if(&self) -> Statement {
        self.expect(TokenType::If);
        let condition = Box::new(Statement::Expression(self.parse_expression()));
        self.expect(TokenType::DoubleRightBrackets);
        let if_statements = self.parse();
        let mut else_statements: Vec<Statement> = Vec::new();
        if self.is_on(TokenType::Else) {
            self.next_token();
            self.expect(TokenType::DoubleRightBrackets);
            else_statements = self.parse();
        }
        self.expect(TokenType::End);
        self.expect(TokenType::DoubleRightBrackets);
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