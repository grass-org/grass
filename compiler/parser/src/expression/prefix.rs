use super::{ExpressionParser, Result};
use interfaces::{Operator, OperatorSpan, Span, UnaryExpression};
use lexer::TokenSpan;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    pub(super) fn parse_prefix_expression(&mut self, symbol: String, span: Span) -> Result {
        let operator = Operator { symbol };

        let binding_power = self.binding_powers.prefix_binding_power(&operator)?;

        let operator = OperatorSpan { operator, span };
        let operand = self.parse_folding(binding_power)?;

        Ok(UnaryExpression::new(operator, operand).into())
    }
}
