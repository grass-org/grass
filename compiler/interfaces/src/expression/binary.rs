use crate::{Expression, ExpressionSpan, OperatorSpan, Span};
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct BinaryExpression {
    pub operator: OperatorSpan,
    pub operand_0: Box<ExpressionSpan>,
    pub operand_1: Box<ExpressionSpan>,
}

impl BinaryExpression {
    pub fn new(
        operator: OperatorSpan,
        operand_0: ExpressionSpan,
        operand_1: ExpressionSpan,
    ) -> Self {
        Self {
            operator,
            operand_0: Box::new(operand_0),
            operand_1: Box::new(operand_1),
        }
    }

    pub const fn span(&self) -> Span {
        let left_span = self.operand_0.span;
        let right_span = self.operand_1.span;

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
        let span = value.span();
        let expression = value.into();
        ExpressionSpan { expression, span }
    }
}

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let BinaryExpression {
            operator,
            operand_0,
            operand_1,
        } = self;

        write!(f, "{operator}({operand_0}, {operand_1})")
    }
}
