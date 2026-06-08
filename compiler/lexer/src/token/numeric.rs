use std::fmt;
use std::fmt::{Display, Formatter};
use crate::Token;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct NumericLiteral {
    pub base: Option<String>,
    pub integral_radits: String,
    pub fractional_radits: Option<String>,
    pub kind: Option<String>,
}

impl From<NumericLiteral> for Token {
    fn from(value: NumericLiteral) -> Self {
        Token::NumericLiteral(value)
    }
}

impl Display for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let NumericLiteral {
            base,
            integral_radits,
            fractional_radits,
            kind,
        } = self;

        if let Some(base) = base {
            write!(f, "{base}#")?;
        };

        write!(f, "{integral_radits}")?;

        if let Some(fractional_radits) = fractional_radits {
            write!(f, ".{fractional_radits}")?;
        };

        if let Some(kind) = kind {
            write!(f, ":{kind}")?;
        };

        Ok(())
    }
}
