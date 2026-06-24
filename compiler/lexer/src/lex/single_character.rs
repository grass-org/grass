use crate::{Cursor, LexError, LexResult, SpanTracker, Token, TokenSpan};

pub(crate) fn try_lex_single_character(cursor: &mut Cursor) -> LexResult {
    let character = cursor.peek().ok_or(LexError::EndOfSource)?;

    let token = lex_single_character(character).ok_or(LexError::Skipped)?;

    let span_tracker = SpanTracker::start(cursor);
    cursor.advance();

    let span = span_tracker.end(cursor);
    Ok(TokenSpan::new(token, span))
}

const fn lex_single_character(character: char) -> Option<Token> {
    let token = match character {
        '(' => Token::OpenParenthesis,
        ')' => Token::CloseParenthesis,
        '[' => Token::OpenBracket,
        ']' => Token::CloseBracket,
        '{' => Token::OpenBrace,
        '}' => Token::CloseBrace,

        ',' => Token::Comma,
        ':' => Token::Colon,

        _ => return None,
    };

    Some(token)
}

#[cfg(test)]
mod tests {
    use super::try_lex_single_character;
    use crate::{Cursor, LexError, Token, TokenSpan};
    use interfaces::Span;

    #[test]
    fn open_parenthesis() {
        let mut cursor = Cursor::new("(");

        let expected = Ok(TokenSpan {
            token: Token::OpenParenthesis,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let actual = try_lex_single_character(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn colon() {
        let mut cursor = Cursor::new(":");

        let expected = Ok(TokenSpan {
            token: Token::Colon,
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let actual = try_lex_single_character(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn delayed_start() {
        let mut cursor = Cursor::new(" {");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_single_character(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_start() {
        let mut cursor = Cursor::new("A{");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_single_character(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn did_not_consume_invalid() {
        let mut cursor = Cursor::new("pizza");

        _ = try_lex_single_character(&mut cursor);

        let expected = Some('p');
        let actual = cursor.peek();

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_source() {
        let mut cursor = Cursor::new("");

        let expected = Err(LexError::EndOfSource);
        let actual = try_lex_single_character(&mut cursor);

        assert_eq!(expected, actual);
    }
}
