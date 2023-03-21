pub trait Evaluatable {
    fn evaluate(&self) -> Expression;
}

pub enum Expression {
    Call(CallExpression),
    TemplateLiteral(TemplateLiteralExpression),
    Variable(VariableExpression)
}

pub struct CallExpression {
    pub callee: Box<Expression>,
    pub name: String
}

pub struct TemplateLiteralExpression {
    pub value: String
}

pub struct VariableExpression {
    pub name: String
}
