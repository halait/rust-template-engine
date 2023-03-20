use crate::{Token, TokenType, tokenizer::{Tokenizer}};

struct Parser<'a> {
    // source: &'a [u8]
    tokenizer: Tokenizer<'a>
}
impl Parser<'_> {
    pub fn new(source: &[u8]) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(source)
        }
    }

    /*
    statement = template_literal
                | expression
                | compound_expression
    */
    fn parse(&mut self) {
        loop {
            // match self.tokenizer.get_token(){
            //     None => {

            //     }
            // }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let expected_result = vec!(
    //     );
    // }
}