mod builder;
mod composite;
mod separated;

pub(crate) use composite::*;
pub(crate) use separated::*;

use crate::{Cursor, Token};

pub(crate) trait Tokenizer {
    fn next_token<'source>(&mut self, cursor: &mut Cursor<'source>) -> NextToken;
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub(crate) enum NextToken {
    Token(Token),
    Unrecognized(char),
    Ignore,
    Done,
}

pub(crate) trait IntoTokenizer {
    fn tokenizer(self) -> impl Tokenizer;
}
