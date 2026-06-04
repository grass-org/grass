use crate::{CharacterLiteral, Expression, FractionLiteral, IntegerLiteral, MagnitudeLiteral};
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum AtomicExpression {
    IntegerLiteral(IntegerLiteral),
    MagnitudeLiteral(MagnitudeLiteral),
    FractionLiteral(FractionLiteral),
    CharacterLiteral(CharacterLiteral),
    // String(StringLiteral),
}

impl From<AtomicExpression> for Expression {
    fn from(value: AtomicExpression) -> Self {
        Self::Atomic(value)
    }
}

impl Display for AtomicExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AtomicExpression::IntegerLiteral(literal) => write!(f, "{literal}"),
            AtomicExpression::MagnitudeLiteral(literal) => write!(f, "{literal}"),
            AtomicExpression::FractionLiteral(literal) => write!(f, "{literal}"),
            AtomicExpression::CharacterLiteral(literal) => write!(f, "{literal}"),
        }
    }
}
