mod cursor;
mod error;
mod fallback;
mod lex;
mod span_tracker;
mod token;
mod whitespace;

use std::iter;
use std::result;

use cursor::*;
use error::*;
use fallback::*;
use interfaces::Spanned;
use lex::*;
use span_tracker::*;
pub use token::*;
use whitespace::*;

type Error = LexError;
type Result<T = Spanned<Token>> = result::Result<T, Error>;

pub fn lex(source: &str) -> impl Iterator<Item = Spanned<Token>> {
    let mut cursor = Cursor::new(source);

    iter::from_fn(move || next_token(&mut cursor))
}

fn next_token(cursor: &mut Cursor) -> Option<Spanned<Token>> {
    let has_leading_whitespace = skip_whitespaces(cursor) == SkipWhitespaceResult::Skipped;

    try_lex_new_line(cursor)
        .fallback(|| try_lex_single_character(cursor))
        .fallback(|| try_lex_operator(cursor, has_leading_whitespace))
        .fallback(|| try_lex_numeric_literal(cursor))
        .fallback(|| try_lex_character_literal(cursor))
        .ok()
}

// TODO: TEST!
