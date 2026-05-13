use crate::{Token, TokenBuilder};

use super::PushTokenCharacterResult;

#[derive(Debug)]
pub(crate) struct SingleCharacterBuilderTemplate {
    character: char,
    token: Token,
}

impl SingleCharacterBuilderTemplate {
    pub fn create(&self, start: char) -> Option<SingleCharacterBuilder> {
        if self.character != start {
            return None;
        }

        let builder = SingleCharacterBuilder {
            token: self.token.clone(),
        };

        Some(builder)
    }
}

#[derive(Debug)]
pub(crate) struct SingleCharacterBuilder {
    token: Token,
}

impl TokenBuilder for SingleCharacterBuilder {
    fn push(&mut self, _: char) -> PushTokenCharacterResult {
        PushTokenCharacterResult::Failed
    }

    fn build(self) -> Token {
        self.token
    }
}

macro_rules! builder_template {
    ($name: ident, $character: expr, $token: expr) => {
        pub(crate) const fn $name() -> SingleCharacterBuilderTemplate {
            SingleCharacterBuilderTemplate {
                character: $character,
                token: $token,
            }
        }
    };
}

builder_template!(dash_builder_template, '-', Token::Dash);
builder_template!(tilde_builder_template, '~', Token::Tilde);
builder_template!(plus_builder_template, '+', Token::Plus);
builder_template!(star_builder_template, '*', Token::Star);
builder_template!(slash_builder_template, '/', Token::Slash);
builder_template!(percent_builder_template, '%', Token::Percent);
