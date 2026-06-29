use super::{Error, ExpressionParser, Result, parse_atomic_expression};
use interfaces::ExpressionSpan;
use lexer::{Token, TokenSpan};

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    pub(super) fn parse_left(&mut self) -> Result {
        let TokenSpan { token, span } = self.tokens.next().ok_or(Error::NoMoreTokens)?;

        if token == Token::OpenParenthesis {
            return self.parse_parentheses();
        }

        if let Token::Operator(operator) = token {
            return self.parse_prefix_expression(operator.symbol, span);
        }

        let expression = parse_atomic_expression(TokenSpan { token, span })?;
        let expression = expression.into();
        Ok(ExpressionSpan { expression, span })
    }
}
