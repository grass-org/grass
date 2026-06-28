use super::{Error, ExpressionParser, Result};
use interfaces::{ExpressionSpan, OperatorSpan};
use lexer::TokenSpan;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    pub(super) fn parse_right(
        &mut self,
        min_binding_power: u32,
    ) -> Result<(OperatorSpan, ExpressionSpan)> {
        let operator_span = self.peek_operator()?;

        let binding_power = self
            .binding_powers
            .infix_binding_power(&operator_span.operator)?;

        if binding_power.left < min_binding_power {
            return Err(Error::NoMoreTokens);
        }

        _ = self.next_token();

        let expression = self.parse_folding(binding_power.right)?;
        Ok((operator_span, expression))
    }
}
