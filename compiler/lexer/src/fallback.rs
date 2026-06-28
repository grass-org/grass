use crate::{Error, Result};

pub(super) trait Fallback {
    fn fallback(self, function: impl FnOnce() -> Result) -> Result;
}

impl Fallback for Result {
    fn fallback(self, function: impl FnOnce() -> Result) -> Result {
        match self {
            ok @ Ok(_) => ok,
            Err(error) => fallback(error, function),
        }
    }
}

fn fallback(error: Error, function: impl FnOnce() -> Result) -> Result {
    match error {
        Error::Skipped => function(),
        error @ Error::EndOfSource => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::Fallback;
    use crate::{Error, Token};
    use interfaces::{Span, Spanned};

    #[test]
    fn prefer_primary() {
        let primary = Ok(Spanned {
            content: Token::Comma,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let secondary = Ok(Spanned {
            content: Token::Colon,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let expected = primary.clone();
        let actual = primary.fallback(|| secondary);

        assert_eq!(expected, actual);
    }

    #[test]
    fn fallback() {
        let secondary = Ok(Spanned {
            content: Token::Colon,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let expected = secondary.clone();
        let actual = Err(Error::Skipped).fallback(|| secondary);

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_source() {
        let secondary = Ok(Spanned {
            content: Token::Colon,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let expected = Err(Error::EndOfSource);
        let actual = Err(Error::EndOfSource).fallback(|| secondary);

        assert_eq!(expected, actual);
    }

    #[test]
    fn skip_fallback_end_of_source() {
        let secondary = Ok(Spanned {
            content: Token::Colon,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let mut called_fallback = false;

        _ = Err(Error::EndOfSource).fallback(|| {
            called_fallback = true;
            secondary
        });

        assert!(!called_fallback);
    }
}
