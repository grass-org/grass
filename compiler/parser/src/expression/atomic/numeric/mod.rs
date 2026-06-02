mod base;
mod fraction;
mod integer;
mod kind;
mod magnitude;
mod radit;

pub use base::*;
pub use kind::*;

use fraction::*;
use integer::*;
use magnitude::*;
use radit::*;

use crate::expression::atomic::AtomicExpressionError;
use interfaces::{AtomicExpression, NumericLiteralKind};
use lexer::NumericLiteral;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum NumericLiteralError {
    InvalidBase(InvalidNumericBaseError),
    InvalidRadit(InvalidRaditError),
    MissingIntegralPart,
    MissingFractionalPart,
    InvalidKind(InvalidNumericKindError),
    IntegerOverflow(IntegerLiteralOverflowError),
}

impl From<NumericLiteralError> for AtomicExpressionError {
    fn from(value: NumericLiteralError) -> Self {
        Self::NumericLiteralError(value)
    }
}

impl Display for NumericLiteralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NumericLiteralError::InvalidBase(error) => error.fmt(f),
            NumericLiteralError::InvalidRadit(error) => error.fmt(f),
            NumericLiteralError::MissingFractionalPart => format_missing_fractional_part(f),
            NumericLiteralError::MissingIntegralPart => format_missing_integral_part(f),
            NumericLiteralError::InvalidKind(error) => error.fmt(f),
            NumericLiteralError::IntegerOverflow(error) => error.fmt(f),
        }
    }
}

fn format_missing_fractional_part(f: &mut Formatter) -> fmt::Result {
    write!(
        f,
        "missing numeric literal fractional part; \
        numeric literal has a '.' without any radits after it"
    )
}

fn format_missing_integral_part(f: &mut Formatter) -> fmt::Result {
    write!(
        f,
        "missing numeric literal integral part; \
        numeric literal does not have any value; \
        consider adding '0' if you meant for it to be zero"
    )
}

impl Error for NumericLiteralError {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct InvalidRaditError {
    pub base: u8,
    pub radit: char,
    pub max_radit: char,
}

impl From<InvalidRaditError> for NumericLiteralError {
    fn from(value: InvalidRaditError) -> Self {
        Self::InvalidRadit(value)
    }
}

impl Display for InvalidRaditError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let InvalidRaditError {
            base,
            radit,
            max_radit,
        } = self;

        write!(
            f,
            "invalid numeric literal radit {radit}; \
            an integer of base {base} can only have radits from 0 to {max_radit}",
        )
    }
}

impl Error for InvalidRaditError {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct IntegerLiteralOverflowError {
    pub base: u8,
    pub kind: NumericLiteralKind,
    pub radits: String,
}

impl From<IntegerLiteralOverflowError> for NumericLiteralError {
    fn from(value: IntegerLiteralOverflowError) -> Self {
        Self::IntegerOverflow(value)
    }
}

impl Display for IntegerLiteralOverflowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let IntegerLiteralOverflowError { base, radits, kind } = self;

        write!(f, "integer literal ")?;

        if *base != 10 {
            write!(f, "{base}#")?;
        }

        write!(f, "{radits} overflowed; ")?;

        let max_value = match kind {
            NumericLiteralKind::Integer(kind) => kind.max_value(),
            NumericLiteralKind::Magnitude(kind) => kind.max_value(),
            NumericLiteralKind::Fraction(_) => panic!("a fraction cannot overflow!"),
        };

        write!(f, "{kind} can only fit up to {max_value}")
    }
}

impl Error for IntegerLiteralOverflowError {}
