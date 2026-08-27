use std::cmp::max;
use std::cmp::min;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::Expression;
use crate::Span;
use crate::Spanned;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct UnaryExpression {
    operator: Spanned<UnaryOperator>,
    operand: Box<Spanned<Expression>>,
}

impl UnaryExpression {
    pub fn new(operator: Spanned<UnaryOperator>, operand: Spanned<Expression>) -> Self {
        Self {
            operator,
            operand: Box::new(operand),
        }
    }

    pub const fn operator(&self) -> &Spanned<UnaryOperator> {
        &self.operator
    }

    pub fn operand(&self) -> &Spanned<Expression> {
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

    pub fn into_values(self) -> (Spanned<UnaryOperator>, Spanned<Expression>) {
        let UnaryExpression { operator, operand } = self;
        (operator, *operand)
    }
}

impl From<UnaryExpression> for Expression {
    fn from(value: UnaryExpression) -> Self {
        Self::Unary(value)
    }
}

impl From<UnaryExpression> for Spanned<Expression> {
    fn from(value: UnaryExpression) -> Self {
        let span = value.span();
        let expression = value.into();
        Spanned {
            content: expression,
            span,
        }
    }
}

impl Display for UnaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let UnaryExpression { operator, operand } = self;
        let operator = &operator.content;

        match operator.position {
            UnaryOperatorPosition::Prefix => write!(f, "{operator}({operand})"),
            UnaryOperatorPosition::Postfix => write!(f, "({operand}){operator}"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct UnaryOperator {
    pub symbol: String,
    pub position: UnaryOperatorPosition,
}

impl UnaryOperator {
    pub const fn prefix(symbol: String) -> Self {
        Self {
            symbol,
            position: UnaryOperatorPosition::Prefix,
        }
    }

    pub const fn postfix(symbol: String) -> Self {
        Self {
            symbol,
            position: UnaryOperatorPosition::Postfix,
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = &self.symbol;
        write!(f, "{symbol}")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum UnaryOperatorPosition {
    Prefix,
    Postfix,
}
