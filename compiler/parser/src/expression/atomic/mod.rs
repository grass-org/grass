mod error;
mod numeric;

pub use error::*;
pub use numeric::*;

use interfaces::{AtomicExpression, Spanned, SyntaxKind};
use lexer::Token;
use std::result;

use crate::UnexpectedSyntaxError;

type Error = AtomicExpressionError;
type Result<T = AtomicExpression, E = Error> = result::Result<T, E>;

pub(super) fn parse_atomic_expression(token: Spanned<Token>) -> Result {
    let Spanned { content, span } = token;

    let operand = match content {
        Token::NumericLiteral(numeric_literal) => parse_numeric_literal(numeric_literal)?,
        _ => {
            return Err(UnexpectedSyntaxError {
                expected: SyntaxKind::AtomicExpression,
                actual: content,
                span,
            })?;
        }
    };

    Ok(operand)
}
