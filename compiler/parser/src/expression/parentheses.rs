use super::{Error, ExpressionParser, Result, UnexpectedSyntaxError};
use interfaces::SyntaxKind;
use lexer::{Token, TokenSpan};

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    pub(super) fn parse_parentheses(&mut self) -> Result {
        let left = self.parse_folding(0)?;

        let TokenSpan {
            token: next_token,
            span: next_span,
        } = self.tokens.next().ok_or(Error::NoMoreTokens)?;

        if next_token != Token::CloseParenthesis {
            Err(UnexpectedSyntaxError {
                expected: SyntaxKind::CloseParenthesis,
                actual: next_token,
                span: next_span,
            })?;
        }

        Ok(left)
    }
}
