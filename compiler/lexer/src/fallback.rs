use crate::{LexError, LexResult};

pub(super) trait Fallback {
    fn fallback(self, function: impl FnOnce() -> LexResult) -> LexResult;
}

impl Fallback for LexResult {
    fn fallback(self, function: impl FnOnce() -> LexResult) -> LexResult {
        match self {
            ok @ Ok(_) => ok,
            Err(error) => fallback(error, function),
        }
    }
}

fn fallback(error: LexError, function: impl FnOnce() -> LexResult) -> LexResult {
    match error {
        LexError::Skipped => function(),
        error @ LexError::EndOfSource => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::Fallback;
    use crate::{LexError, Token, TokenSpan};
    use interfaces::Span;

    #[test]
    fn prefer_primary() {
        let primary = Ok(TokenSpan {
            token: Token::Comma,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let secondary = Ok(TokenSpan {
            token: Token::Colon,
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
        let secondary = Ok(TokenSpan {
            token: Token::Colon,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let expected = secondary.clone();
        let actual = Err(LexError::Skipped).fallback(|| secondary);

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_source() {
        let secondary = Ok(TokenSpan {
            token: Token::Colon,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let expected = Err(LexError::EndOfSource);
        let actual = Err(LexError::EndOfSource).fallback(|| secondary);

        assert_eq!(expected, actual);
    }

    #[test]
    fn skip_fallback_end_of_source() {
        let secondary = Ok(TokenSpan {
            token: Token::Colon,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let mut called_fallback = false;

        _ = Err(LexError::EndOfSource).fallback(|| {
            called_fallback = true;
            secondary
        });

        assert!(!called_fallback);
    }
}
