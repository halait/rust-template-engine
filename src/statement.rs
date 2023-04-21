use crate::expression::{self};

/// Represents an AST for a statement
#[derive(Debug)]
pub enum Statement<'a> {
    Expression(expression::Expression<'a>),
    For(ForStatement<'a>),
    If(IfStatement<'a>)
}


/// Represents an AST for for statement
#[derive(Debug)]
pub struct ForStatement<'a> {
    pub instance_identifier: &'a [u8],
    pub array_variable: Box<Statement<'a>>,
    pub statements: Vec<Statement<'a>>
}

/// Represents an AST for if statement
#[derive(Debug)]
pub struct IfStatement<'a> {
    pub condition: Box<Statement<'a>>,
    pub if_statements: Vec<Statement<'a>>,
    pub else_statements: Vec<Statement<'a>>
}