use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::UndefinedBindingPowerError;
use interfaces::{Operator, Span, SyntaxKind};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum ParseExpressionError {
    NoMoreTokens,
    UnexpectedSyntax(UnexpectedSyntaxError),
    UndefinedBindingPower(UndefinedBindingPowerError),
}

impl ParseExpressionError {
    pub const fn unexpected_syntax(expected: SyntaxKind, span: Span) -> Self {
        Self::UnexpectedSyntax(UnexpectedSyntaxError { expected, span })
    }

    pub const fn undefined_binding_power(operator: Operator) -> Self {
        Self::UndefinedBindingPower(UndefinedBindingPowerError { operator })
    }
}

impl Error for ParseExpressionError {}

impl Display for ParseExpressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMoreTokens => write!(f, "no more tokens to parse"),
            Self::UnexpectedSyntax(error) => error.fmt(f),
            Self::UndefinedBindingPower(error) => error.fmt(f),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct UnexpectedSyntaxError {
    pub expected: SyntaxKind,
    pub span: Span,
}

impl From<UnexpectedSyntaxError> for ParseExpressionError {
    fn from(value: UnexpectedSyntaxError) -> Self {
        Self::UnexpectedSyntax(value)
    }
}

impl Display for UnexpectedSyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unexpected syntax at {}; expected: {:?}",
            self.span, self.expected
        )
    }
}

impl Error for UnexpectedSyntaxError {}
