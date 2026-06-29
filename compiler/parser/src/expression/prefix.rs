use super::{ExpressionParser, Result};
use interfaces::{Operator, Span, Spanned, UnaryExpression};
use lexer::Token;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = Spanned<Token>>,
{
    pub(super) fn parse_prefix_expression(&mut self, symbol: String, span: Span) -> Result {
        let binding_power = self.binding_powers.prefix_binding_power(&symbol)?;

        let operator = Operator { symbol };
        let operator = Spanned {
            content: operator,
            span,
        };
        let operand = self.parse_folding(binding_power)?;

        Ok(UnaryExpression::new(operator, operand).into())
    }
}
