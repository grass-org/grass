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
