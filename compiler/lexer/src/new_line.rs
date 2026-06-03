use crate::{is_newline, is_whitespace, Cursor, LexError, LexResult, SpanTracker, Token, TokenSpan};

pub(super) fn try_lex_new_line(cursor: &mut Cursor) -> LexResult {
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

    span_tracker.create_token_span(cursor, Token::NewLine)
}
