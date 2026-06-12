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

    let token = lex_literal(cursor)
        .map(|literal| {
            let symbol = literal.symbol;

            if symbol.is_empty() {
                return Token::Invalid;
            }

            if literal.escaped {
                Token::CharacterLiteral(symbol)
            } else {
                if symbol.chars().count() == 1 {
                    Token::CharacterLiteral(symbol)
                } else {
                    Token::Invalid 
                }
            }
        })
        .unwrap_or_default();

    span_tracker.create_token_span(cursor, token)
}


fn lex_literal(cursor: &mut Cursor) -> Option<CharacterLiteral> {
    let open_quote = cursor.pop()?;
    if open_quote != '\'' {
        return None;
    }

    let mut symbol_buffer = String::new();
    let mut has_escapes = false;

    while let Some(ch) = cursor.pop() {
        if ch == '\\' {
            if let Some(escaped_char) = cursor.pop() {
                symbol_buffer.push('\\');
                symbol_buffer.push(escaped_char);
                has_escapes = true;
            } else {
                return None; 
            }
        } else if ch == '\'' {
            return Some(CharacterLiteral {
                symbol: symbol_buffer,
                escaped: has_escapes,
            });
        } else {
            symbol_buffer.push(ch);
        }
    }

    None 
}

fn close_character(cursor: &mut Cursor, mut symbol:String, escaped: bool) -> Option<CharacterLiteral> {
    let end = cursor.pop()?;

    if end != '\'' {
        return None;
    }

    symbol.push('\'');

    Some(CharacterLiteral { symbol, escaped })
}

#[cfg(test)]
mod tests {
    use super::try_lex_character_literal;
    use crate::{Cursor, LexError, Token, TokenSpan};
    use interfaces::Span;

    #[test]
    fn unescaped() {
        let mut cursor = Cursor::new("'a'");
        
        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(String::from("a")),
            span: Span {
                start: 0,
                length: 3,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn escaped() {
        let mut cursor = Cursor::new(r#"'\a'"#);

        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(r#"\a"#.to_string()),
            span: Span {
                start: 0,
                length: 4,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn whitespace() {
        let mut cursor = Cursor::new("' '");

        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(String::from(" ")),
            span: Span {
                start: 0,
                length: 3,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_adjacent() {
        let mut cursor = Cursor::new("' '67");

        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(" ".to_string()),
            span: Span {
                start: 0,
                length: 3,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty() {
        let mut cursor = Cursor::new("''");

        let expected = Ok(TokenSpan {
            token: Token::Invalid,
            span: Span {
                start: 0,
                length: 2,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn too_long() {
        let mut cursor = Cursor::new("'a1'");

        let expected = Ok(TokenSpan {
            token: Token::Invalid,
            span: Span {
                start: 0,
                length: 4,
            },
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn delayed_start() {
        let mut cursor = Cursor::new(" 'a'");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_start() {
        let mut cursor = Cursor::new("pizza");

        let expected = Err(LexError::Skipped);
        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn did_not_consume_invalid() {
        let mut cursor = Cursor::new("pizza");

        _ = try_lex_character_literal(&mut cursor);

        let expected = Some('p');
        let actual = cursor.peek();

        assert_eq!(expected, actual);
    }

    #[test]
    fn end_of_source() {
        let mut cursor = Cursor::new("");

        let expected = Err(LexError::EndOfSource);
        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lex_character_literal_complex_sequence() {
        let mut cursor = Cursor::new(r#"'a\\""fhh27\'67'"#);

        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(String::from(r#"a\\""fhh27\'67"#)),
            span: Span { 
                start: 0, 
                length: 16,
            }
        });

        let actual = try_lex_character_literal(&mut cursor);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multi_byte_utf8_character() {
        let mut cursor = Cursor::new("'✨'");
        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(String::from("✨")),
            span: Span { 
                start: 0, 
                length: 5 
            }, 
        });

        assert_eq!(expected, try_lex_character_literal(&mut cursor));
    }

    #[test]
    fn test_unclosed_literal_returns_invalid() {
        let mut cursor = Cursor::new("'a");
        let expected = Ok(TokenSpan {
            token: Token::Invalid,
            span: Span { 
                start: 0, 
                length: 2 
            },
        });

        assert_eq!(expected, try_lex_character_literal(&mut cursor));
    }

    #[test]
    fn test_escaped_backslash_before_closing_quote() {
        let mut cursor = Cursor::new(r#"'\\'"#);
        let expected = Ok(TokenSpan {
            token: Token::CharacterLiteral(String::from(r#"\\"#)),
            span: Span { 
                start: 0, 
                length: 4 
            },
        });
        assert_eq!(expected, try_lex_character_literal(&mut cursor));
    }
  
}
