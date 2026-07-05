use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::Expression;
use crate::Operator;
use crate::Span;
use crate::Spanned;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct BinaryExpression {
    operator: Spanned<Operator>,
    operands: Box<[Spanned<Expression>; 2]>,
}

impl BinaryExpression {
    pub fn new(operator: Spanned<Operator>, operands: [Spanned<Expression>; 2]) -> Self {
        Self {
            operator,
            operands: Box::new(operands),
        }
    }

    pub const fn operator(&self) -> &Spanned<Operator> {
        &self.operator
    }

    pub fn operands(&self) -> &[Spanned<Expression>; 2] {
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

    pub fn into_values(self) -> (Spanned<Operator>, Spanned<Expression>, Spanned<Expression>) {
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

impl From<BinaryExpression> for Spanned<Expression> {
    fn from(value: BinaryExpression) -> Self {
        let span = value.span();
        let expression = value.into();
        Spanned {
            content: expression,
            span,
        }
    }
}

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let BinaryExpression { operator, operands } = self;
        let [operand0, operand1] = &**operands;

        write!(f, "{operator}({operand0}, {operand1})")
    }
}
