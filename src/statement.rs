use crate::expression::{self, VariableExpression, CallExpression, Expression};


#[derive(Debug)]
pub enum Statement {
    Expression(expression::Expression),
    For(ForStatement),
}

#[derive(Debug)]
pub struct ForStatement {
    pub instance_identifier: Expression,
    pub array_variable: Expression,
    pub statements: Vec<Statement>
}