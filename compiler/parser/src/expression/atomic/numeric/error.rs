use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::AtomicExpressionError;
use crate::InvalidNumericBaseError;
use crate::InvalidNumericKindError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum NumericLiteralError {
    InvalidBase(InvalidNumericBaseError),
    InvalidRadit(InvalidRaditError),
    MissingIntegralPart,
    MissingFractionalPart,
    InvalidKind(InvalidNumericKindError),
    IntegerOverflow(MaxIntegerLiteralOverflowError),
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
        "missing literal literal fractional part; \
        literal literal has a '.' without any radits after it"
    )
}

fn format_missing_integral_part(f: &mut Formatter) -> fmt::Result {
    write!(
        f,
        "missing literal literal integral part; \
        literal literal does not have any value; \
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
            "invalid literal literal radit {radit}; \
            an integer of base {base} can only have radits from 0 to {max_radit}",
        )
    }
}

impl Error for InvalidRaditError {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct MaxIntegerLiteralOverflowError {
    pub base: u8,
    pub radits: String,
}

impl From<MaxIntegerLiteralOverflowError> for NumericLiteralError {
    fn from(value: MaxIntegerLiteralOverflowError) -> Self {
        Self::IntegerOverflow(value)
    }
}

impl Display for MaxIntegerLiteralOverflowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let MaxIntegerLiteralOverflowError { base, radits } = self;

        write!(f, "integer literal ")?;

        if *base != 10 {
            write!(f, "{base}#")?;
        }

        write!(
            f,
            "{radits} overflowed; integer literals can only fit up to u128::MAX"
        )
    }
}

impl Error for MaxIntegerLiteralOverflowError {}
