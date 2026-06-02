use crate::NumericLiteralError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum AtomicExpressionError {
    NumericLiteralError(NumericLiteralError),
    UnknownError, // TODO: This shouldn't be a thing
}
