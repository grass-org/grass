use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::NumericLiteralError;
use crate::ParseExpressionError;
use crate::UnexpectedSyntaxError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum AtomicExpressionError {
    UnexpectedSyntax(UnexpectedSyntaxError),
    NumericLiteralError(NumericLiteralError),
}

impl From<UnexpectedSyntaxError> for AtomicExpressionError {
    fn from(value: UnexpectedSyntaxError) -> Self {
        Self::UnexpectedSyntax(value)
    }
}

impl From<AtomicExpressionError> for ParseExpressionError {
    fn from(value: AtomicExpressionError) -> Self {
        match value {
            AtomicExpressionError::UnexpectedSyntax(error) => Self::UnexpectedSyntax(error),
            AtomicExpressionError::NumericLiteralError(error) => Self::NumericLiteralError(error),
        }
    }
}

impl Display for AtomicExpressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AtomicExpressionError::UnexpectedSyntax(error) => error.fmt(f),
            AtomicExpressionError::NumericLiteralError(error) => error.fmt(f),
        }
    }
}

impl Error for AtomicExpressionError {}
