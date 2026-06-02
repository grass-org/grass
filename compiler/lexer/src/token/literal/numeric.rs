use crate::{PushTokenCharacterResult, Token, TokenBuilder};
use std::fmt::{Display, Formatter};
use std::mem::take;
use std::{char, fmt};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct NumericLiteral {
    pub base: Option<String>,
    pub integral_radits: String,
    pub fractional_radits: Option<String>,
    pub kind: Option<String>,
}

impl Display for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let NumericLiteral {
            base,
            integral_radits,
            fractional_radits,
            kind,
        } = self;

        if let Some(base) = base {
            write!(f, "{base}#")?;
        };

        write!(f, "{integral_radits}")?;

        if let Some(fractional_radits) = fractional_radits {
            write!(f, ".{fractional_radits}")?;
        };

        if let Some(kind) = kind {
            write!(f, ":{kind}")?;
        };

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct NumericLiteralBuilder {
    pub base: Option<String>,
    pub integral_radits: String,
    pub fractional_radits: Option<String>,
    pub kind: Option<String>,
}

impl NumericLiteralBuilder {
    pub fn start(start: char) -> Option<Self> {
        if !start.is_ascii_digit() {
            return None;
        }

        let mut integral_radits = String::new();
        integral_radits.push(start);

        let builder = Self {
            base: None,
            integral_radits,
            fractional_radits: None,
            kind: None,
        };

        Some(builder)
    }

    #[must_use]
    fn move_symbol_to_base(&mut self) -> PushTokenCharacterResult {
        if self.base.is_some() || self.fractional_radits.is_some() || self.kind.is_some() {
            return PushTokenCharacterResult::Failed;
        }

        self.base = Some(take(&mut self.integral_radits));
        PushTokenCharacterResult::Successful
    }

    #[must_use]
    fn start_fractional(&mut self) -> PushTokenCharacterResult {
        if self.fractional_radits.is_some() || self.kind.is_some() {
            return PushTokenCharacterResult::Failed;
        }

        self.fractional_radits = Some(String::new());
        PushTokenCharacterResult::Successful
    }

    #[must_use]
    fn start_kind(&mut self) -> PushTokenCharacterResult {
        if self.kind.is_some() {
            return PushTokenCharacterResult::Failed;
        }

        self.kind = Some(String::new());
        PushTokenCharacterResult::Successful
    }

    fn push_symbol(&mut self, character: char) {
        if let Some(kind) = &mut self.kind {
            kind.push(character);
            return;
        };

        if let Some(fractional_radits) = &mut self.fractional_radits {
            fractional_radits.push(character);
            return;
        };

        self.integral_radits.push(character);
    }
}

impl TokenBuilder for NumericLiteralBuilder {
    fn push(&mut self, character: char) -> PushTokenCharacterResult {
        if character == '#' {
            return self.move_symbol_to_base();
        }

        if character == '.' {
            return self.start_fractional();
        }

        if character == ':' {
            return self.start_kind();
        }

        if !character.is_ascii_alphanumeric() {
            return PushTokenCharacterResult::Failed;
        }

        self.push_symbol(character);
        PushTokenCharacterResult::Successful
    }

    fn build(self) -> Token {
        let literal = NumericLiteral {
            base: self.base,
            integral_radits: self.integral_radits,
            fractional_radits: self.fractional_radits,
            kind: self.kind,
        };

        Token::NumericLiteral(literal)
    }
}
