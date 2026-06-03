use crate::{Cursor, LexError, LexResult, SpanTracker, Token};

pub(super) fn try_lex_single_character(cursor: &mut Cursor) -> LexResult {
    let character = cursor.peek().ok_or(LexError::EndOfSource)?;

    let token = lex_single_character(character).ok_or(LexError::Skipped)?;

    let span_tracker = SpanTracker::start(cursor);
    cursor.advance();

    Ok(span_tracker.create_token_span(cursor, token))
}

fn lex_single_character(character: char) -> Option<Token> {
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
