use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::CharacterLiteral;
use crate::Expression;
use crate::FractionLiteral;
use crate::IntegerLiteral;
use crate::MagnitudeLiteral;

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
