use crate::{Literal, PushTokenCharacterResult, Token, TokenBuilder};

use super::LiteralKind;

#[derive(Debug)]
pub(crate) struct StringLiteralBuilder {
    symbol: String,
    closed: bool,
}

impl StringLiteralBuilder {
    pub fn new(start: char) -> Option<Self> {
        if start != '"' {
            return None;
        }

        let mut symbol = String::new();
        symbol.push(start);

        let builder = Self {
            symbol,
            closed: false,
        };

        Some(builder)
    }
}

impl TokenBuilder for StringLiteralBuilder {
    fn push(&mut self, character: char) -> PushTokenCharacterResult {
        if self.closed {
            return PushTokenCharacterResult::Failed;
        }

        if character == '"' {
            self.closed = true;
        }

        self.symbol.push(character);
        PushTokenCharacterResult::Successful
    }

    fn build(self) -> Token {
        if !self.closed {
            return Token::Invalid;
        }

        // SAFETY: We checked every character we pushed to `symbol` via `push`.
        // We also kept track if it's properly closed with `closed`,
        // which was updated on push.
        let literal = unsafe { Literal::new_unchecked(LiteralKind::String, self.symbol) };
        Token::Literal(literal)
    }
}

pub(super) fn is_valid_string(symbol: &str) -> bool {
    for (index, character) in symbol.chars().enumerate() {
        if (index == 0 || index == symbol.len() - 1) && character != '"' {
            return false;
        }

        // middle symbol cannot be `"`
        if character == '"' {
            return false;
        }
    }

    true
}
