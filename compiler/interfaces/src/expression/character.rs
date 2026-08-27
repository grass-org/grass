use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::AtomicExpression;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct CharacterLiteral(pub char);

impl From<CharacterLiteral> for AtomicExpression {
    fn from(value: CharacterLiteral) -> Self {
        Self::CharacterLiteral(value)
    }
}

impl Display for CharacterLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.0)
    }
}
