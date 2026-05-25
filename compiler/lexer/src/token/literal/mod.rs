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

    pub fn new(kind: LiteralKind, symbol: String) -> Option<Self> {
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
