use crate::UnexpectedSyntaxError;
use interfaces::{Operator, OperatorSpan, SyntaxKind};
use lexer::{Token, TokenSpan};

pub(super) fn parse_operator(token: TokenSpan) -> Result<OperatorSpan, UnexpectedSyntaxError> {
    let TokenSpan { token, span } = token.clone();

    let Token::Operator(operator) = token else {
        let error = UnexpectedSyntaxError {
            expected: SyntaxKind::Operator,
            actual: token,
            span,
        };

        return Err(error);
    };

    let operator = Operator {
        symbol: operator.symbol,
    };

    Ok(OperatorSpan { operator, span })
}
