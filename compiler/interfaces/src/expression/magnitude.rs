use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::AtomicExpression;
use crate::NumericLiteralKind;

/// a magnitude literal represented as u128.
/// example: `365:M64`
/// we'll just do further bounds check in later stages
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct MagnitudeLiteral {
    pub value: u128,
    pub kind: MagnitudeLiteralKind,
}

impl From<MagnitudeLiteral> for AtomicExpression {
    fn from(value: MagnitudeLiteral) -> Self {
        Self::MagnitudeLiteral(value)
    }
}

impl Display for MagnitudeLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let MagnitudeLiteral { value, kind } = self;

        match kind {
            MagnitudeLiteralKind::Magnitude8 => write!(f, "{value}:M8"),
            MagnitudeLiteralKind::Magnitude16 => write!(f, "{value}:M16"),
            MagnitudeLiteralKind::Magnitude32 => write!(f, "{value}:M32"),
            MagnitudeLiteralKind::Magnitude64 => write!(f, "{value}:M64"),
            MagnitudeLiteralKind::Magnitude128 => write!(f, "{value}:M128"),
            MagnitudeLiteralKind::ArchMagnitude => write!(f, "{value}:M128"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum MagnitudeLiteralKind {
    Magnitude8,
    Magnitude16,
    Magnitude32,
    Magnitude64,
    Magnitude128,
    ArchMagnitude,
}

impl MagnitudeLiteralKind {
    pub const ALL: [MagnitudeLiteralKind; 6] = [
        MagnitudeLiteralKind::Magnitude8,
        MagnitudeLiteralKind::Magnitude16,
        MagnitudeLiteralKind::Magnitude32,
        MagnitudeLiteralKind::Magnitude64,
        MagnitudeLiteralKind::Magnitude128,
        MagnitudeLiteralKind::ArchMagnitude,
    ];

    pub fn all() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }

    pub const fn max_value(self) -> u128 {
        match self {
            MagnitudeLiteralKind::Magnitude8 => u8::MAX as u128,
            MagnitudeLiteralKind::Magnitude16 => u16::MAX as u128,
            MagnitudeLiteralKind::Magnitude32 => u32::MAX as u128,
            MagnitudeLiteralKind::Magnitude64 => u64::MAX as u128,
            MagnitudeLiteralKind::Magnitude128 => u128::MAX,
            MagnitudeLiteralKind::ArchMagnitude => usize::MAX as u128,
        }
    }
}

impl From<MagnitudeLiteralKind> for NumericLiteralKind {
    fn from(value: MagnitudeLiteralKind) -> Self {
        Self::Magnitude(value)
    }
}

impl Display for MagnitudeLiteralKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
