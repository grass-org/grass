use crate::{PushTokenCharacterResult, Token, TokenBuilder};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct Identifier {
    symbol: String,
}

impl Identifier {
    /// # Safety
    ///
    /// The caller must guarantee that `symbol` is valid
    pub const unsafe fn new_unchecked(symbol: String) -> Identifier {
        Identifier { symbol }
    }

    pub fn new(symbol: impl Into<String>) -> Option<Identifier> {
        let symbol = symbol.into();

        if !is_identifier_symbol(&symbol) {
            return None;
        }

        // SAFETY: `symbol` cannot be invalid at this point
        let identifier = unsafe { Self::new_unchecked(symbol) };
        Some(identifier)
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn take_symbol(self) -> String {
        self.symbol
    }
}

fn is_identifier_symbol(symbol: &str) -> bool {
    if symbol.is_empty() {
        return false;
    }

    symbol.chars().enumerate().all(|(index, character)| {
        if index == 0 {
            return is_identifier_start(character);
        }

        is_identifier_character(character)
    })
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
        // SAFETY: We checked every character we pushed to `symbol` via `push`
        let identifier = unsafe { Identifier::new_unchecked(self.symbol) };
        Token::Identifier(identifier)
    }
}

const fn is_identifier_start(character: char) -> bool {
    character.is_ascii_alphabetic()
}

const fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric()
}
