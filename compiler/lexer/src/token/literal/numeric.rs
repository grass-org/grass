use crate::{Literal, LiteralKind, PushTokenCharacterResult, Token, TokenBuilder};

#[derive(Debug)]
pub(crate) struct NumericLiteralBuilder {
    symbol: String,
    kind: NumericLiteralKind,
}

impl NumericLiteralBuilder {
    pub fn new(start: char) -> Option<Self> {
        if !start.is_numeric() {
            return None;
        }

        let mut symbol = String::new();
        symbol.push(start);

        let builder = Self {
            symbol,
            kind: NumericLiteralKind::Integer,
        };

        Some(builder)
    }

    const fn push_action(&self, character: char) -> NumericLiteralPushAction {
        if character == '.' {
            return match self.kind {
                NumericLiteralKind::Integer => NumericLiteralPushAction::ToFraction,
                NumericLiteralKind::Fraction => NumericLiteralPushAction::Fail,
            };
        }

        if !character.is_ascii_digit() {
            return NumericLiteralPushAction::Fail;
        }

        NumericLiteralPushAction::Push
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum NumericLiteralPushAction {
    Fail,
    Push,
    ToFraction,
}

impl TokenBuilder for NumericLiteralBuilder {
    fn push(&mut self, character: char) -> PushTokenCharacterResult {
        let push_action = self.push_action(character);

        if push_action == NumericLiteralPushAction::Fail {
            return PushTokenCharacterResult::Failed;
        }

        if push_action == NumericLiteralPushAction::ToFraction {
            self.kind = NumericLiteralKind::Fraction;
        }

        self.symbol.push(character);
        PushTokenCharacterResult::Successful
    }

    fn build(self) -> Token {
        // SAFETY: We checked every character we pushed to `symbol` via `push`
        let literal = unsafe { Literal::new_unchecked(self.kind.into(), self.symbol) };
        Token::Literal(literal)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum NumericLiteralKind {
    Integer,
    Fraction,
}

impl From<NumericLiteralKind> for LiteralKind {
    fn from(value: NumericLiteralKind) -> Self {
        match value {
            NumericLiteralKind::Integer => LiteralKind::Integer,
            NumericLiteralKind::Fraction => LiteralKind::Fraction,
        }
    }
}

pub(super) fn is_valid_integer(symbol: &str) -> bool {
    // TODO: Support suffixes for types
    symbol.chars().all(|character| character.is_numeric())
}

pub(super) fn is_valid_fraction(symbol: &str) -> bool {
    // TODO: Support suffixes for types
    let mut has_dot = false;

    for character in symbol.chars() {
        if character == '.' {
            if has_dot {
                return false;
            }

            has_dot = true;
        }

        if !character.is_ascii_digit() {
            return false;
        }
    }

    has_dot
}
