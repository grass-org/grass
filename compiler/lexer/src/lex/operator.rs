use interfaces::Spanned;

use crate::Cursor;
use crate::Error;
use crate::Operator;
use crate::Result;
use crate::SpanTracker;
use crate::Token;
use crate::is_horizontal_whitespace;

pub(crate) fn try_lex_operator(cursor: &mut Cursor, has_leading_whitespace: bool) -> Result {
    let start = cursor.peek().ok_or(Error::EndOfSource)?;

    if !is_operator_character(start) {
        return Err(Error::Skipped);
    }

    Ok(lex_operator(cursor, has_leading_whitespace))
}

fn lex_operator(cursor: &mut Cursor, has_leading_whitespace: bool) -> Spanned<Token> {
    let span_tracker = SpanTracker::start(cursor);

    let symbol = lex_symbol(cursor);
    let has_trailing_whitespace = has_trailing_whitespace(cursor);

    let operator = Operator {
        symbol,
        has_leading_whitespace,
        has_trailing_whitespace,
    };

    let span = span_tracker.end(cursor);
    Spanned::new(operator, span)
}

fn lex_symbol(cursor: &mut Cursor) -> String {
    let mut symbol = String::new();

    loop {
        let Some(character) = cursor.peek() else {
            break;
        };

        if !is_operator_character(character) {
            break;
        }

        symbol.push(character);
        cursor.advance();
    }

    symbol
}

const fn is_operator_character(character: char) -> bool {
    match character {
        '(' | ')' | '[' | ']' | '{' | '}' | ',' | ':' => false,
        _ => !character.is_ascii_alphanumeric() && !character.is_whitespace(),
    }
}

fn has_trailing_whitespace(cursor: &mut Cursor) -> bool {
    let Some(next_character) = cursor.peek() else {
        return false;
    };

    is_horizontal_whitespace(next_character)
}

#[cfg(test)]
mod tests {
    use interfaces::Span;
    use interfaces::Spanned;

    use super::try_lex_operator;
    use crate::Cursor;
    use crate::Error;
    use crate::Operator;
    use crate::Token;

    #[test]
    fn single_character() {
        let mut cursor = Cursor::new("+");

        let expected = Ok(Spanned {
            content: Token::Operator(Operator {
                symbol: "+".into(),
                has_leading_whitespace: false,
                has_trailing_whitespace: false,
            }),
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let actual = try_lex_operator(&mut cursor, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_character() {
        let mut cursor = Cursor::new("+=");

        let expected = Ok(Spanned {
            content: Token::Operator(Operator {
                symbol: "+=".into(),
                has_leading_whitespace: false,
                has_trailing_whitespace: false,
            }),
            span: Span {
                start: 0,
                length: 2,
            },
        });

        let actual = try_lex_operator(&mut cursor, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_leading_whitespace() {
        let mut cursor = Cursor::new("+=");

        let expected = Ok(Spanned {
            content: Token::Operator(Operator {
                symbol: "+=".into(),
                has_leading_whitespace: true,
                has_trailing_whitespace: false,
            }),
            span: Span {
                start: 0,
                length: 2,
            },
        });

        let actual = try_lex_operator(&mut cursor, true);

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_trailing_whitespace() {
        let mut cursor = Cursor::new("+= ");

        let expected = Ok(Spanned {
            content: Token::Operator(Operator {
                symbol: "+=".into(),
                has_leading_whitespace: false,
                has_trailing_whitespace: true,
            }),
            span: Span {
                start: 0,
                length: 2,
            },
        });

        let actual = try_lex_operator(&mut cursor, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_leading_and_trailing_whitespace() {
        let mut cursor = Cursor::new("+= ");

        let expected = Ok(Spanned {
            content: Token::Operator(Operator {
                symbol: "+=".into(),
                has_leading_whitespace: true,
                has_trailing_whitespace: true,
            }),
            span: Span {
                start: 0,
                length: 2,
            },
        });

        let actual = try_lex_operator(&mut cursor, true);

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_adjacent() {
        let mut cursor = Cursor::new("-35");

        let expected = Ok(Spanned {
            content: Token::Operator(Operator {
                symbol: "-".into(),
                has_leading_whitespace: false,
                has_trailing_whitespace: false,
            }),
            span: Span {
                start: 0,
                length: 1,
            },
        });

        let actual = try_lex_operator(&mut cursor, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn delayed_start() {
        let mut cursor = Cursor::new(" +");

        let expected = Err(Error::Skipped);
        let actual = try_lex_operator(&mut cursor, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_start() {
        let mut cursor = Cursor::new("6+");

        let expected = Err(Error::Skipped);
        let actual = try_lex_operator(&mut cursor, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn did_not_consume_invalid() {
        let mut cursor = Cursor::new("pizza");

        _ = try_lex_operator(&mut cursor, false);

        let expected = Some('p');
        let actual = cursor.peek();

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_source() {
        let mut cursor = Cursor::new("");

        let expected = Err(Error::EndOfSource);
        let actual = try_lex_operator(&mut cursor, false);

        assert_eq!(expected, actual);
    }
}
