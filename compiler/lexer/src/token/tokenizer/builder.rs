use crate::{Cursor, IntoTokenizer, NextToken, PushTokenCharacterResult, TokenBuilder, Tokenizer};

#[derive(Debug)]
pub(crate) struct BuilderTokenizer<Builder, CreateBuilder>
where
    Builder: TokenBuilder,
    CreateBuilder: Fn(char) -> Option<Builder>,
{
    create_builder: CreateBuilder,
}

impl<Builder, CreateBuilder> BuilderTokenizer<Builder, CreateBuilder>
where
    Builder: TokenBuilder,
    CreateBuilder: Fn(char) -> Option<Builder>,
{
    pub const fn new(create_builder: CreateBuilder) -> Self {
        BuilderTokenizer { create_builder }
    }
}

impl<Builder, CreateBuilder> Tokenizer for BuilderTokenizer<Builder, CreateBuilder>
where
    Builder: TokenBuilder,
    CreateBuilder: Fn(char) -> Option<Builder>,
{
    fn next_token<'source>(&mut self, cursor: &mut Cursor<'source>) -> NextToken {
        let Some(start) = cursor.pop() else {
            return NextToken::Done;
        };

        let Some(mut builder) = (self.create_builder)(start) else {
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

impl<Builder, CreateBuilder> IntoTokenizer for CreateBuilder
where
    Builder: TokenBuilder,
    CreateBuilder: Fn(char) -> Option<Builder>,
{
    fn tokenizer(self) -> impl Tokenizer {
        BuilderTokenizer::new(self)
    }
}
