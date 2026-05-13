use crate::{Cursor, NextToken, Tokenizer};

#[derive(Debug)]
pub(crate) struct CompositeTokenizer<A, B>
where
    A: Tokenizer,
    B: Tokenizer,
{
    primary: A,
    secondary: B,
}

impl<A, B> Tokenizer for CompositeTokenizer<A, B>
where
    A: Tokenizer,
    B: Tokenizer,
{
    fn next_token<'source>(&mut self, cursor: &mut Cursor<'source>) -> NextToken {
        let next_token = self.primary.next_token(cursor);

        let NextToken::Unrecognized(character) = next_token else {
            return next_token;
        };

        cursor.push(character);
        self.secondary.next_token(cursor)
    }
}

pub(crate) trait WithTokenizer: Sized {
    fn with(self, secondary: impl Tokenizer) -> impl Tokenizer;
}

impl<T> WithTokenizer for T
where
    T: Tokenizer,
{
    fn with(self, secondary: impl Tokenizer) -> impl Tokenizer {
        CompositeTokenizer {
            primary: self,
            secondary,
        }
    }
}
