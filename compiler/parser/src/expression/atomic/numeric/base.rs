use crate::NumericLiteralError;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub(super) fn base(string: Option<String>) -> Result<u8, InvalidNumericBaseError> {
    let Some(string) = string else {
        return Ok(10);
    };

    let base = match str::parse(&string) {
        Ok(base) => base,
        Err(_) => {
            return Err(base_error(string));
        }
    };

    if base > 36 {
        return Err(base_error(string));
    }

    Ok(base)
}

fn base_error(base: String) -> InvalidNumericBaseError {
    InvalidNumericBaseError { base, max_base: 36 }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct InvalidNumericBaseError {
    pub base: String,
    pub max_base: u8,
}

impl From<InvalidNumericBaseError> for NumericLiteralError {
    fn from(value: InvalidNumericBaseError) -> Self {
        Self::InvalidBase(value)
    }
}

impl Display for InvalidNumericBaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let InvalidNumericBaseError { base, max_base } = self;

        write!(
            f,
            "invalid literal literal base {base}; \
            literal literals can only support bases from 2 to {max_base}"
        )
    }
}

impl Error for InvalidNumericBaseError {}
