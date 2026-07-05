use interfaces::Spanned;
use lexer::Token;

use super::Error;
use super::ExpressionParser;
use super::Result;
use super::parse_atomic_expression;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = Spanned<Token>>,
{
    pub(super) fn parse_left(&mut self) -> Result {
        let Spanned { content, span } = self.tokens.next().ok_or(Error::NoMoreTokens)?;

        if content == Token::OpenParenthesis {
            return self.parse_parentheses();
        }

        if let Token::Operator(operator) = content {
            return self.parse_prefix_expression(operator.symbol, span);
        }

        let expression = parse_atomic_expression(Spanned { content, span })?;
        let expression = expression.into();
        Ok(Spanned {
            content: expression,
            span,
        })
    }
}
