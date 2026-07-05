use interfaces::Span;

use crate::Cursor;

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

    pub const fn end(self, cursor: &Cursor) -> Span {
        let end = cursor.index();

        let start = self.start;
        let length = end - start;

        Span { start, length }
    }
}

#[cfg(test)]
mod tests {
    use interfaces::Span;

    use super::SpanTracker;
    use crate::Cursor;

    #[test]
    fn did_not_move() {
        let cursor = Cursor::new("1234567");
        let span_tracker = SpanTracker::start(&cursor);

        let expected = Span {
            start: 0,
            length: 0,
        };

        let actual = span_tracker.end(&cursor);

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

        let actual = span_tracker.end(&cursor);

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

        let actual = span_tracker.end(&cursor);

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

        let actual = span_tracker.end(&cursor);

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

        let actual = span_tracker.end(&cursor);

        assert_eq!(expected, actual);
    }
}
