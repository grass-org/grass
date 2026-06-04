use crate::{AtomicExpression, NumericLiteralKind};
use std::fmt::{self, Display, Formatter};

/// a fraction literal represented as f64.
/// example: `365.67:F32`
/// we'll just do further bounds check in later stages
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct FractionLiteral {
    pub value: f64,
    pub kind: FractionLiteralKind,
}

impl From<FractionLiteral> for AtomicExpression {
    fn from(value: FractionLiteral) -> Self {
        Self::FractionLiteral(value)
    }
}

impl Display for FractionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let FractionLiteral { value, kind } = self;
        match kind {
            FractionLiteralKind::Fraction32 => write!(f, "{value}:F32"),
            FractionLiteralKind::Fraction64 => write!(f, "{value}:F64"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum FractionLiteralKind {
    Fraction32,
    Fraction64,
}

impl FractionLiteralKind {
    pub const ALL: [FractionLiteralKind; 2] = [
        FractionLiteralKind::Fraction32,
        FractionLiteralKind::Fraction64,
    ];

    pub fn all() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }
}

impl From<FractionLiteralKind> for NumericLiteralKind {
    fn from(value: FractionLiteralKind) -> Self {
        Self::Fraction(value)
    }
}

impl Display for FractionLiteralKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
