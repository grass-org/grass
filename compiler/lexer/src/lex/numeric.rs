use crate::{Cursor, LexError, LexResult, NumericLiteral, SpanTracker, Token, TokenSpan};
use std::mem::take;
use std::result;

pub(crate) fn try_lex_numeric_literal(cursor: &mut Cursor) -> LexResult {
    let start = cursor.peek().ok_or(LexError::EndOfSource)?;

    if !start.is_ascii_digit() {
        return Err(LexError::Skipped);
    }

    Ok(lex_numeric_literal(cursor))
}

fn lex_numeric_literal(cursor: &mut Cursor) -> TokenSpan {
    let span_tracker = SpanTracker::start(cursor);
    let literal = lex_literal(cursor);
    span_tracker.create_token_span(cursor, literal)
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
    pub const fn new() -> Self {
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

#[cfg(test)]
mod tests {
    use super::try_lex_numeric_literal;
    use crate::{Cursor, LexError, NumericLiteral, Token, TokenSpan};
    use interfaces::Span;

    #[test]
    fn basic_integer() {
        let mut cursor = Cursor::new("365");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: None,
                integral_radits: String::from("365"),
                fractional_radits: None,
                kind: None,
            }),
            span: Span {
                start: 0,
                length: 3,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn basic_fraction() {
        let mut cursor = Cursor::new("365.67");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: None,
                integral_radits: String::from("365"),
                fractional_radits: Some(String::from("67")),
                kind: None,
            }),
            span: Span {
                start: 0,
                length: 6,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn integer_with_base() {
        let mut cursor = Cursor::new("67#365");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: Some(String::from("67")),
                integral_radits: String::from("365"),
                fractional_radits: None,
                kind: None,
            }),
            span: Span {
                start: 0,
                length: 6,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn fraction_with_base() {
        let mut cursor = Cursor::new("67#365.69");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: Some(String::from("67")),
                integral_radits: String::from("365"),
                fractional_radits: Some(String::from("69")),
                kind: None,
            }),
            span: Span {
                start: 0,
                length: 9,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn integer_with_kind() {
        let mut cursor = Cursor::new("369:I32");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: None,
                integral_radits: String::from("369"),
                fractional_radits: None,
                kind: Some(String::from("I32")),
            }),
            span: Span {
                start: 0,
                length: 7,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn fraction_with_kind() {
        let mut cursor = Cursor::new("369.67:F64");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: None,
                integral_radits: String::from("369"),
                fractional_radits: Some(String::from("67")),
                kind: Some(String::from("F64")),
            }),
            span: Span {
                start: 0,
                length: 10,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn integer_with_base_and_type() {
        let mut cursor = Cursor::new("2#369:I32");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: Some(String::from("2")),
                integral_radits: String::from("369"),
                fractional_radits: None,
                kind: Some(String::from("I32")),
            }),
            span: Span {
                start: 0,
                length: 9,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn fraction_with_base_and_type() {
        let mut cursor = Cursor::new("2#369.67:F64");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: Some(String::from("2")),
                integral_radits: String::from("369"),
                fractional_radits: Some(String::from("67")),
                kind: Some(String::from("F64")),
            }),
            span: Span {
                start: 0,
                length: 12,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_adjacent() {
        let mut cursor = Cursor::new("2#369:F64 35");

        let expected = Ok(TokenSpan {
            token: Token::NumericLiteral(NumericLiteral {
                base: Some(String::from("2")),
                integral_radits: String::from("369"),
                fractional_radits: None,
                kind: Some(String::from("F64")),
            }),
            span: Span {
                start: 0,
                length: 9,
            },
        });

        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn delayed_start() {
        let mut cursor = Cursor::new(" 2#369.67:F64 35");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_start() {
        let mut cursor = Cursor::new("H2#369.67:F64 35");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn did_not_consume_invalid() {
        let mut cursor = Cursor::new("pizza");

        _ = try_lex_numeric_literal(&mut cursor);

        let expected = Some('p');
        let actual = cursor.peek();

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_source() {
        let mut cursor = Cursor::new("");

        let expected = Err(LexError::EndOfSource);
        let actual = try_lex_numeric_literal(&mut cursor);

        assert_eq!(expected, actual);
    }
}
