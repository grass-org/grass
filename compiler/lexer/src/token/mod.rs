mod builder;
mod identifier;
mod literal;
mod new_line;
mod operator;
mod single_character;
mod tokenizer;

pub use literal::*;
use std::fmt::{Display, Formatter};

pub(crate) use builder::*;
pub(crate) use identifier::*;
pub(crate) use new_line::*;
pub(crate) use operator::*;
pub(crate) use single_character::*;
pub(crate) use tokenizer::*;

use interfaces::Span;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct TokenSpan {
    pub token: Token,
    pub span: Span,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum Token {
    NewLine,

    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenBracket,      // [
    CloseBracket,     // ]
    OpenBrace,        // {
    CloseBrace,       // }

    Comma, // ,
    Colon, // :

    Operator(String),

    NumericLiteral(NumericLiteral),
    CharacterLiteral(CharacterLiteral),
    StringLiteral(String),
    Identifier(String),

    Invalid,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            Token::StringLiteral(value) => value.fmt(f),
            Token::Identifier(value) => value.fmt(f),
            Token::Invalid => write!(f, "{{invalid_token}}"),
        }
    }
}
