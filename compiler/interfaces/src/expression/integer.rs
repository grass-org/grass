use crate::{AtomicExpression, NumericLiteralKind};
use std::fmt;
use std::fmt::{Display, Formatter};

/// a magnitude literal represented as u128, since it can only be positive.
/// example: `365:I64`
/// if there is a negation `-67`, only `67` will be represented by `IntegerLiteral`.
/// we use `u128` instead of `u128`,
/// since we need to be able to represent `i128::MIN.abs()`,
/// which is `i128::MAX + 1`, which is `> i128::MAX`.
/// we'll just do further bounds check in later stages
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct IntegerLiteral {
    pub value: u128,
    pub kind: IntegerLiteralKind,
}

impl From<IntegerLiteral> for AtomicExpression {
    fn from(value: IntegerLiteral) -> Self {
        Self::IntegerLiteral(value)
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let IntegerLiteral { value, kind } = self;

        match kind {
            IntegerLiteralKind::Integer8 => write!(f, "{value}:I8"),
            IntegerLiteralKind::Integer16 => write!(f, "{value}:I16"),
            IntegerLiteralKind::Integer32 => write!(f, "{value}:I32"),
            IntegerLiteralKind::Integer64 => write!(f, "{value}:I64"),
            IntegerLiteralKind::Integer128 => write!(f, "{value}:I128"),
            IntegerLiteralKind::ArchInteger => write!(f, "{value}:I128"),
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
