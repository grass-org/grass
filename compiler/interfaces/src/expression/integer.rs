use crate::{AtomicExpression, NumericLiteralKind};
use std::fmt;
use std::fmt::{Display, Formatter};
use repetitive::repetitive;
use crate::expression::numeric::from;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum IntegerLiteral {
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Integer128(i128),
    ArchInteger(isize),
}

impl IntegerLiteral {
    pub fn kind(self) -> IntegerLiteralKind {
        match self {
            IntegerLiteral::Integer8(_) => IntegerLiteralKind::Integer8,
            IntegerLiteral::Integer16(_) => IntegerLiteralKind::Integer16,
            IntegerLiteral::Integer32(_) => IntegerLiteralKind::Integer32,
            IntegerLiteral::Integer64(_) => IntegerLiteralKind::Integer64,
            IntegerLiteral::Integer128(_) => IntegerLiteralKind::Integer128,
            IntegerLiteral::ArchInteger(_) => IntegerLiteralKind::ArchInteger,
        }
    }
}

impl From<IntegerLiteral> for AtomicExpression {
    fn from(value: IntegerLiteral) -> Self {
        Self::IntegerLiteral(value)
    }
}

repetitive! {
    @for size in [8, 16, 32, 64, 128] {
        from!(@['i' size], IntegerLiteral , @["Integer" size]);
    }
}

from!(isize, IntegerLiteral, ArchInteger);

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IntegerLiteral::Integer8(value) => write!(f, "{value}:I8"),
            IntegerLiteral::Integer16(value) => write!(f, "{value}:I16"),
            IntegerLiteral::Integer32(value) => write!(f, "{value}:I32"),
            IntegerLiteral::Integer64(value) => write!(f, "{value}:I64"),
            IntegerLiteral::Integer128(value) => write!(f, "{value}:I128"),
            IntegerLiteral::ArchInteger(value) => write!(f, "{value}:I128"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum IntegerLiteralKind {
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Integer128,
    ArchInteger,
}

impl IntegerLiteralKind {
    pub const ALL: [IntegerLiteralKind; 6] = [
        IntegerLiteralKind::Integer8,
        IntegerLiteralKind::Integer16,
        IntegerLiteralKind::Integer32,
        IntegerLiteralKind::Integer64,
        IntegerLiteralKind::Integer128,
        IntegerLiteralKind::ArchInteger,
    ];

    pub fn all() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }

    pub fn max_value(self) -> u128 {
        match self {
            IntegerLiteralKind::Integer8 => i8::MAX as u128,
            IntegerLiteralKind::Integer16 => i16::MAX as u128,
            IntegerLiteralKind::Integer32 => i32::MAX as u128,
            IntegerLiteralKind::Integer64 => i64::MAX as u128,
            IntegerLiteralKind::Integer128 => i128::MAX as u128,
            IntegerLiteralKind::ArchInteger => isize::MAX as u128,
        }
    }
}

impl From<IntegerLiteralKind> for NumericLiteralKind {
    fn from(value: IntegerLiteralKind) -> Self {
        Self::Integer(value)
    }
}

impl Display for IntegerLiteralKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
