use crate::{Token, TokenType, message_formatter};

pub struct Tokenizer<'a> {
    // current index
    i: usize,
    token_start: usize,
    // source code
    source: &'a [u8],
    // resultant tokens
    // pub tokens: Vec<Token<'a>>,
    token_type_map: std::collections::HashMap<&'a [u8], TokenType>,
    in_curly: bool
}
impl<'a> Tokenizer<'a> {
    pub fn new(source: &[u8]) -> Tokenizer {
        Tokenizer {
            i: 0,
            token_start: 0,
            source: source,
            // tokens: Vec::new(),
            token_type_map: std::collections::HashMap::from([
                ("{{".as_bytes(), TokenType::LeftDoubleBrackets),
                ("}}".as_bytes(), TokenType::RightDoubleBrackets),
                ("for".as_bytes(), TokenType::For),
                ("in".as_bytes(), TokenType::In),
                ("when".as_bytes(), TokenType::When),
                ("if".as_bytes(), TokenType::If),
            ]),
            in_curly: false
        }
    }

    fn get_current(&self) -> Option<u8> {
        if self.i == self.source.len() {
            return None;
        }
        Some(self.source[self.i])
    }

    fn increment(&mut self) -> Option<u8> {
        self.i += 1;
        self.get_current()
    }

    fn is_on(&self, character: u8) -> bool {
        match self.get_current() {
            Some(i) => if i == character {true} else {false}
            None => false
        }
    }

    fn is_next(&self, character: u8) -> bool {
        match self.peek() {
            None => false,
            Some(next_character) => if character == next_character {true} else {false}
        }
    }

    fn get_last_token(&self) -> &[u8] {
        &self.source[self.token_start .. self.i]
    }

    fn tokenize(&self, type_type: TokenType) -> Token {
        Token {
            token_type: type_type,
            token_value: self.get_last_token()
        }
    }

    fn peek(&self) -> Option<u8> {
        let next = self.i + 1;
        if next >= self.source.len() {
            return None;
        }
        Some(self.source[next])
    }

    fn at_curly_start(&self) -> bool {
        if self.is_on(b'{') && self.is_next(b'{') {
            true
        } else {
            false
        }
    }

    fn tokenize_template_string(&mut self) -> Token {
        loop {
            match self.increment() {
                None => {
                    return self.tokenize(TokenType::TempalteLiteral)
                },
                Some(_) => {
                    if self.at_curly_start() {
                        return self.tokenize(TokenType::TempalteLiteral)
                    }
                }
            }
        }
    }

    fn get_symbol_token_type(&self, symbol: &[u8]) -> TokenType {
        self.token_type_map.get(symbol).unwrap_or(&TokenType::Identifier).clone()
    }

    fn tokenize_symbol(&mut self) -> Token {
        loop {
            match self.increment() {
                None => {
                    return self.tokenize(self.get_symbol_token_type(self.get_last_token()));
                },
                Some(character) => {
                    if !character.is_ascii_alphanumeric() {
                        return self.tokenize(self.get_symbol_token_type(self.get_last_token()));
                    }
                }
            }
        }
    }

    fn tokenize_string_literal(&mut self) -> Token {
        loop {
            match self.increment() {
                None => {
                    return self.tokenize(TokenType::StringLiteral);
                }
                Some(character) => {
                    if character != b'\\' && self.is_next(b'"') {
                        // eat quotes
                        self.increment();
                        self.increment();
                        return self.tokenize(TokenType::StringLiteral);
                    }
                }
            }
        }
    }

    pub fn get_token(&mut self) -> Token {
        loop {
            self.token_start = self.i;
            match self.get_current() {
                Some(character) => {
                    if !self.in_curly {
                        if self.at_curly_start() {
                            self.in_curly = true;
                            // skip curly
                            self.increment();
                            self.increment();
                            return self.tokenize(TokenType::LeftDoubleBrackets);
                        } else {
                            return self.tokenize_template_string();
                        }
                    } else {
                        if character.is_ascii_alphabetic() {
                            return self.tokenize_symbol();
                        } else if character == b'}' && self.is_next(b'}') {
                            self.in_curly = false;
                            // skip curly
                            self.increment();
                            self.increment();
                            return self.tokenize(TokenType::RightDoubleBrackets);
                        } else if character == b'"' {
                            return self.tokenize_string_literal();
                        } else if character.is_ascii_whitespace() {
                            self.increment();
                        } else {
                            panic!("{}", message_formatter::format(&self.source, self.i, "Invalid character"));
                        }
                    }
                },
                None => {
                    return self.tokenize(TokenType::END);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tokener = Tokenizer::new(
            // r#"hello, {{ name }} yes man {{ "no man" }}{{ for item in items }}{{ if property"#
            r#"hello, {{      name}} y {{yes}}{{for item in items}}  a {{ "yes" }} {{ if property}}"#.as_bytes()
        );
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::TempalteLiteral, token_value: "hello, ".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::LeftDoubleBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::Identifier, token_value: "name".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::RightDoubleBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::TempalteLiteral, token_value: " y ".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::LeftDoubleBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::Identifier, token_value: "yes".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::RightDoubleBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::LeftDoubleBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::For, token_value: "for".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::Identifier, token_value: "item".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::In, token_value: "in".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::Identifier, token_value: "items".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::RightDoubleBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::TempalteLiteral, token_value: "  a ".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::LeftDoubleBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::StringLiteral, token_value: "\"yes\"".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::RightDoubleBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::TempalteLiteral, token_value: " ".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::LeftDoubleBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::If, token_value: "if".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::Identifier, token_value: "property".as_bytes()});
        assert_eq!(tokener.get_token(), Token{token_type: TokenType::RightDoubleBrackets, token_value: "}}".as_bytes()});
    }
}