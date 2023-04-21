use std::cell::RefCell;

use crate::{Token, TokenType, message_formatter};

/// Tokenizes given source code
pub struct Tokenizer<'a> {
    /// current index
    i: RefCell<usize>,
    token_start: RefCell<usize>,
    /// source code
    source: &'a [u8],
    alphabetic_token_map: std::collections::HashMap<&'static [u8], TokenType>,
    in_curly: RefCell<bool>
}

impl<'a> Tokenizer<'a> {
    const TOKEN_MAP: [(&[u8], TokenType); 8] = [
        ("{{".as_bytes(), TokenType::DoubleLeftBrackets),
        ("}}".as_bytes(), TokenType::DoubleRightBrackets),
        (".".as_bytes(), TokenType::Dot),
        ("==".as_bytes(), TokenType::DoubleEquals),
        ("!=".as_bytes(), TokenType::ExclaimationEqual),
        ("!".as_bytes(), TokenType::Exclaimation),
        ("&&".as_bytes(), TokenType::DoubleAmpersand),
        ("||".as_bytes(), TokenType::DoublePipe)
    ];

    /// Returns a tokenizer used to tokenize `source`
    /// 
    /// # Arguments
    /// 
    /// * `source` - the text to tokenize
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            i: RefCell::new(0),
            token_start: RefCell::new(0),
            source: source,
            alphabetic_token_map: std::collections::HashMap::from([
                ("for".as_bytes(), TokenType::For),
                ("in".as_bytes(), TokenType::In),
                ("when".as_bytes(), TokenType::When),
                ("if".as_bytes(), TokenType::If),
                ("else".as_bytes(), TokenType::Else),
                ("end".as_bytes(), TokenType::End),
            ]),
            in_curly: RefCell::new(false)
        }
    }

    /// Returns tokens of refrenced text
    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut token = self.next();
        while token != None {
            tokens.push(token.unwrap());
            token = self.next();
        }
        tokens
    }

    /// Returns `Option::Some` with current `u8`, if at EOF returns `Option::None`
    fn get_current(&self) -> Option<u8> {
        if *self.i.borrow() == self.source.len() {
            return None;
        }
        Some(self.source[*self.i.borrow()])
    }

    /// Increments parser to next `u8` and retuens Option::Some with it, if at EOF returns `Option::None`
    fn increment(&self) -> Option<u8> {
        let new = *self.i.borrow() + 1;
        self.i.replace(new);
        self.get_current()
    }
    
    /// Returns true if currently on `characters` starting at current index else false
    /// 
    /// # Arguments
    /// 
    /// * `characters` - the characters to check for
    fn is_on(&self, characters: &[u8]) -> bool {
        let mut i = self.i.borrow().clone();
        for character in characters {
            if i >= self.source.len() {
                return false;
            }
            if self.source[i] != *character {
                return false;
            }
            i += 1;
        }
        return true;
    }

    /// Returns true if last index in parser was equal to `character`
    /// 
    /// # Arguements
    /// 
    /// * `character` - the character to be checked
    fn is_previous(&self, character: u8) -> bool {
        let current = *self.i.borrow();
        if current == 0 {
            false
        } else if self.source[current - 1] == character {
            true
        } else {
            false
        }
    }
    
    /// Returns text of token parser is currently on, ends at current index
    fn get_last_token(&self) -> &[u8] {
        &self.source[*self.token_start.borrow() .. *self.i.borrow()]
    }

    /// Returns token parser is currently on, ends at current index
    /// 
    /// # Arguments
    /// * `token_type` - the type of the `Token` to be returned
    fn tokenize_last(&self, type_type: TokenType) -> Token {
        Token {
            token_type: type_type,
            token_value: self.get_last_token()
        }
    }
    
    /// Tokenizes template string parser is currently on
    fn tokenize_template_string(&self) -> Token {
        loop {
            match self.increment() {
                None => {
                    return self.tokenize_last(TokenType::TempalteLiteral)
                },
                Some(_) => {
                    if self.is_on("{{".as_bytes()) {
                        return self.tokenize_last(TokenType::TempalteLiteral)
                    }
                }
            }
        }
    }

    /// Gets type of token represented by `symbol`, must be alphabetic
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - alphabetic symbol to get `TokenType` for
    fn get_symbol_token_type(&self, symbol: &[u8]) -> TokenType {
        self.alphabetic_token_map.get(symbol).unwrap_or(&TokenType::Identifier).clone()
    }

    /// Tokenizes alphabetic symbol parser is currently on
    fn tokenize_symbol(&self) -> Token {
        loop {
            match self.increment() {
                None => {
                    return self.tokenize_last(self.get_symbol_token_type(self.get_last_token()));
                },
                Some(character) => {
                    if character != b'_' && !character.is_ascii_alphanumeric() {
                        return self.tokenize_last(self.get_symbol_token_type(self.get_last_token()));
                    }
                }
            }
        }
    }

    /// Tokenizes string literal parser is currently on, double quotes (") in string can be escaped with backslash (\)
    fn tokenize_string_literal(&self) -> Token {
        loop {
            match self.increment() {
                None => {
                    panic!("Unexpected end of input");
                }
                Some(character) => {
                    if !self.is_previous(b'\\') && character == b'"' {
                        // eat quotes
                        self.increment();
                        return self.tokenize_last(TokenType::String);
                    }
                }
            }
        }
    }

    /// Returns Option::Some with next token in refrenced `[u8]`, or Option::None if at end
    pub fn next(&self) -> Option<Token> {
        loop {
            self.token_start.replace(*self.i.borrow());
            match self.get_current() {
                Some(character) => {
                    if !*self.in_curly.borrow() {
                        if self.is_on("{{".as_bytes()) {
                            self.in_curly.replace(true);
                            // skip curly
                            self.increment();
                            self.increment();
                            return Some(self.tokenize_last(TokenType::DoubleLeftBrackets));
                        } else {
                            return Some(self.tokenize_template_string());
                        }
                    } else {
                        if character.is_ascii_alphabetic() {
                            return Some(self.tokenize_symbol());
                        } else if character == b'"' {
                            return Some(self.tokenize_string_literal());
                        } else if character.is_ascii_whitespace() {
                            self.increment();
                        } else {
                            for entry in Self::TOKEN_MAP {
                                if self.is_on(entry.0) {
                                    let new = *self.i.borrow() + entry.0.len();
                                    self.i.replace(new);
                                    if entry.1 == TokenType::DoubleRightBrackets {
                                        self.in_curly.replace(false);
                                    }
                                    return Some(self.tokenize_last(entry.1));
                                }
                            }
                            panic!("{}", message_formatter::format(&self.source, *self.i.borrow(), "Invalid character"));
                        }
                    }
                },
                None => {
                    return None;
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
        let tokener = Tokenizer::new(
            // r#"hello, {{ name }} yes man {{ "no man" }}{{ for item in items }}{{ if property"#
            r#"hello, {{    person.name}} y {{yes}}{{for item in items}}  a {{ "yes" }} {{ if property == "yes" && property || property }}"#.as_bytes()
        );
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::TempalteLiteral, token_value: "hello, ".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleLeftBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Identifier, token_value: "person".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Dot, token_value: ".".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Identifier, token_value: "name".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleRightBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::TempalteLiteral, token_value: " y ".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleLeftBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Identifier, token_value: "yes".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleRightBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleLeftBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::For, token_value: "for".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Identifier, token_value: "item".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::In, token_value: "in".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Identifier, token_value: "items".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleRightBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::TempalteLiteral, token_value: "  a ".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleLeftBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::String, token_value: "\"yes\"".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleRightBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::TempalteLiteral, token_value: " ".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleLeftBrackets, token_value: "{{".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::If, token_value: "if".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Identifier, token_value: "property".as_bytes()});

        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleEquals, token_value: "==".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::String, token_value: "\"yes\"".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleAmpersand, token_value: "&&".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Identifier, token_value: "property".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoublePipe, token_value: "||".as_bytes()});
        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::Identifier, token_value: "property".as_bytes()});

        assert_eq!(tokener.next().unwrap(), Token{token_type: TokenType::DoubleRightBrackets, token_value: "}}".as_bytes()});
        assert_eq!(tokener.next(), None);
    }
}