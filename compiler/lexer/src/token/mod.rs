mod builder;
mod identifier;
mod literal;
mod new_line;
mod single_character;
mod tokenizer;

pub use literal::*;

pub(crate) use builder::*;
pub(crate) use identifier::*;
pub(crate) use new_line::*;
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

    Dash,  // -
    Tilde, // ~

    Plus,    // +
    Star,    // *
    Slash,   // /
    Percent, // %


    NumericLiteral(NumericLiteral),
    CharacterLiteral(CharacterLiteral),
    StringLiteral(String),
    Identifier(String),

    Invalid,
}
