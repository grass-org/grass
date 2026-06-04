use crate::{Cursor, Token, TokenSpan};
use interfaces::Span;

#[derive(Debug)]
pub(super) struct SpanTracker {
    start: usize,
}

impl SpanTracker {
    pub const fn start(cursor: &Cursor) -> Self {
        Self {
            start: cursor.index(),
        }
    }

    pub const fn create_token_span(self, cursor: &Cursor, token: Token) -> TokenSpan {
        let span = self.end(cursor);
        TokenSpan { token, span }
    }

    const fn end(self, cursor: &Cursor) -> Span {
        let end = cursor.index();

        let start = self.start;
        let length = end - start;

        Span { start, length }
    }
}

#[cfg(test)]
mod tests {
    use super::SpanTracker;
    use crate::{Cursor, Token};
    use interfaces::Span;

    #[test]
    fn matches_token() {
        let cursor = Cursor::new("1234567");
        let span_tracker = SpanTracker::start(&cursor);
        let dummy_token = Token::Invalid;

        let expected = dummy_token.clone();
        let actual = span_tracker.create_token_span(&cursor, dummy_token).token;

        assert_eq!(expected, actual);
    }

    #[test]
    fn did_not_move() {
        let cursor = Cursor::new("1234567");
        let span_tracker = SpanTracker::start(&cursor);

        let expected = Span {
            start: 0,
            length: 0,
        };

        let actual = span_tracker.create_token_span(&cursor, Token::Invalid).span;

        assert_eq!(expected, actual);
    }

    #[test]
    fn moved_by_1() {
        let mut cursor = Cursor::new("1234567");
        let span_tracker = SpanTracker::start(&cursor);

        cursor.advance();

        let expected = Span {
            start: 0,
            length: 1,
        };

        let actual = span_tracker.create_token_span(&cursor, Token::Invalid).span;

        assert_eq!(expected, actual);
    }

    #[test]
    fn moved_by_3() {
        let mut cursor = Cursor::new("1234567");
        let span_tracker = SpanTracker::start(&cursor);

        cursor.advance();
        cursor.advance();
        cursor.advance();

        let expected = Span {
            start: 0,
            length: 3,
        };

        let actual = span_tracker.create_token_span(&cursor, Token::Invalid).span;

        assert_eq!(expected, actual);
    }

    #[test]
    fn mid_game() {
        let mut cursor = Cursor::new("1234567");

        cursor.advance();
        cursor.advance();

        let span_tracker = SpanTracker::start(&cursor);

        cursor.advance();
        cursor.advance();

        let expected = Span {
            start: 2,
            length: 2,
        };

        let actual = span_tracker.create_token_span(&cursor, Token::Invalid).span;

        assert_eq!(expected, actual);
    }

    #[test]
    fn did_not_overcount() {
        let mut cursor = Cursor::new("123");
        let span_tracker = SpanTracker::start(&cursor);

        cursor.advance();
        cursor.advance();
        cursor.advance();
        cursor.advance();
        cursor.advance();
        cursor.advance();
        cursor.advance();

        let expected = Span {
            start: 0,
            length: 3,
        };

        let actual = span_tracker.create_token_span(&cursor, Token::Invalid).span;

        assert_eq!(expected, actual);
    }
}
