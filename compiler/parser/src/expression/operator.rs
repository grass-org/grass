use crate::UnexpectedSyntaxError;
use interfaces::{Operator, OperatorSpan, SyntaxKind};
use lexer::{Token, TokenSpan};

pub(super) fn parse_operator(token: &TokenSpan) -> Result<OperatorSpan, UnexpectedSyntaxError> {
    // TODO: this is a String clone!!
    let TokenSpan { token, span } = token.clone();

    let Token::Operator(symbol) = token else {
        let error = UnexpectedSyntaxError {
            expected: SyntaxKind::Operator,
            span,
        };

        return Err(error);
    };

    let operator = Operator { symbol };
    Ok(OperatorSpan { operator, span })
}
