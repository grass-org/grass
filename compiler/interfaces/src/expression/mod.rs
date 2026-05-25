mod atomic;
mod binary;
mod literal;

use std::fmt::{self, Display, Formatter};

pub use atomic::*;
pub use binary::*;
pub use literal::*;

use crate::Span;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct ExpressionSpan {
    pub expression: Expression,
    pub span: Span,
}

impl ExpressionSpan {
    pub const fn atomic(atomic_expression: AtomicExpression, span: Span) -> Self {
        let expression = Expression::Atomic(atomic_expression);
        ExpressionSpan { expression, span }
    }

    pub const fn binary(binary_expression: BinaryExpression) -> Self {
        let span = binary_expression.span();
        let expression = Expression::Binary(binary_expression);
        ExpressionSpan { expression, span }
    }
}

impl Display for ExpressionSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.expression.fmt(f)
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Expression {
    Atomic(AtomicExpression),
    Binary(BinaryExpression),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atomic(atomic_expression) => atomic_expression.fmt(f),
            Expression::Binary(binary_expression) => binary_expression.fmt(f),
        }
    }
}
