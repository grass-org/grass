mod atomic;
mod binary;
mod character;
mod fraction;
mod integer;
mod magnitude;
mod numeric;
mod operator;
mod unary;

pub use atomic::*;
pub use binary::*;
pub use character::*;
pub use fraction::*;
pub use integer::*;
pub use magnitude::*;
pub use numeric::*;
pub use operator::*;
pub use unary::*;

use crate::Span;
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct ExpressionSpan {
    pub expression: Expression,
    pub span: Span,
}

impl Display for ExpressionSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.expression.fmt(f)
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Expression {
    Atomic(AtomicExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atomic(expression) => expression.fmt(f),
            Expression::Unary(expression) => expression.fmt(f),
            Expression::Binary(expression) => expression.fmt(f),
        }
    }
}
