use std::cell::RefCell;

use serde_json::{Value};

use crate::{statement::{Statement}, expression::Expression, TokenType};

/// Interprets AST
pub struct Interperter {
    // TODO: can &Value be used?
    context_stack: RefCell<Vec<Value>>
}
// &[u8] is used to avoid cloning
enum ValueOrStr<'a> {
    Value(serde_json::Value),
    Str(&'a [u8])
}

impl<'a> Interperter {
    /// Returns an interperter with given context
    /// 
    /// # Arguments
    /// 
    /// * `context` - the context of the template
    pub fn new(context: Value) -> Self {
        Self {
            context_stack: RefCell::new(vec!(context))
        }
    }
    /// Interprets given statements returning resulting String
    /// 
    /// # Arguments
    ///
    /// * `statements` - Abstract Syntax Tree (AST) Vector to be interpreted
    pub fn interpret(&self, statements: &Vec<Statement>) -> String {
        let mut result = String::new();
        for statement in statements {
            // println!("{:?}", statement);
            result.push_str(&Self::to_string(self.execute(statement)));
        }
        result
    }
    
    /// Converts all ValueOrStr values to string represtation
    /// Value::Null converts to "null"
    /// Value::Number converts to base10 string representation
    fn to_string(value_or_str: ValueOrStr) -> String {
        // TODO: Return &str?
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

    /// Returns the value of the key from current context_stack, starts with top of stack and moves down
    /// returns Value::Null if not found
    /// 
    /// # Arguments
    /// 
    /// * `key` - the key to search for
    fn get(&self, key: &[u8]) -> ValueOrStr {
        for stack in self.context_stack.borrow().iter().rev() {
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
    fn execute(&'a self, statement: &'a Statement) -> ValueOrStr {
        match statement {
            Statement::Expression(expression) => {
                match expression {
                    Expression::Binary(binary_expression) => {
                        let left = self.execute(&binary_expression.left);
                        let right = self.execute(&binary_expression.right);
                        if binary_expression.operator.token_type == TokenType::DoubleEquals {
                            return ValueOrStr::Value(Value::Bool(Self::is_equals(&left, &right)));
                        } else if binary_expression.operator.token_type == TokenType::ExclaimationEqual {
                            return ValueOrStr::Value(Value::Bool(!Self::is_equals(&left, &right)));
                        } else if binary_expression.operator.token_type == TokenType::DoublePipe {
                            if Self::is_truthy(left) || Self::is_truthy(right) {
                                ValueOrStr::Value(Value::Bool(true))
                            } else {
                                ValueOrStr::Value(Value::Bool(false))
                            }
                        } else if binary_expression.operator.token_type == TokenType::DoubleAmpersand {
                            if Self::is_truthy(left) && Self::is_truthy(right) {
                                ValueOrStr::Value(Value::Bool(true))
                            } else {
                                ValueOrStr::Value(Value::Bool(false))
                            }
                        } else {
                            todo!();
                        }
                    }
                    Expression::Unary(unary_expression) => {
                        let value = Self::is_truthy(self.execute(&unary_expression.right));
                        assert_eq!(unary_expression.operator.token_type, TokenType::Exclaimation);
                        ValueOrStr::Value(Value::Bool(!value))
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
                        ValueOrStr::Value(value[std::str::from_utf8(call_expression.name).unwrap()].clone())
                    }
                    Expression::Variable(variable_expression) => {
                        // the value from context_scope
                        self.get(variable_expression.name)
                    }
                    Expression::Literal(literal_expression) => {
                        match literal_expression.token.token_type {
                            TokenType::String => {
                                let value = literal_expression.token.token_value;
                                ValueOrStr::Str(&value[1 .. value.len() - 1])
                            }
                            _ => todo!(),
                        }
                    }
                    Expression::TemplateLiteral(template_literal_expression) => {
                        // just the template literal
                        return ValueOrStr::Str(template_literal_expression.value);
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
                    self.context_stack.borrow_mut().push(serde_json::json!({std::str::from_utf8(for_statement.instance_identifier).unwrap(): i}));
                    // interpret the block for each element in array
                    result.push_str(&self.interpret(&for_statement.statements));
                    self.context_stack.borrow_mut().pop();
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

    /// Returns true if given value is truthy else falsy
    /// Value::Null, Value::Number(0), Value::String(""), Value::Array(array) of len 0, Value::Bool(false), 
    /// str of length 0 are falsy, all other ValueOrStr are truthy
    /// 
    /// # Arguments
    /// 
    /// * `value_or_str` - the value to evaluate
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

    /// Return true if `left` and `right` are equal
    /// implementation unstable (to be changed)
    fn is_equals(left: &ValueOrStr, right: &ValueOrStr) -> bool {
        let left_string = match left {
            ValueOrStr::Str(string) => string,
            ValueOrStr::Value(value) => {
                match value {
                    Value::String(string) => string.as_bytes(),
                    _ => todo!(),
                }
            }
        };
        let right_string = match right {
            ValueOrStr::Str(string) => string,
            ValueOrStr::Value(value) => {
                match value {
                    Value::String(string) => string.as_bytes(),
                    _ => todo!(),
                }
            }
        };
        if left_string == right_string {
            return true;
        }
        false
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
Yoo {{ "here" }} {{ if "yea" && items && fds }}true{{ else }}false{{ end }}
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
        let interperter = Interperter::new(value);
        println!("{}", interperter.interpret(&statements));
        assert!(false);
    }
}