use serde_json::{Value};

use crate::{statement::{Statement}, expression::Expression};

pub struct Interperter {
    pub context: Value,
    pub result: String
}

impl Interperter {
    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.result.push_str(self.execute(statement).as_str().unwrap());
        }
    }

    fn execute(&self, statement: Statement) -> Value {
        match statement {
            Statement::Expression(expression) => {
                match expression {
                    Expression::Variable(variable_expression) => {
                        return self.context[variable_expression.name].clone();
                    }
                    Expression::TemplateLiteral(template_literal_expression) => {
                        return serde_json::Value::String(template_literal_expression.value);
                    }
                    Expression::Call(call_expression) => {
                        let value = self.execute(Statement::Expression(*call_expression.callee));
                        // if !value.is_object() {
                        //     panic!("{:?} is undefined", call_expression.name);
                        // }
                        return value[call_expression.name].clone();
                    }
                }
            }
        }
        Value::Null
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, interperter};

    use super::*;

    #[test]
    fn it_works() {
        let mut statements = Parser::new(r#"Yoo {{ person.name.first }}"#.as_bytes()).parse();
        let value: Value = serde_json::from_str(r#"{ "person" : { "name" : {"first": "Punit"} } }"#).unwrap();
        //println!("{}", value["name"]);
        let mut interperter = Interperter {context: value, result: String::new()};
        let result = interperter.interpret(statements);
        println!("{}", interperter.result);
        assert!(false);
    }
}