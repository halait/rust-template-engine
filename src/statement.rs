use crate::expression::{self};

#[derive(Debug)]
pub enum Statement {
    Expression(expression::Expression),
    For(ForStatement),
}

#[derive(Debug)]
pub struct ForStatement {
    pub instance_identifier: String,
    pub array_variable: Box<Statement>,
    pub statements: Vec<Statement>
}