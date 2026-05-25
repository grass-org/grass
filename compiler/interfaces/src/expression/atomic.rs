use std::fmt::{self, Display, Formatter};

use crate::LiteralExpression;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum AtomicExpression {
    Literal(LiteralExpression),
}

impl Display for AtomicExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AtomicExpression::Literal(literal_expression) => literal_expression.fmt(f),
        }
    }
}
