use crate::{NumericLiteralError, UndefinedBindingPowerError};
use interfaces::{Span, SyntaxKind};
use lexer::Token;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum ParseExpressionError {
    NoMoreTokens,
    UnexpectedSyntax(UnexpectedSyntaxError),
    UndefinedBindingPower(UndefinedBindingPowerError),
    NumericLiteralError(NumericLiteralError),
}

impl Error for ParseExpressionError {}

impl Display for ParseExpressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMoreTokens => write!(f, "no more tokens to parse"),
            Self::UnexpectedSyntax(error) => error.fmt(f),
            Self::UndefinedBindingPower(error) => error.fmt(f),
            Self::NumericLiteralError(error) => error.fmt(f),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct UnexpectedSyntaxError {
    pub expected: SyntaxKind,
    pub actual: Token,
    pub span: Span,
}

impl From<UnexpectedSyntaxError> for ParseExpressionError {
    fn from(value: UnexpectedSyntaxError) -> Self {
        Self::UnexpectedSyntax(value)
    }
}

impl Display for UnexpectedSyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let UnexpectedSyntaxError {
            expected,
            actual,
            span,
        } = self;

        write!(
            f,
            "unexpected syntax at {span}; expected: {expected:?}; actual: {actual:?}",
        )
    }
}

impl Error for UnexpectedSyntaxError {}
