use interfaces::BinaryOperator;
use interfaces::Expression;
use interfaces::Spanned;
use lexer::Token;

use super::Error;
use super::ExpressionParser;
use super::OperatorToken;
use super::Result;
use super::peek_operator_token;

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = Spanned<Token>>,
{
    pub(super) fn parse_right(
        &mut self,
        min_binding_power: u32,
    ) -> Result<(Spanned<BinaryOperator>, Spanned<Expression>)> {
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

fn next_infix_operator(
    mut tokens: impl Iterator<Item = Spanned<Token>>,
) -> Option<Spanned<BinaryOperator>> {
    let Spanned { content, span } = tokens.next()?;

    let Token::Operator(OperatorToken { symbol, .. }) = content else {
        return None;
    };

    let operator = BinaryOperator { symbol };
    Some(Spanned::new(operator, span))
}
