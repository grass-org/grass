use crate::{Cursor, LexError, LexResult, NumericLiteral, SpanTracker, Token, TokenSpan};
use std::mem::take;
use std::result;

pub(super) fn try_lex_numeric_literal(cursor: &mut Cursor) -> LexResult {
    let start = cursor.peek().ok_or(LexError::EndOfSource)?;

    if !start.is_ascii_digit() {
        return Err(LexError::Skipped);
    }

    Ok(lex_numeric_literal(cursor))
}

fn lex_numeric_literal(cursor: &mut Cursor) -> TokenSpan {
    let span_tracker = SpanTracker::start(cursor);
    let literal = lex_literal(cursor);
    span_tracker.create_token_span(cursor, Token::NumericLiteral(literal))
}

fn lex_literal(cursor: &mut Cursor) -> NumericLiteral {
    let mut builder = NumericLiteralBuilder::new();

    loop {
        let Some(character) = cursor.peek() else {
            break;
        };

        let Ok(()) = builder.push(character) else {
            break;
        };

        cursor.advance();
    }

    builder.build()
}

type Result = result::Result<(), ()>;
const OK: Result = Ok(());
const ERR: Result = Err(());

struct NumericLiteralBuilder {
    base: Option<String>,
    integral_radits: String,
    fractional_radits: Option<String>,
    kind: Option<String>,
}

impl NumericLiteralBuilder {
    pub fn new() -> Self {
        Self {
            base: None,
            integral_radits: String::new(),
            fractional_radits: None,
            kind: None,
        }
    }

    pub fn push(&mut self, character: char) -> Result {
        if character == '#' {
            return self.move_symbol_to_base();
        }

        if character == '.' {
            return self.start_fractional();
        }

        if character == ':' {
            return self.start_kind();
        }

        if !character.is_ascii_alphanumeric() {
            return ERR;
        }

        self.push_character(character);
        OK
    }

    fn move_symbol_to_base(&mut self) -> Result {
        if self.base.is_some() || self.fractional_radits.is_some() || self.kind.is_some() {
            return ERR;
        }

        self.base = Some(take(&mut self.integral_radits));
        OK
    }

    fn start_fractional(&mut self) -> Result {
        if self.fractional_radits.is_some() || self.kind.is_some() {
            return ERR;
        }

        self.fractional_radits = Some(String::new());
        OK
    }

    fn start_kind(&mut self) -> Result {
        if self.kind.is_some() {
            return ERR;
        }

        self.kind = Some(String::new());
        OK
    }

    fn push_character(&mut self, character: char) {
        if let Some(kind) = &mut self.kind {
            kind.push(character);
            return;
        };

        if let Some(fractional_radits) = &mut self.fractional_radits {
            fractional_radits.push(character);
            return;
        };

        self.integral_radits.push(character);
    }

    pub fn build(self) -> NumericLiteral {
        NumericLiteral {
            base: self.base,
            integral_radits: self.integral_radits,
            fractional_radits: self.fractional_radits,
            kind: self.kind,
        }
    }
}
