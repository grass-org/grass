mod character_literal;
mod cursor;
mod error;
mod fallback;
mod new_line;
mod numeric_literal;
mod operator;
mod single_character;
mod span_tracker;
mod token;
mod whitespace;

pub use token::*;

use cursor::*;
use error::*;
use fallback::*;
use new_line::*;
use numeric_literal::*;
use operator::*;
use single_character::*;
use span_tracker::*;
use whitespace::*;

use std::iter;
use crate::character_literal::try_lex_character_literal;

type LexResult<T = TokenSpan> = Result<T, LexError>;

pub fn lex(source: &str) -> impl Iterator<Item = TokenSpan> {
    let mut cursor = Cursor::new(source);

    iter::from_fn(move || next_token(&mut cursor))
}

fn next_token(cursor: &mut Cursor) -> Option<TokenSpan> {
    let has_leading_whitespace = skip_whitespaces(cursor) == SkipWhitespaceResult::Skipped;

    try_lex_new_line(cursor)
        .fallback(|| try_lex_single_character(cursor))
        .fallback(|| try_lex_operator(cursor, has_leading_whitespace))
        .fallback(|| try_lex_numeric_literal(cursor))
        .fallback(|| try_lex_character_literal(cursor))
        .ok()
}
