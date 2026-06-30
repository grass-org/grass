use super::{Error, ExpressionParser, Result};
use interfaces::{ExpressionSpan, Operator, OperatorSpan};
use lexer::{Token, TokenSpan};
use std::iter::Peekable;

type OperatorToken = lexer::Operator;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    pub(super) fn parse_right(
        &mut self,
        min_binding_power: u32,
    ) -> Result<(OperatorSpan, ExpressionSpan)> {
        let OperatorToken { symbol, .. } =
            peek_operator_token(&mut self.tokens).ok_or(Error::NoMoreTokens)?;

        let binding_power = self.binding_powers.infix_binding_power(symbol)?;

        if binding_power.left < min_binding_power {
            return Err(Error::NoMoreTokens);
        }

        let operator = next_infix_operator(&mut self.tokens).ok_or(Error::NoMoreTokens)?;

        let expression = self.parse_folding(binding_power.right)?;
        Ok((operator, expression))
    }
}

fn peek_operator_token(
    tokens: &mut Peekable<impl Iterator<Item = TokenSpan>>,
) -> Option<&OperatorToken> {
    let TokenSpan { token, .. } = tokens.peek()?;

    let Token::Operator(operator) = token else {
        return None;
    };

    Some(operator)
}

fn next_infix_operator(mut tokens: impl Iterator<Item = TokenSpan>) -> Option<OperatorSpan> {
    let TokenSpan { token, span } = tokens.next()?;

    let Token::Operator(OperatorToken { symbol, .. }) = token else {
        return None;
    };

    let operator = Operator { symbol };
    Some(OperatorSpan { operator, span })
}
