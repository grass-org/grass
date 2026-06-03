mod atomic;
mod binding_power;
mod error;
mod operator;

pub use atomic::*;
pub use binding_power::*;
pub use error::*;

use std::{iter::Peekable, result};

use interfaces::{
    BinaryExpression, ExpressionSpan, Operator, OperatorSpan, Span, SyntaxKind, UnaryExpression,
};
use lexer::{Token, TokenSpan};

use binding_power::BindingPowers;
use operator::parse_operator;

pub type Error = ParseExpressionError;
pub type Result<T = ExpressionSpan, E = Error> = result::Result<T, E>;

pub fn parse_expression(tokens: impl Iterator<Item = TokenSpan>) -> Result {
    let mut parser = ExpressionParser::new(tokens);
    parser.parse()
}

struct ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    binding_powers: BindingPowers,
    tokens: Peekable<Iter>,
}

impl<Iter> ExpressionParser<Iter>
where
    Iter: Iterator<Item = TokenSpan>,
{
    pub fn new(tokens: Iter) -> Self {
        Self {
            binding_powers: BindingPowers::default(),
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result {
        self.try_parse(0)
    }

    fn try_parse(&mut self, min_binding_power: u32) -> Result {
        let mut expression = self.parse_left()?;

        loop {
            let Ok(right) = self.parse_right(min_binding_power) else {
                break;
            };

            let ExpressionRight { operator, operand } = right;
            let binary_expression = BinaryExpression::new(operator, expression, operand);
            expression = binary_expression.into();
        }

        Ok(expression)
    }

    fn parse_left(&mut self) -> Result {
        let TokenSpan { token, span } = self.next_token()?;

        if token == Token::OpenParenthesis {
            return self.parse_parentheses();
        }

        if let Token::Operator(operator) = token {
            return self.parse_prefix_expression(operator.symbol, span);
        }

        let expression = parse_atomic_expression(token)?;
        let expression = expression.into();
        Ok(ExpressionSpan { expression, span })
    }

    fn parse_parentheses(&mut self) -> Result {
        let left = self.try_parse(0)?;

        let TokenSpan {
            token: next_token,
            span: next_span,
        } = self.next_token()?;

        if next_token != Token::CloseParenthesis {
            let error = Error::unexpected_syntax(SyntaxKind::CloseParenthesis, next_span);
            return Err(error);
        }

        Ok(left)
    }

    fn parse_prefix_expression(&mut self, symbol: String, span: Span) -> Result {
        let operator = Operator { symbol };

        let binding_power = self.binding_powers.prefix_binding_power(&operator)?;

        let operator = OperatorSpan { operator, span };
        let operand = self.try_parse(binding_power)?;

        Ok(UnaryExpression::new(operator, operand).into())
    }

    fn parse_right(&mut self, min_binding_power: u32) -> Result<ExpressionRight> {
        let operator_span = self.peek_operator()?;

        let binding_power = self
            .binding_powers
            .infix_binding_power(&operator_span.operator)?;

        if binding_power.left < min_binding_power {
            return Err(Error::NoMoreTokens);
        }

        _ = self.next_token();

        let right = ExpressionRight {
            operator: operator_span,
            operand: self.try_parse(binding_power.right)?,
        };

        Ok(right)
    }

    fn next_token(&mut self) -> Result<TokenSpan> {
        self.tokens.next().ok_or(ParseExpressionError::NoMoreTokens)
    }

    fn peek_operator(&mut self) -> Result<OperatorSpan> {
        let Some(token) = self.tokens.peek() else {
            return Err(Error::NoMoreTokens);
        };

        // TODO: this is a String clone!!
        Ok(parse_operator(token.clone())?)
    }
}

struct ExpressionRight {
    pub operator: OperatorSpan,
    pub operand: ExpressionSpan,
}

#[cfg(test)]
mod tests {
    use lexer::lex;

    use crate::expression::{parse_expression, Result};

    #[test]
    fn test_sequential() -> Result<()> {
        let tokens = lex("1 % 2 / 3 * 4 - 5 + 6");
        let expected = "+(-(*(/(%(1:I32, 2:I32), 3:I32), 4:I32), 5:I32), 6:I32)";
        let actual = format!("{}", parse_expression(tokens)?);
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_precedence() -> Result<()> {
        let tokens = lex("1 + 2 * 3 / 4 - 5 % 6 % 7");
        let expected = "-(+(1:I32, /(*(2:I32, 3:I32), 4:I32)), %(%(5:I32, 6:I32), 7:I32))";
        let actual = format!("{}", parse_expression(tokens)?);
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parentheses() -> Result<()> {
        let tokens = lex("(1 + 2) * 3 / (4 - 5) % 6 % 7");
        let expected = "%(%(/(*(+(1:I32, 2:I32), 3:I32), -(4:I32, 5:I32)), 6:I32), 7:I32)";
        let actual = format!("{}", parse_expression(tokens)?);
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_prefix() -> Result<()> {
        let tokens = lex("-1 + 2 * -3 / 4 - 5 % -6 % 7");
        let expected = "-(+(-(1:I32), /(*(2:I32, -(3:I32)), 4:I32)), %(%(5:I32, -(6:I32)), 7:I32))";
        let actual = format!("{}", parse_expression(tokens)?);
        assert_eq!(expected, actual);

        Ok(())
    }
}
