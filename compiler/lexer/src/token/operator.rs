use crate::token::{PushTokenCharacterResult, TokenBuilder};
use crate::Token;

#[derive(Debug)]
pub(crate) struct OperatorBuilder {
    symbol: String,
}

impl OperatorBuilder {
    pub fn new(start: char) -> Option<Self> {
        if !is_operator_character(start) {
            return None;
        }

        let mut symbol = String::new();
        symbol.push(start);

        Some(OperatorBuilder { symbol })
    }
}

impl TokenBuilder for OperatorBuilder {
    fn push(&mut self, character: char) -> PushTokenCharacterResult {
        if !is_operator_character(character) {
            return PushTokenCharacterResult::Failed;
        }

        self.symbol.push(character);
        PushTokenCharacterResult::Successful
    }

    fn build(self) -> Token {
        Token::Operator(self.symbol)
    }
}

fn is_operator_character(character: char) -> bool {
    match character {
        '(' | ')' | '[' | ']' | '{' | '}' | ',' | ':' => false,
        _ => !character.is_ascii_alphanumeric() && !character.is_whitespace(),
    }
}
