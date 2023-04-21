use crate::{statement::Statement, Token};

// pub trait Evaluatable {
//     fn evaluate(&self) -> Expression;
// }

/// Represents an AST for an expression
#[derive(Debug)]
pub enum Expression<'a> {
    Call(CallExpression<'a>),
    TemplateLiteral(TemplateLiteralExpression<'a>),
    Variable(VariableExpression<'a>),
    Unary(UnaryExpression<'a>),
    Binary(BinaryExpression<'a>),
    Literal(LiteralExpression<'a>)
}

/// Represents an AST for an unary expression
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

/// Represents an AST for a binary expression
#[derive(Debug)]
pub struct BinaryExpression<'a> {
    pub left: Box<Statement<'a>>,
    pub operator: &'a Token<'a>,
    pub right: Box<Statement<'a>>
}

/// Represents an AST for a template literal expression
#[derive(Debug)]
pub struct TemplateLiteralExpression<'a> {
    pub value: &'a [u8]
}

/// Represents an AST for a variable expression
#[derive(Debug)]
pub struct VariableExpression<'a> {
    pub name: &'a [u8]
}

/// Represents an AST for a literal expression
#[derive(Debug)]
pub struct LiteralExpression<'a> {
    pub token: Token<'a>
}
