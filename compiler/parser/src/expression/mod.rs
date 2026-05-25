mod binding_power;

use std::iter::Peekable;

use interfaces::{
    AtomicExpression, BinaryExpression, BinaryOperator, BinaryOperatorSpan, ExpressionSpan,
    LiteralExpression,
};
use lexer::{Literal, LiteralKind, Token, TokenSpan};

use binding_power::BindingPowers;

pub fn parse_expression(tokens: impl Iterator<Item = TokenSpan>) -> Option<ExpressionSpan> {
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

    pub fn parse(&mut self) -> Option<ExpressionSpan> {
        self.pratt_parse(0)
    }

    fn pratt_parse(&mut self, highest_binding_power: u32) -> Option<ExpressionSpan> {
        let TokenSpan { token, span } = self.tokens.next()?;
        let expression = parse_atomic(token)?;
        let mut expression = ExpressionSpan::atomic(expression, span);

        loop {
            let Some(right) = self.pratt_parse_right(highest_binding_power) else {
                break;
            };

            let ExpressionRight { operator, operand } = right;
            let binary_expression = BinaryExpression::new(expression, operator, operand);
            expression = ExpressionSpan::binary(binary_expression);
        }

        Some(expression)
    }

    fn pratt_parse_right(&mut self, highest_binding_power: u32) -> Option<ExpressionRight> {
        let operator_span = self.peek_operator()?;
        let operator = operator_span.operator;

        let binding_power = self.binding_powers.binding_power(operator)?;

        if binding_power.left < highest_binding_power {
            return None;
        }

        _ = self.tokens.next();

        let right = ExpressionRight {
            operator: operator_span,
            operand: self.pratt_parse(binding_power.right)?,
        };

        Some(right)
    }

    fn peek_operator(&mut self) -> Option<BinaryOperatorSpan> {
        let TokenSpan { token, span } = self.tokens.peek()?;

        let operator = BinaryOperatorSpan {
            operator: parse_operator(token)?,
            span: *span,
        };

        Some(operator)
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

    use crate::expression::parse_expression;

    #[test]
    fn test() {
        let tokens = lex("1.0 + 2 * 3 * 4.5 / 5 - 3 + 2");
        println!("{}", parse_expression(tokens).unwrap());
        let tokens = lex("1 * 2 + 3.5");
        println!("{}", parse_expression(tokens).unwrap());
    }
}
