use serde_json::{Value};

use crate::{statement::{Statement}, expression::Expression};

pub struct Interperter {
    pub context_stack: Vec<Value>
}

impl Interperter {
    pub fn new(context: Value) -> Self {
        Self {
            context_stack: vec!(context)
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Statement>) -> String {
        let mut result = String::new();
        for statement in statements {
            println!("{:?}", statement);
            result.push_str(&Interperter::to_string(self.execute(statement)));
        }
        result
    }

    fn to_string(value: Value) -> String{
        if value.is_string() {
            value.as_str().unwrap().to_owned()
        } else if value.is_number() {
            value.as_f64().unwrap().to_string()
        } else if value.is_null() {
            "null".to_string()
        } else {
            todo!("{:?}", value);
        }
    }

    fn get(&self, key: &str) -> Value {
        for stack in self.context_stack.iter().rev() {
            let value = stack[key].clone();
            if !value.is_null() {
                return value;
            }
        }
        serde_json::Value::Null
    }

    fn execute(&mut self, statement: &Statement) -> Value {
        match statement {
            Statement::Expression(expression) => {
                match expression {
                    Expression::Variable(variable_expression) => {
                        return self.get(&variable_expression.name);
                    }
                    Expression::TemplateLiteral(template_literal_expression) => {
                        return serde_json::Value::String(template_literal_expression.value.clone());
                    }
                    Expression::Call(call_expression) => {
                        let value = self.execute(&call_expression.callee);
                        if !value.is_object() {
                            panic!("{:?} is undefined", call_expression.name);
                        }
                        return value[&call_expression.name].clone();
                    }
                }
            }
            Statement::For(for_statement) => {
                let array = self.execute(&for_statement.array_variable);
                if !array.is_array() {
                    panic!("Not array");
                }
                let mut result = String::new();
                for i in array.as_array().unwrap() {
                    self.context_stack.push(serde_json::json!({for_statement.instance_identifier.clone(): i}));
                    result.push_str(&self.interpret(&for_statement.statements));
                    self.context_stack.pop();
                }
                return serde_json::Value::String(result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, tokenizer::{Tokenizer}};

    use super::*;

    #[test]
    fn it_works() {
        let mut tokenizer = Tokenizer::new(r#"
Yoo {{ for i in items }} name: {{ i.name }} {{ end }} | king: {{ person.name }}
        "#.as_bytes());
        // let mut tokenizer = Tokenizer::new(r#"Yoo {{ person.name }}"#.as_bytes());
        let mut parser = Parser::new();
        parser.init(&mut tokenizer);
        let statements = parser.parse(&mut tokenizer);
        let value: Value = serde_json::from_str(r#"
{"items": [{"name": "John"}, {"name": "Bob"}, {"name": "Chris"}], "person": {"name": "bob"}}
        "#).unwrap();
        //println!("{}", value["name"]);
        let mut interperter = Interperter::new(value);
        println!("{}", interperter.interpret(&statements));
        assert!(false);
    }
}