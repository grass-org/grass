use std::fmt::{self, Display, Formatter};

use crate::{Expression, ExpressionSpan, OperatorSpan, Span};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct BinaryExpression {
    pub left: Box<ExpressionSpan>,
    pub operator: OperatorSpan,
    pub right: Box<ExpressionSpan>,
}

impl BinaryExpression {
    pub fn new(left: ExpressionSpan, operator: OperatorSpan, right: ExpressionSpan) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub const fn span(&self) -> Span {
        let left_span = self.left.span;
        let right_span = self.right.span;

        let start = left_span.start;
        let end = right_span.end();
        let length = end - start;

        Span { start, length }
    }
}

impl From<BinaryExpression> for Expression {
    fn from(value: BinaryExpression) -> Self {
        Self::Binary(value)
    }
}

impl From<BinaryExpression> for ExpressionSpan {
    fn from(value: BinaryExpression) -> Self {
        Self::binary(value)
    }
}

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let BinaryExpression {
            left,
            operator,
            right,
        } = self;

        write!(f, "({left} {operator} {right})")
    }
}
