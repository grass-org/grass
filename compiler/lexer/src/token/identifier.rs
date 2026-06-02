use crate::{PushTokenCharacterResult, Token, TokenBuilder};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct Identifier {
    pub symbol: String,
}

#[derive(Debug)]
pub(crate) struct IdentifierBuilder {
    symbol: String,
}

impl IdentifierBuilder {
    pub fn new(start: char) -> Option<Self> {
        if !is_identifier_start(start) {
            return None;
        }

        let mut symbol = String::new();
        symbol.push(start);
        Some(IdentifierBuilder { symbol })
    }
}

impl TokenBuilder for IdentifierBuilder {
    fn push(&mut self, character: char) -> PushTokenCharacterResult {
        if !is_identifier_character(character) {
            return PushTokenCharacterResult::Failed;
        }

        self.symbol.push(character);
        PushTokenCharacterResult::Successful
    }

    fn build(self) -> Token {
        let symbol = self.symbol;
        Token::Identifier(Identifier { symbol })
    }
}

const fn is_identifier_start(character: char) -> bool {
    character.is_ascii_alphabetic()
}

const fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric()
}
