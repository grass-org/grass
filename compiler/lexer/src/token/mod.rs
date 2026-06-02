mod bracket;
mod builder;
mod identifier;
mod literal;
mod new_line;
mod operator;
mod tokenizer;

pub use literal::*;

pub(crate) use bracket::*;
pub(crate) use builder::*;
pub(crate) use identifier::*;
pub(crate) use new_line::*;
pub(crate) use operator::*;
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
    Operator(String),

    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenBracket,      // [
    CloseBracket,     // ]
    OpenBrace,        // {
    CloseBrace,       // }

    NumericLiteral(NumericLiteral),
    CharacterLiteral(CharacterLiteral),
    StringLiteral(String),
    Identifier(String),

    Invalid,
}
