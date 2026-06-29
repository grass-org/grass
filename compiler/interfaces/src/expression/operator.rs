use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct Operator {
    pub symbol: String,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = &self.symbol;
        write!(f, "{symbol}")
    }
}
