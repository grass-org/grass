use crate::{Expression, ExpressionSpan, OperatorSpan, Span};
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct BinaryExpression {
    operator: OperatorSpan,
    operands: Box<[ExpressionSpan; 2]>,
}

impl BinaryExpression {
    pub fn new(operator: OperatorSpan, operands: [ExpressionSpan; 2]) -> Self {
        Self {
            operator,
            operands: Box::new(operands),
        }
    }

    pub const fn operator(&self) -> &OperatorSpan {
        &self.operator
    }

    pub fn operands(&self) -> &[ExpressionSpan; 2] {
        &self.operands
    }

    pub fn span(&self) -> Span {
        let operands = self.operands();

        let operand_0_span = operands[0].span;
        let operand_1_span = operands[1].span;

        let start = operand_0_span.start;
        let end = operand_1_span.end();
        let length = end - start;

        Span { start, length }
    }

    pub fn into_values(self) -> (OperatorSpan, ExpressionSpan, ExpressionSpan) {
        let BinaryExpression { operator, operands } = self;
        let [operand0, operand1] = *operands;
        (operator, operand0, operand1)
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
        let BinaryExpression { operator, operands } = self;
        let [operand0, operand1] = &**operands;

        write!(f, "{operator}({operand0}, {operand1})")
    }
}
