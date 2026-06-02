use crate::token::{PushTokenCharacterResult, TokenBuilder};
use crate::Token;

#[derive(Debug)]
pub(crate) struct BracketBuilder {
    bracket: Bracket,
}

impl BracketBuilder {
    pub fn new(start: char) -> Option<Self> {
        let Some(bracket) = Bracket::from(start) else {
            return None;
        };

        Some(BracketBuilder { bracket })
    }
}

impl TokenBuilder for BracketBuilder {
    fn push(&mut self, _: char) -> PushTokenCharacterResult {
        PushTokenCharacterResult::Failed
    }

    fn build(self) -> Token {
        self.bracket.into()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum Bracket {
    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenBracket,      // [
    CloseBracket,     // ]
    OpenBrace,        // {
    CloseBrace,       // }
}

impl Bracket {
    pub fn from(character: char) -> Option<Self> {
        let bracket = match character {
            '(' => Bracket::OpenParenthesis,
            ')' => Bracket::CloseParenthesis,
            '[' => Bracket::OpenBracket,
            ']' => Bracket::CloseBracket,
            '{' => Bracket::OpenBrace,
            '}' => Bracket::CloseBrace,
            _ => return None,
        };

        Some(bracket)
    }
}

impl From<Bracket> for Token {
    fn from(value: Bracket) -> Self {
        match value {
            Bracket::OpenParenthesis => Token::OpenParenthesis,
            Bracket::CloseParenthesis => Token::CloseParenthesis,
            Bracket::OpenBracket => Token::OpenBracket,
            Bracket::CloseBracket => Token::CloseBracket,
            Bracket::OpenBrace => Token::OpenBrace,
            Bracket::CloseBrace => Token::CloseBrace,
        }
    }
}
