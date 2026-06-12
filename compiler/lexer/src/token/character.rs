use std::fmt::{self, Display, Formatter};

// TODO: Maybe just return the whole symbol as a string, no matter how long it is
//  so the parser can give a better error like "Character can only store a single character"
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct CharacterLiteral {
    pub symbol: String,
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
