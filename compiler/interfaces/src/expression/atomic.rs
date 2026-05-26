use std::fmt::{self, Display, Formatter};

use crate::{Expression, LiteralExpression};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum AtomicExpression {
    Literal(LiteralExpression),
}

impl From<AtomicExpression> for Expression {
    fn from(value: AtomicExpression) -> Self {
        Self::Atomic(value)
    }
}

impl Display for AtomicExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AtomicExpression::Literal(literal_expression) => literal_expression.fmt(f),
        }
    }
}
