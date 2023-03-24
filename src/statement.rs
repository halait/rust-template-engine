use crate::expression::{self};

#[derive(Debug)]
pub enum Statement<'a> {
    Expression(expression::Expression<'a>),
    For(ForStatement<'a>),
}

#[derive(Debug)]
pub struct ForStatement<'a> {
    pub instance_identifier: &'a [u8],
    pub array_variable: Box<Statement<'a>>,
    pub statements: Vec<Statement<'a>>
}