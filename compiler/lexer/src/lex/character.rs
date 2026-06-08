use crate::{CharacterLiteral, Cursor, LexError, LexResult, SpanTracker, Token, TokenSpan};

pub(crate) fn try_lex_character_literal(cursor: &mut Cursor) -> LexResult {
    let start = cursor.peek().ok_or(LexError::EndOfSource)?;

    if start != '\'' {
        return Err(LexError::Skipped);
    }

    Ok(lex_character_literal(cursor))
}

fn lex_character_literal(cursor: &mut Cursor) -> TokenSpan {
    let span_tracker = SpanTracker::start(cursor);

    cursor.advance();

    let token = lex_literal(cursor)
        .map(Token::CharacterLiteral)
        .unwrap_or_default();

    span_tracker.create_token_span(cursor, token)
}

fn lex_literal(cursor: &mut Cursor) -> Option<CharacterLiteral> {
    let symbol = cursor.pop()?;

    if symbol != '\\' {
        let escaped = false;
        return close_character(cursor, symbol, escaped);
    }

    let symbol = cursor.pop()?;
    let escaped = true;

    close_character(cursor, symbol, escaped)
}

fn close_character(cursor: &mut Cursor, symbol: char, escaped: bool) -> Option<CharacterLiteral> {
    let end = cursor.pop()?;

    if end != '\'' {
        return None;
    }

    Some(CharacterLiteral { symbol, escaped })
}

#[cfg(test)]
mod tests {
    use super::try_lex_character_literal;
    use crate::{CharacterLiteral, Cursor, LexError, Token, TokenSpan};
    use interfaces::Span;

    #[test]
    fn unescaped() {
        let mut cursor = Cursor::new("'a'");

        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(CharacterLiteral {
                symbol: 'a',
                escaped: false,
            }),
            span: Span {
                start: 0,
                length: 3,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn escaped() {
        let mut cursor = Cursor::new(r#"'\a'"#);

        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(CharacterLiteral {
                symbol: 'a',
                escaped: true,
            }),
            span: Span {
                start: 0,
                length: 4,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn whitespace() {
        let mut cursor = Cursor::new("' '");

        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(CharacterLiteral {
                symbol: ' ',
                escaped: false,
            }),
            span: Span {
                start: 0,
                length: 3,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_adjacent() {
        let mut cursor = Cursor::new("' '67");

        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(CharacterLiteral {
                symbol: ' ',
                escaped: false,
            }),
            span: Span {
                start: 0,
                length: 3,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty() {
        let mut cursor = Cursor::new("''");

        let expected = Ok(TokenSpan {
            token: Token::Invalid,
            span: Span {
                start: 0,
                length: 2,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn too_long() {
        let mut cursor = Cursor::new("'a1'");

        let expected = Ok(TokenSpan {
            token: Token::Invalid,
            span: Span {
                start: 0,
                length: 3,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn delayed_start() {
        let mut cursor = Cursor::new(" 'a'");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_start() {
        let mut cursor = Cursor::new("pizza");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn did_not_consume_invalid() {
        let mut cursor = Cursor::new("pizza");

        _ = try_lex_character_literal(&mut cursor);

        let expected = Some('p');
        let actual = cursor.peek();

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_source() {
        let mut cursor = Cursor::new("");

        let expected = Err(LexError::EndOfSource);
        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }
}
