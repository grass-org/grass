use crate::token::{PushTokenCharacterResult, TokenBuilder};
use crate::Token;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct CharacterLiteral {
    pub symbol: char,
    pub escaped: bool,
}

pub(crate) struct CharacterLiteralBuilder {
    symbol: Option<char>,
    escaped: bool,
    closed: bool,
}

impl CharacterLiteralBuilder {
    pub fn start(character: char) -> Option<Self> {
        if character != '\'' {
            return None;
        }

        let builder = Self {
            symbol: None,
            escaped: false,
            closed: false,
        };

        Some(builder)
    }
}

impl TokenBuilder for CharacterLiteralBuilder {
    fn push(&mut self, character: char) -> PushTokenCharacterResult {
        if self.closed {
            return PushTokenCharacterResult::Failed;
        }

        if character == '\'' {
            self.closed = true;
            return PushTokenCharacterResult::Successful;
        }

        if character == '\\' && !self.escaped {
            self.escaped = true;
            return PushTokenCharacterResult::Successful;
        }

        self.symbol = Some(character);
        PushTokenCharacterResult::Successful
    }

    fn build(self) -> Token {
        if !self.closed {
            return Token::Invalid;
        }

        let Some(symbol) = self.symbol else {
            return Token::Invalid;
        };

        let literal = CharacterLiteral {
            symbol,
            escaped: self.escaped,
        };

        Token::CharacterLiteral(literal)
    }
}
