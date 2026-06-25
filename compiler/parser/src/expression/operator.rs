use super::{Error, ExpressionParser, Result};
use crate::UnexpectedSyntaxError;
use interfaces::{Operator, OperatorSpan, Span};
use lexer::{Token, TokenSpan};

type OperatorToken = lexer::Operator;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    pub(super) fn next_if_operator(&mut self) -> Result<OperatorSpan> {
        let (operator, span) = self
            .tokens
            .next_if_map(if_operator_map)
            .ok_or(Error::NoMoreTokens)?;

        Ok(parse_operator(operator, span)?)
    }
}

fn if_operator_map(token_span: TokenSpan) -> Result<(OperatorToken, Span), TokenSpan> {
    let Token::Operator(operator) = token_span.token else {
        return Err(token_span);
    };

    Ok((operator, token_span.span))
}

fn parse_operator(
    operator: OperatorToken,
    span: Span,
) -> Result<OperatorSpan, UnexpectedSyntaxError> {
    let operator = Operator {
        symbol: operator.symbol,
    };

    Ok(OperatorSpan { operator, span })
}
