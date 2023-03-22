use crate::expression::{self, VariableExpression, CallExpression, Expression};


#[derive(Debug)]
pub enum Statement {
    Expression(expression::Expression),
    For(),
}

struct ForStatement {
    instance_identifier: VariableExpression,
    array_variable: Expression,
    statements: Vec<Statement>
}