use crate::{Cursor, IntoTokenizer, NextToken, PushTokenCharacterResult, TokenBuilder, Tokenizer};

#[derive(Debug)]
pub(crate) struct BuilderTokenizer<Builder, Start>
where
    Builder: TokenBuilder,
    Start: Fn(char) -> Option<Builder>,
{
    start: Start,
}

impl<Builder, Start> BuilderTokenizer<Builder, Start>
where
    Builder: TokenBuilder,
    Start: Fn(char) -> Option<Builder>,
{
    pub const fn new(start: Start) -> Self {
        BuilderTokenizer { start }
    }
}

impl<Builder, Start> Tokenizer for BuilderTokenizer<Builder, Start>
where
    Builder: TokenBuilder,
    Start: Fn(char) -> Option<Builder>,
{
    fn next_token<'source>(&mut self, cursor: &mut Cursor<'source>) -> NextToken {
        let Some(start) = cursor.pop() else {
            return NextToken::Done;
        };

        let Some(mut builder) = (self.start)(start) else {
            return NextToken::Unrecognized(start);
        };

        loop {
            let Some(character) = cursor.pop() else {
                break;
            };

            if builder.push(character) == PushTokenCharacterResult::Failed {
                cursor.push(character);
                break;
            }
        }

        NextToken::Token(builder.build())
    }
}

impl<Builder, Start> IntoTokenizer for Start
where
    Builder: TokenBuilder,
    Start: Fn(char) -> Option<Builder>,
{
    fn tokenizer(self) -> impl Tokenizer {
        BuilderTokenizer::new(self)
    }
}
