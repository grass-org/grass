use std::iter::Peekable;

use interfaces::Expression;
use interfaces::Operator;
use interfaces::Spanned;
use lexer::Token;

use super::Error;
use super::ExpressionParser;
use super::Result;

type OperatorToken = lexer::Operator;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = Spanned<Token>>,
{
    pub(super) fn parse_right(
        &mut self,
        min_binding_power: u32,
    ) -> Result<(Spanned<Operator>, Spanned<Expression>)> {
        let OperatorToken { symbol, .. } =
            peek_operator_token(&mut self.tokens).ok_or(Error::NoMoreTokens)?;

        let binding_power = self.binding_powers.infix_binding_power(symbol)?;

        if binding_power.left < min_binding_power {
            return Err(Error::NoMoreTokens);
        }

        let operator = next_infix_operator(&mut self.tokens).ok_or(Error::NoMoreTokens)?;

        let expression = self.parse_folding(binding_power.right)?;
        Ok((operator, expression))
    }
}

fn peek_operator_token(
    tokens: &mut Peekable<impl Iterator<Item = Spanned<Token>>>,
) -> Option<&OperatorToken> {
    let Spanned { content, .. } = tokens.peek()?;

    let Token::Operator(operator) = content else {
        return None;
    };

    Some(operator)
}

fn next_infix_operator(
    mut tokens: impl Iterator<Item = Spanned<Token>>,
) -> Option<Spanned<Operator>> {
    let Spanned { content, span } = tokens.next()?;

    let Token::Operator(OperatorToken { symbol, .. }) = content else {
        return None;
    };

    let operator = Operator { symbol };
    Some(Spanned {
        content: operator,
        span,
    })
}
