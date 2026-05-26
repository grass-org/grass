mod binding_power;
mod error;

pub use error::*;

use std::{iter::Peekable, result};

use interfaces::{
    AtomicExpression, BinaryExpression, BinaryOperator, BinaryOperatorSpan, ExpressionSpan,
    LiteralExpression, SyntaxKind,
};
use lexer::{Literal, LiteralKind, Token, TokenSpan};

use binding_power::BindingPowers;

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
            binding_powers: BindingPowers::new(),
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result {
        self.pratt_parse(0)
    }

    fn pratt_parse(&mut self, highest_binding_power: u32) -> Result {
        let Some(TokenSpan { token, span }) = self.tokens.next() else {
            return Err(Error::NoMoreTokens);
        };

        let Some(expression) = parse_atomic(token) else {
            return Err(Error::unexpected_syntax(SyntaxKind::Expression, span));
        };

        let mut expression = ExpressionSpan::atomic(expression, span);

        loop {
            let Ok(right) = self.pratt_parse_right(highest_binding_power) else {
                break;
            };

            let ExpressionRight { operator, operand } = right;
            let binary_expression = BinaryExpression::new(expression, operator, operand);
            expression = ExpressionSpan::binary(binary_expression);
        }

        Ok(expression)
    }

    fn pratt_parse_right(&mut self, highest_binding_power: u32) -> Result<ExpressionRight> {
        let operator_span = self.peek_operator()?;

        let operator = operator_span.operator;

        let Some(binding_power) = self.binding_powers.binding_power(operator) else {
            return Err(Error::undefined_binding_power(operator));
        };

        if binding_power.left < highest_binding_power {
            return Err(Error::NoMoreTokens);
        }

        _ = self.tokens.next();

        let right = ExpressionRight {
            operator: operator_span,
            operand: self.pratt_parse(binding_power.right)?,
        };

        Ok(right)
    }

    fn peek_operator(&mut self) -> Result<BinaryOperatorSpan> {
        let Some(TokenSpan { token, span }) = self.tokens.peek() else {
            return Err(Error::NoMoreTokens);
        };

        let span = *span;

        let Some(operator) = parse_operator(token) else {
            return Err(Error::unexpected_syntax(SyntaxKind::BinaryOperator, span));
        };

        let operator = BinaryOperatorSpan { operator, span };

        Ok(operator)
    }
}

struct ExpressionRight {
    pub operator: BinaryOperatorSpan,
    pub operand: ExpressionSpan,
}

fn parse_atomic(token: Token) -> Option<AtomicExpression> {
    let operand = match token {
        Token::Literal(literal) => AtomicExpression::Literal(parse_literal(literal)?),
        Token::Identifier(_) => todo!(),
        _ => return None,
    };

    Some(operand)
}

fn parse_literal(literal: Literal) -> Option<LiteralExpression> {
    let kind = literal.kind();
    let symbol = literal.take_symbol();

    let expression = match kind {
        LiteralKind::Integer => LiteralExpression::integer(parse_integer_literal(&symbol)?),
        LiteralKind::Fraction => LiteralExpression::fraction(parse_fraction_literal(&symbol)?),
        LiteralKind::String => LiteralExpression::string(symbol),
    };

    Some(expression)
}

fn parse_integer_literal(symbol: &str) -> Option<i32> {
    symbol.parse().ok()
}

fn parse_fraction_literal(symbol: &str) -> Option<f64> {
    symbol.parse().ok()
}

const fn parse_operator(token: &Token) -> Option<BinaryOperator> {
    let operator = match token {
        Token::Plus => BinaryOperator::Add,
        Token::Dash => BinaryOperator::Subtract,
        Token::Star => BinaryOperator::Multiply,
        Token::Slash => BinaryOperator::Divide,
        Token::Percent => BinaryOperator::Remainder,
        _ => return None,
    };

    Some(operator)
}

#[cfg(test)]
mod tests {
    use lexer::lex;

    use crate::expression::{Result, parse_expression};

    #[test]
    fn test_sequential() -> Result<()> {
        let tokens = lex("1 % 2 / 3 * 4 - 5 + 6");
        let expected = "(((((1i % 2i) / 3i) * 4i) - 5i) + 6i)";
        let actual = format!("{}", parse_expression(tokens)?);
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_precedence() -> Result<()> {
        let tokens = lex("1 + 2 * 3 / 4 - 5 % 6 % 7");
        let expected = "((1i + ((2i * 3i) / 4i)) - ((5i % 6i) % 7i))";
        let actual = format!("{}", parse_expression(tokens)?);
        assert_eq!(expected, actual);

        Ok(())
    }
}
