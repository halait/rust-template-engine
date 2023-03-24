use crate::statement::Statement;

pub trait Evaluatable {
    fn evaluate(&self) -> Expression;
}

#[derive(Debug)]
pub enum Expression<'a> {
    Call(CallExpression<'a>),
    TemplateLiteral(TemplateLiteralExpression<'a>),
    Variable(VariableExpression<'a>)
}
#[derive(Debug)]
pub struct CallExpression<'a> {
    pub callee: Box<Statement<'a>>,
    pub name: &'a [u8]
}
#[derive(Debug)]
pub struct TemplateLiteralExpression<'a> {
    pub value: &'a [u8]
}
#[derive(Debug)]
pub struct VariableExpression<'a> {
    pub name: &'a [u8]
}
