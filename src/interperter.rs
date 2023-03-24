use serde_json::{Value};

use crate::{statement::{Statement}, expression::Expression};

pub struct Interperter {
    context_stack: Vec<Value>
}
enum ValueOrStr<'a> {
    Value(serde_json::Value),
    Str(&'a [u8])
}

impl<'a> Interperter {
    pub fn new(context: Value) -> Self {
        Self {
            context_stack: vec!(context)
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Statement>) -> String {
        let mut result = String::new();
        for statement in statements {
            println!("{:?}", statement);
            result.push_str(&Self::to_string(self.execute(statement)));
        }
        result
    }
    // TODO: Return &str?
    fn to_string(value_or_str: ValueOrStr) -> String {
        match value_or_str {
            ValueOrStr::Value(value) => {
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
            ValueOrStr::Str(value) => String::from_utf8(value.to_vec()).unwrap()
        }
        
    }

    fn get(&self, key: &[u8]) -> ValueOrStr {
        for stack in self.context_stack.iter().rev() {
            let value = &stack[std::str::from_utf8(key).unwrap()];
            if !value.is_null() {
                return ValueOrStr::Value(value.clone());
            }
        }
        ValueOrStr::Value(Value::Null)
    }
    /// Returns a ValurOrStr enum that is a result of executing statement
    /// 
    /// # Arguments
    /// 
    /// * `statement` - An Abstract Syntax Tree (AST) that represents a statement
    fn execute(&'a mut self, statement: &'a Statement) -> ValueOrStr {
        match statement {
            Statement::Expression(expression) => {
                match expression {
                    Expression::Variable(variable_expression) => {
                        // the value from context_scope
                        return self.get(variable_expression.name);
                    }
                    Expression::TemplateLiteral(template_literal_expression) => {
                        // just the template literal
                        return ValueOrStr::Str(template_literal_expression.value);
                    }
                    Expression::Call(call_expression) => {
                        // recurse on callee
                        let value_or_str = self.execute(&call_expression.callee);
                        let value;
                        if let ValueOrStr::Value(i) = value_or_str {
                            value = i;
                        } else {
                            panic!("{:?} is undefined", call_expression.name);
                        }
                        // only objects can be called
                        if !value.is_object() { 
                            panic!("{:?} is undefined", call_expression.name);
                        }
                        return ValueOrStr::Value(value[std::str::from_utf8(call_expression.name).unwrap()].clone());
                    }
                }
            }
            Statement::For(for_statement) => {
                let value_or_str = self.execute(&for_statement.array_variable);
                let array;
                if let ValueOrStr::Value(i) = value_or_str {
                    array = i;
                } else {
                    // only array can be used with for loop
                    panic!("Not array");
                }
                if !array.is_array() {
                    panic!("Not array");
                }
                let mut result = String::new();
                for i in array.as_array().unwrap() {
                    // add current array value to context_scope
                    self.context_stack.push(serde_json::json!({std::str::from_utf8(for_statement.instance_identifier).unwrap(): i}));
                    // interpret the block for each element in array
                    result.push_str(&self.interpret(&for_statement.statements));
                    self.context_stack.pop();
                }
                return ValueOrStr::Value(serde_json::Value::String(result));
            }
            Statement::If(if_statement) => {
                if Self::is_truthy(self.execute(&if_statement.condition)) {
                    ValueOrStr::Value(serde_json::Value::String(self.interpret(&if_statement.if_statements)))
                } else if if_statement.else_statements.len() != 0 {
                    ValueOrStr::Value(serde_json::Value::String(self.interpret(&if_statement.else_statements)))
                } else {
                    ValueOrStr::Value(serde_json::Value::String(String::from("")))
                }
            },
        }
    }

    fn is_truthy(value_or_str: ValueOrStr) -> bool {
        match value_or_str {
            ValueOrStr::Value(value) => {
                match value {
                    Value::Null => false,
                    Value::Bool(boolean) => boolean,
                    Value::Number(number) => if number.as_f64().unwrap() != 0.0 {true} else {false},
                    Value::String(string) => {
                        if string.len() != 0 {true} else {false}
                    }
                    Value::Array(array) => if array.len() != 0 {true} else {false},
                    Value::Object(_) => true,
                }
            }
            ValueOrStr::Str(str) => {
                if str.len() != 0 {true} else {false}
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
//         let source = r#"
// Yoo {{ for i in items }} name: {{ i.name }} {{ end }} | king: {{ person.name }}
//         "#.as_bytes();
        let source = r#"
Yoo {{ if not_Exists }} person exists {{ else }} person does not exist {{ end }}
        "#.as_bytes();
        let binding = Tokenizer::new(source);
        let tokens = binding.tokenize();
        // for token in &tokens {
        //     println!("Token: {:?}", token);
        // }
        let binding = Parser::new(&tokens);
        let statements = binding.parse();
        let value: Value = serde_json::from_str(r#"
{"items": [{"name": "John"}, {"name": "Bob"}, {"name": "Chris"}], "person": {"name": "bob"}}
        "#).unwrap();
//         let value: Value = serde_json::from_str(r#"
// { "name": {"first": "Punit" } }
//          "#).unwrap();
        //println!("{}", value["name"]);
        let mut interperter = Interperter::new(value);
        println!("{}", interperter.interpret(&statements));
        assert!(false);
    }
}