use super::{Error, ExpressionParser, Result, UnexpectedSyntaxError};
use interfaces::{SyntaxKind, Spanned};
use lexer::Token;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = Spanned<Token>>,
{
    pub(super) fn parse_parentheses(&mut self) -> Result {
        let left = self.parse_folding(0)?;

        let Spanned {
            content: next_token,
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
