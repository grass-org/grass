use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use interfaces::Spanned;

use super::format_slice;
use crate::Token;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum StringLiteralPart {
    Characters(String),
    Interpolation(Vec<Spanned<Token>>),
}

impl Display for StringLiteralPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StringLiteralPart::Characters(characters) => characters.fmt(f),
            StringLiteralPart::Interpolation(tokens) => format_slice(tokens, f),
        }
    }
}
