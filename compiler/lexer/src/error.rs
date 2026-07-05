use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub(super) enum LexError {
    Skipped,
    EndOfSource,
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LexError::Skipped => write!(f, "lexer skipped current character"),
            LexError::EndOfSource => write!(f, "lexer reached end of source"),
        }
    }
}

impl Error for LexError {}
