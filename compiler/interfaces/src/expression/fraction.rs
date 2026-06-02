use crate::{AtomicExpression, NumericLiteralKind};
use std::fmt;
use std::fmt::{Display, Formatter};
use repetitive::repetitive;
use crate::expression::numeric::from;

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum FractionLiteral {
    Fraction32(f32),
    Fraction64(f64),
}

impl FractionLiteral {
    pub fn kind(self) -> FractionLiteralKind {
        match self {
            FractionLiteral::Fraction32(_) => FractionLiteralKind::Fraction32,
            FractionLiteral::Fraction64(_) => FractionLiteralKind::Fraction64,
        }
    }
}

impl From<FractionLiteral> for AtomicExpression {
    fn from(value: FractionLiteral) -> Self {
        Self::FractionLiteral(value)
    }
}

impl Display for FractionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FractionLiteral::Fraction32(value) => write!(f, "{value}:F32"),
            FractionLiteral::Fraction64(value) => write!(f, "{value}:F64"),
        }
    }
}

repetitive! {
    @for size in [32, 64] {
        from!(@['f' size], FractionLiteral, @["Fraction" size]);
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
