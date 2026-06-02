mod error;
mod numeric;

use interfaces::AtomicExpression;
use lexer::Token;

pub use error::*;
pub use numeric::*;

pub(super) fn parse_atomic(token: Token) -> Result<AtomicExpression, AtomicExpressionError> {
    let operand = match token {
        Token::NumericLiteral(numeric_literal) => parse_numeric_literal(numeric_literal)?,
        Token::Identifier(_) => return Err(AtomicExpressionError::UnknownError),
        _ => return Err(AtomicExpressionError::UnknownError),
    };

    Ok(operand)
}
