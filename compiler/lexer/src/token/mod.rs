mod format;
mod numeric;
mod operator;
mod string_literal;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use format::format_slice;
pub use numeric::*;
pub use operator::*;
pub use string_literal::*;

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
    StringLiteral(Vec<StringLiteralPart>),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Invalid => write!(f, "{{invalid_token}}"),
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
            Token::StringLiteral(parts) => format_slice(parts, f),
        }
    }
}
