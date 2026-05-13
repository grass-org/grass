use crate::TokenBuilder;

use super::{PushTokenCharacterResult, Token};

#[derive(Debug)]
pub(crate) struct NewLineTokenBuilder;

impl NewLineTokenBuilder {
    pub const fn new(character: char) -> Option<Self> {
        if !is_newline(character) {
            return None;
        }

        Some(Self)
    }
}

impl TokenBuilder for NewLineTokenBuilder {
    fn push(&mut self, character: char) -> PushTokenCharacterResult {
        // We also compound normal whitespaces to an already started newline token
        if !is_whitespace(character) {
            return PushTokenCharacterResult::Failed;
        }

        PushTokenCharacterResult::Successful
    }

    fn build(self) -> Token {
        Token::NewLine
    }
}

const fn is_newline(character: char) -> bool {
    match character {
        // End-of-line characters
        | '\u{000A}' // line feed (\n)
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // carriage return (\r)
        | '\u{0085}' // next line (from latin1)
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR

        // `Default_Ignorable_Code_Point` characters
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK
        => true,
        _ => false,
    }
}

const fn is_whitespace(character: char) -> bool {
    match character {
        // End-of-line characters
        | '\u{000A}' // line feed (\n)
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // carriage return (\r)
        | '\u{0085}' // next line (from latin1)
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR

        // `Default_Ignorable_Code_Point` characters
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Horizontal space characters
        | '\u{0009}'   // tab (\t)
        | '\u{0020}' // space
        => true,
        _ => false,
    }
}
