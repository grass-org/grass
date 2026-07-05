use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::Token;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct Operator {
    pub symbol: String,
    pub has_leading_whitespace: bool,
    pub has_trailing_whitespace: bool,
}

impl From<Operator> for Token {
    fn from(value: Operator) -> Self {
        Token::Operator(value)
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Operator {
            symbol,
            has_leading_whitespace: has_leading_space,
            has_trailing_whitespace: has_trailing_space,
        } = self;

        if !has_leading_space {
            write!(f, " ")?;
        }

        write!(f, "{symbol}")?;

        if !has_trailing_space {
            write!(f, " ")?;
        }

        Ok(())
    }
}
