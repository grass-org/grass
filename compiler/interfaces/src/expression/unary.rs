use std::cmp::{max, min};
use std::fmt::{self, Display, Formatter};

use crate::{Expression, ExpressionSpan, Operator, Span, Spanned};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct UnaryExpression {
    operator: Spanned<Operator>,
    operand: Box<ExpressionSpan>,
}

impl UnaryExpression {
    pub fn new(operator: Spanned<Operator>, operand: ExpressionSpan) -> Self {
        Self {
            operator,
            operand: Box::new(operand),
        }
    }

    pub const fn operator(&self) -> &Spanned<Operator> {
        &self.operator
    }

    pub fn operand(&self) -> &ExpressionSpan {
        &self.operand
    }

    pub fn span(&self) -> Span {
        let operator_span = self.operand().span;
        let operand_span = self.operand().span;

        let start = min(operator_span.start, operand_span.start);
        let end = max(operator_span.end(), operand_span.end());
        let length = end - start;

        Span { start, length }
    }

    pub fn into_values(self) -> (Spanned<Operator>, ExpressionSpan) {
        let UnaryExpression { operator, operand } = self;
        (operator, *operand)
    }
}

impl From<UnaryExpression> for Expression {
    fn from(value: UnaryExpression) -> Self {
        Self::Unary(value)
    }
}

impl From<UnaryExpression> for ExpressionSpan {
    fn from(value: UnaryExpression) -> Self {
        let span = value.span();
        let expression = value.into();
        ExpressionSpan { expression, span }
    }
}

impl Display for UnaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let UnaryExpression { operator, operand } = self;
        write!(f, "{operator}({operand})")
    }
}
