use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct CharacterLiteral {
    pub symbol: char,
    pub escaped: bool,
}

impl Display for CharacterLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let CharacterLiteral { symbol, escaped } = self;

        write!(f, "'")?;

        if *escaped {
            write!(f, "\\")?;
        }

        write!(f, "{symbol}")
    }
}
