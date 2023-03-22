pub trait Evaluatable {
    fn evaluate(&self) -> Expression;
}

#[derive(Debug)]
pub enum Expression {
    Call(CallExpression),
    TemplateLiteral(TemplateLiteralExpression),
    Variable(VariableExpression)
}
#[derive(Debug)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub name: String
}
#[derive(Debug)]
pub struct TemplateLiteralExpression {
    pub value: String
}
#[derive(Debug)]
pub struct VariableExpression {
    pub name: String
}
