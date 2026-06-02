use crate::{AtomicExpression, NumericLiteralKind};
use std::fmt;
use std::fmt::{Display, Formatter};
use repetitive::repetitive;
use crate::expression::numeric::from;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum MagnitudeLiteral {
    Magnitude8(u8),
    Magnitude16(u16),
    Magnitude32(u32),
    Magnitude64(u64),
    Magnitude128(u128),
    ArchMagnitude(usize),
}

impl MagnitudeLiteral {
    pub fn kind(self) -> MagnitudeLiteralKind {
        match self {
            MagnitudeLiteral::Magnitude8(_) => MagnitudeLiteralKind::Magnitude8,
            MagnitudeLiteral::Magnitude16(_) => MagnitudeLiteralKind::Magnitude16,
            MagnitudeLiteral::Magnitude32(_) => MagnitudeLiteralKind::Magnitude32,
            MagnitudeLiteral::Magnitude64(_) => MagnitudeLiteralKind::Magnitude64,
            MagnitudeLiteral::Magnitude128(_) => MagnitudeLiteralKind::Magnitude128,
            MagnitudeLiteral::ArchMagnitude(_) => MagnitudeLiteralKind::ArchMagnitude,
        }
    }
}

impl From<MagnitudeLiteral> for AtomicExpression {
    fn from(value: MagnitudeLiteral) -> Self {
        Self::MagnitudeLiteral(value)
    }
}

repetitive! {
    @for size in [8, 16, 32, 64, 128] {
        from!(@['u' size], MagnitudeLiteral , @["Magnitude" size]);
    }
}

from!(usize, MagnitudeLiteral, ArchMagnitude);

impl Display for MagnitudeLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MagnitudeLiteral::Magnitude8(value) => write!(f, "{value}:M8"),
            MagnitudeLiteral::Magnitude16(value) => write!(f, "{value}:M16"),
            MagnitudeLiteral::Magnitude32(value) => write!(f, "{value}:M32"),
            MagnitudeLiteral::Magnitude64(value) => write!(f, "{value}:M64"),
            MagnitudeLiteral::Magnitude128(value) => write!(f, "{value}:M128"),
            MagnitudeLiteral::ArchMagnitude(value) => write!(f, "{value}:M128"),
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
    
    pub fn max_value(self) -> u128 {
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
