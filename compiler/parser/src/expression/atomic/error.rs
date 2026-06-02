use crate::{NumericLiteralError, ParseExpressionError};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum AtomicExpressionError {
    NumericLiteralError(NumericLiteralError),
    UnknownError, // TODO: This shouldn't be a thing
}

impl From<AtomicExpressionError> for ParseExpressionError {
    fn from(value: AtomicExpressionError) -> Self {
        Self::AtomicExpressionError(value)
    }
}

impl Display for AtomicExpressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AtomicExpressionError::NumericLiteralError(error) => error.fmt(f),
            AtomicExpressionError::UnknownError => write!(f, "unknown atomic expression error"),
        }
    }
}

impl Error for AtomicExpressionError {}
