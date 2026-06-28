use super::{Error, ExpressionParser, Result};
use crate::UnexpectedSyntaxError;
use interfaces::{Operator, OperatorSpan, Span, SyntaxKind};
use lexer::{Token, TokenSpan};

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    pub(super) fn peek_operator(&mut self) -> Result<OperatorSpan> {
        let Some(token) = self.tokens.peek() else {
            return Err(Error::NoMoreTokens);
        };

        // TODO: this is a String clone!!
        Ok(parse_operator(token.clone())?)
    }
}

fn parse_operator(token: TokenSpan) -> Result<OperatorSpan, UnexpectedSyntaxError> {
    let TokenSpan { token, span } = token;

    let Token::Operator(operator) = token else {
        return Err(UnexpectedSyntaxError {
            expected: SyntaxKind::Operator,
            actual: token,
            span,
        });
    };

    let operator = Operator {
        symbol: operator.symbol,
    };

    Ok(OperatorSpan { operator, span })
}
