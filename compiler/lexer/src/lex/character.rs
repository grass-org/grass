use crate::{Cursor, Error, Result, SpanTracker, Token};
use interfaces::Spanned;

pub(crate) fn try_lex_character_literal(cursor: &mut Cursor) -> Result {
    let start = cursor.peek().ok_or(Error::EndOfSource)?;

    if start != '\'' {
        return Err(Error::Skipped);
    }

    Ok(lex_character_literal(cursor))
}

fn lex_character_literal(cursor: &mut Cursor) -> Spanned<Token> {
    let span_tracker = SpanTracker::start(cursor);

    cursor.advance();

    let token = lex_literal(cursor);
    let span = span_tracker.end(cursor);
    Spanned::new(token, span)
}

fn lex_literal(cursor: &mut Cursor) -> Token {
    match lex_symbol(cursor) {
        None => Token::Invalid,
        Some(symbol) => Token::CharacterLiteral(symbol),
    }
}

fn lex_symbol(cursor: &mut Cursor) -> Option<String> {
    let mut symbol = String::new();

    loop {
        let character = cursor.pop()?;

        if character == '\'' {
            break;
        }

        if character != '\\' {
            symbol.push(character);

            continue;
        }

        let next_character = cursor.pop()?;

        symbol.push(character);
        symbol.push(next_character);
    }

    Some(symbol)
}

#[cfg(test)]
mod tests {
    use super::try_lex_character_literal;
    use crate::{Cursor, Error, Token};
    use interfaces::{Span, Spanned};

    #[test]
    fn unescaped() {
        let mut cursor = Cursor::new("'a'");

        let expected = Ok(Spanned {
            content: Token::CharacterLiteral("a".into()),
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

        let expected = Ok(Spanned {
            content: Token::CharacterLiteral(r#"\a"#.to_string()),
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

        let expected = Ok(Spanned {
            content: Token::CharacterLiteral(" ".into()),
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

        let expected = Ok(Spanned {
            content: Token::CharacterLiteral(" ".to_string()),
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

        let expected = Ok(Spanned {
            content: Token::CharacterLiteral("".into()),
            span: Span {
                start: 0,
                length: 2,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn long() {
        let mut cursor = Cursor::new("'a1'");

        let expected = Ok(Spanned {
            content: Token::CharacterLiteral("a1".into()),
            span: Span {
                start: 0,
                length: 4,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn delayed_start() {
        let mut cursor = Cursor::new(" 'a'");

        let expected = Err(Error::Skipped);
        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_start() {
        let mut cursor = Cursor::new("pizza");

        let expected = Err(Error::Skipped);
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

        let expected = Err(Error::EndOfSource);
        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn complex_sequence() {
        let mut cursor = Cursor::new(r#"'a\\""fhh27\'67'"#);

        let expected = Ok(Spanned {
            content: Token::CharacterLiteral(r#"a\\""fhh27\'67"#.into()),
            span: Span {
                start: 0,
                length: 16,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_byte_utf8_character() {
        let mut cursor = Cursor::new("'✨'");
        let expected = Ok(Spanned {
            content: Token::CharacterLiteral("✨".into()),
            span: Span {
                start: 0,
                length: 5,
            },
        });

        assert_eq!(expected, try_lex_character_literal(&mut cursor));
    }

    #[test]
    fn unclosed() {
        let mut cursor = Cursor::new("'a");
        let expected = Ok(Spanned {
            content: Token::Invalid,
            span: Span {
                start: 0,
                length: 2,
            },
        });

        assert_eq!(expected, try_lex_character_literal(&mut cursor));
    }

    #[test]
    fn escaped_backslash_before_closing_quote() {
        let mut cursor = Cursor::new(r#"'\\'"#);
        let expected = Ok(Spanned {
            content: Token::CharacterLiteral(r#"\\"#.into()),
            span: Span {
                start: 0,
                length: 4,
            },
        });
        assert_eq!(expected, try_lex_character_literal(&mut cursor));
    }
}
