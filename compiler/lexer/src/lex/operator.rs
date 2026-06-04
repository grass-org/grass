use crate::{
    is_horizontal_whitespace, Cursor, LexError, LexResult, Operator, SpanTracker, Token, TokenSpan,
};

pub(crate) fn try_lex_operator(cursor: &mut Cursor, has_leading_whitespace: bool) -> LexResult {
    let start = cursor.peek().ok_or(LexError::EndOfSource)?;

    if !is_operator_character(start) {
        return Err(LexError::Skipped);
    }

    Ok(lex_operator(cursor, has_leading_whitespace))
}

fn lex_operator(cursor: &mut Cursor, has_leading_whitespace: bool) -> TokenSpan {
    let span_tracker = SpanTracker::start(cursor);

    let symbol = lex_symbol(cursor);
    let has_trailing_whitespace = has_trailing_whitespace(cursor);

    let operator = Operator {
        symbol,
        has_leading_whitespace,
        has_trailing_whitespace,
    };

    span_tracker.create_token_span(cursor, Token::Operator(operator))
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
