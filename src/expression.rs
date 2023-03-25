use crate::{statement::Statement, Token};

pub trait Evaluatable {
    fn evaluate(&self) -> Expression;
}

#[derive(Debug)]
pub enum Expression<'a> {
    Call(CallExpression<'a>),
    TemplateLiteral(TemplateLiteralExpression<'a>),
    Variable(VariableExpression<'a>),
    Unary(UnaryExpression<'a>),
    Binary(BinaryExpression<'a>),
    Literal(LiteralExpression<'a>)
}

#[derive(Debug)]
pub struct UnaryExpression<'a> {
    pub operator: Token<'a>,
    pub right: Box<Statement<'a>>
}

#[derive(Debug)]
pub struct CallExpression<'a> {
    pub callee: Box<Statement<'a>>,
    pub name: &'a [u8]
}

#[derive(Debug)]
pub struct BinaryExpression<'a> {
    pub left: Box<Statement<'a>>,
    pub operator: &'a Token<'a>,
    pub right: Box<Statement<'a>>
}

#[derive(Debug)]
pub struct TemplateLiteralExpression<'a> {
    pub value: &'a [u8]
}
#[derive(Debug)]
pub struct VariableExpression<'a> {
    pub name: &'a [u8]
}

#[derive(Debug)]
pub struct LiteralExpression<'a> {
    pub token: Token<'a>
}
