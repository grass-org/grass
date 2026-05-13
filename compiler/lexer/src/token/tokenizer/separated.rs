use crate::{Cursor, NextToken, Tokenizer};

#[derive(Debug)]
pub(crate) struct SpaceSeparatedTokenizer<T>
where
    T: Tokenizer,
{
    tokenizer: T,
}

impl<T> Tokenizer for SpaceSeparatedTokenizer<T>
where
    T: Tokenizer,
{
    fn next_token<'source>(&mut self, cursor: &mut Cursor<'source>) -> NextToken {
        let next_token = self.tokenizer.next_token(cursor);

        let NextToken::Unrecognized(character) = next_token else {
            return next_token;
        };

        if is_horizontal_whitespace(character) {
            return NextToken::Ignore;
        }

        next_token
    }
}

const fn is_horizontal_whitespace(character: char) -> bool {
    matches!(
        character,
        // `Default_Ignorable_Code_Point` characters
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Horizontal space characters
        | '\u{0009}'   // tab (\t)
        | '\u{0020}'
    )
}

pub(crate) trait SpaceSeparateTokenizer: Sized {
    fn space_separate(self) -> impl Tokenizer;
}

impl<T> SpaceSeparateTokenizer for T
where
    T: Tokenizer,
{
    fn space_separate(self) -> impl Tokenizer {
        SpaceSeparatedTokenizer { tokenizer: self }
    }
}
