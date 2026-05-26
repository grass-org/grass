mod numeric;
mod string;

pub(crate) use numeric::*;
pub(crate) use string::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct Literal {
    kind: LiteralKind,
    symbol: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum LiteralKind {
    Integer,
    Fraction,
    String,
}

impl Literal {
    /// # Safety
    ///
    /// The caller must guarantee that `symbol` is valid
    pub const unsafe fn new_unchecked(kind: LiteralKind, symbol: String) -> Self {
        Literal { kind, symbol }
    }

    pub fn new(kind: LiteralKind, symbol: impl Into<String>) -> Option<Self> {
        let symbol = symbol.into();

        let valid = match kind {
            LiteralKind::Integer => is_valid_integer(&symbol),
            LiteralKind::Fraction => is_valid_fraction(&symbol),
            LiteralKind::String => is_valid_string(&symbol),
        };

        if !valid {
            return None;
        }

        // SAFETY: `symbol` cannot be invalid at this point
        let literal = unsafe { Self::new_unchecked(kind, symbol) };
        Some(literal)
    }

    pub fn integer(symbol: impl Into<String>) -> Option<Self> {
        let symbol = symbol.into();

        if !is_valid_integer(&symbol) {
            return None;
        }

        // SAFETY: `symbol` cannot be invalid at this point
        let literal = unsafe { Self::new_unchecked(LiteralKind::Integer, symbol) };
        Some(literal)
    }

    pub fn fraction(symbol: impl Into<String>) -> Option<Self> {
        let symbol = symbol.into();

        if !is_valid_fraction(&symbol) {
            return None;
        }

        // SAFETY: `symbol` cannot be invalid at this point
        let literal = unsafe { Self::new_unchecked(LiteralKind::Fraction, symbol) };
        Some(literal)
    }

    pub fn string(symbol: impl Into<String>) -> Option<Self> {
        let symbol = symbol.into();

        if !is_valid_string(&symbol) {
            return None;
        }

        // SAFETY: `symbol` cannot be invalid at this point
        let literal = unsafe { Self::new_unchecked(LiteralKind::String, symbol) };
        Some(literal)
    }

    pub const fn kind(&self) -> LiteralKind {
        self.kind
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn take_symbol(self) -> String {
        self.symbol
    }
}
