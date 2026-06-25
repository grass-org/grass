mod error;
mod numeric;

pub use error::*;
pub use numeric::*;

use interfaces::{AtomicExpression, SyntaxKind};
use lexer::{Token, TokenSpan};
use std::result;

use crate::UnexpectedSyntaxError;

type Error = AtomicExpressionError;
type Result<T = AtomicExpression, E = Error> = result::Result<T, E>;

pub(super) fn parse_atomic_expression(token: TokenSpan) -> Result {
    let TokenSpan { token, span } = token;

    let operand = match token {
        Token::NumericLiteral(numeric_literal) => parse_numeric_literal(numeric_literal)?,
        _ => {
            return Err(UnexpectedSyntaxError {
                expected: SyntaxKind::AtomicExpression,
                actual: token,
                span,
            })?;
        }
    };

    Ok(operand)
}
