use crate::{
    Cursor, LexError, LexResult, SpanTracker, Token, TokenSpan, is_newline, is_whitespace,
};

pub(crate) fn try_lex_new_line(cursor: &mut Cursor) -> LexResult {
    let start = cursor.peek().ok_or(LexError::EndOfSource)?;

    if !is_newline(start) {
        return Err(LexError::Skipped);
    }

    Ok(lex_new_line(cursor))
}

fn lex_new_line(cursor: &mut Cursor) -> TokenSpan {
    let span_tracker = SpanTracker::start(cursor);
    cursor.advance();

    // We also compound all whitespaces to an already started newline token
    loop {
        let Some(character) = cursor.peek() else {
            break;
        };

        if !is_whitespace(character) {
            break;
        }

        cursor.advance();
    }

    let span = span_tracker.end(cursor);
    TokenSpan::new(Token::NewLine, span)
}

#[cfg(test)]
mod tests {
    use super::try_lex_new_line;
    use crate::{Cursor, LexError, Token, TokenSpan};
    use interfaces::Span;

    #[test]
    fn single_new_line() {
        let mut cursor = Cursor::new("\n");

        let expected = Ok(TokenSpan {
            token: Token::NewLine,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let actual = try_lex_new_line(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn extended_new_line() {
        let mut cursor = Cursor::new("\n        ");

        let expected = Ok(TokenSpan {
            token: Token::NewLine,
            span: Span {
                start: 0,
                length: 9,
            },
        });

        let actual = try_lex_new_line(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_adjacent() {
        let mut cursor = Cursor::new("\n        67");

        let expected = Ok(TokenSpan {
            token: Token::NewLine,
            span: Span {
                start: 0,
                length: 9,
            },
        });

        let actual = try_lex_new_line(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn delayed_start() {
        let mut cursor = Cursor::new(" \n        ");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_new_line(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_start() {
        let mut cursor = Cursor::new("1\n        ");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_new_line(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn did_not_consume_invalid() {
        let mut cursor = Cursor::new("pizza");

        _ = try_lex_new_line(&mut cursor);

        let expected = Some('p');
        let actual = cursor.peek();

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_source() {
        let mut cursor = Cursor::new("");

        let expected = Err(LexError::EndOfSource);
        let actual = try_lex_new_line(&mut cursor);

        assert_eq!(expected, actual);
    }
}
