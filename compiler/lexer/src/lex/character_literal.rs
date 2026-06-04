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
