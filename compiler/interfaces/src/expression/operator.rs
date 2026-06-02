use crate::Span;
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct OperatorSpan {
    pub operator: Operator,
    pub span: Span,
}

impl Display for OperatorSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.operator.fmt(f)
    }
}

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

// TODO: These are temporarily hardcoded, at least until we implement Grass operators
impl Operator {
    pub fn add() -> Self {
        Self {
            symbol: String::from("+"),
        }
    }

    pub fn subtract() -> Self {
        Self {
            symbol: String::from("-"),
        }
    }

    pub fn multiply() -> Self {
        Self {
            symbol: String::from("*"),
        }
    }

    pub fn divide() -> Self {
        Self {
            symbol: String::from("/"),
        }
    }

    pub fn remainder() -> Self {
        Self {
            symbol: String::from("%"),
        }
    }
}
