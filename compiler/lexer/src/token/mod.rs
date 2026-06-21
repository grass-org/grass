mod numeric;
mod operator;

pub use numeric::*;
pub use operator::*;

use interfaces::Span;
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct TokenSpan {
    pub token: Token,
    pub span: Span,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug, Default)]
pub enum Token {
    #[default]
    Invalid,

    NewLine,

    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenBracket,      // [
    CloseBracket,     // ]
    OpenBrace,        // {
    CloseBrace,       // }

    Comma, // ,
    Colon, // :

    Operator(Operator),

    NumericLiteral(NumericLiteral),
    CharacterLiteral(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::NewLine => write!(f, "\\n"),
            Token::OpenParenthesis => write!(f, "("),
            Token::CloseParenthesis => write!(f, ")"),
            Token::OpenBracket => write!(f, "["),
            Token::CloseBracket => write!(f, "]"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Operator(value) => value.fmt(f),
            Token::NumericLiteral(value) => value.fmt(f),
            Token::CharacterLiteral(value) => value.fmt(f),
            Token::Invalid => write!(f, "{{invalid_token}}"),
        }
    }
}
