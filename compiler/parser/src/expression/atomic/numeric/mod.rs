mod base;
mod error;
mod fraction;
mod integer;
mod kind;
mod magnitude;
mod radit;

pub use base::*;
pub use error::*;
pub use kind::*;

use fraction::*;
use integer::*;
use magnitude::*;
use radit::*;

use interfaces::{AtomicExpression, NumericLiteralKind};
use lexer::NumericLiteral;

pub(crate) fn parse_numeric_literal(
    numeric_literal: NumericLiteral,
) -> Result<AtomicExpression, NumericLiteralError> {
    let base = base(numeric_literal.base)?;

    let kind = kind(
        numeric_literal.kind,
        numeric_literal.fractional_radits.is_some(),
    )?;

    let integral_part = numeric_literal.integral_radits;
    let fractional_part = numeric_literal.fractional_radits;

    let literal = match kind {
        NumericLiteralKind::Integer(kind) => {
            parse_integer_literal(base, kind, integral_part)?.into()
        }
        NumericLiteralKind::Magnitude(kind) => {
            parse_magnitude_literal(base, kind, integral_part)?.into()
        }
        NumericLiteralKind::Fraction(kind) => {
            parse_fraction_literal(base, kind, integral_part, fractional_part)?.into()
        }
    };

    Ok(literal)
}
