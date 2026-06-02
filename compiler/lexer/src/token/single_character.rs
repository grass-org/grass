use crate::token::{PushTokenCharacterResult, TokenBuilder};
use crate::Token;

#[derive(Debug)]
pub(crate) struct SingleCharacterTokenBuilder {
    bracket: SingleCharacterToken,
}

impl SingleCharacterTokenBuilder {
    pub fn new(start: char) -> Option<Self> {
        let Some(bracket) = SingleCharacterToken::from(start) else {
            return None;
        };

        Some(SingleCharacterTokenBuilder { bracket })
    }
}

impl TokenBuilder for SingleCharacterTokenBuilder {
    fn push(&mut self, _: char) -> PushTokenCharacterResult {
        PushTokenCharacterResult::Failed
    }

    fn build(self) -> Token {
        self.bracket.into()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum SingleCharacterToken {
    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenBracket,      // [
    CloseBracket,     // ]
    OpenBrace,        // {
    CloseBrace,       // }

    Comma, // ,
    Colon, // :
}

impl SingleCharacterToken {
    pub fn from(character: char) -> Option<Self> {
        let bracket = match character {
            '(' => SingleCharacterToken::OpenParenthesis,
            ')' => SingleCharacterToken::CloseParenthesis,
            '[' => SingleCharacterToken::OpenBracket,
            ']' => SingleCharacterToken::CloseBracket,
            '{' => SingleCharacterToken::OpenBrace,
            '}' => SingleCharacterToken::CloseBrace,

            ',' => SingleCharacterToken::Comma,
            ':' => SingleCharacterToken::Colon,

            _ => return None,
        };

        Some(bracket)
    }
}

impl From<SingleCharacterToken> for Token {
    fn from(value: SingleCharacterToken) -> Self {
        match value {
            SingleCharacterToken::OpenParenthesis => Token::OpenParenthesis,
            SingleCharacterToken::CloseParenthesis => Token::CloseParenthesis,
            SingleCharacterToken::OpenBracket => Token::OpenBracket,
            SingleCharacterToken::CloseBracket => Token::CloseBracket,
            SingleCharacterToken::OpenBrace => Token::OpenBrace,
            SingleCharacterToken::CloseBrace => Token::CloseBrace,

            SingleCharacterToken::Comma => Token::Comma,
            SingleCharacterToken::Colon => Token::Colon,
        }
    }
}
