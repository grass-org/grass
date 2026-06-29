mod atomic;
mod binding_power;
mod error;
mod left;
mod parentheses;
mod prefix;
mod right;

pub use atomic::*;
pub use binding_power::*;
pub use error::*;

use binding_power::BindingPowers;

use interfaces::{BinaryExpression, ExpressionSpan};
use lexer::TokenSpan;
use std::{iter::Peekable, result};

type Error = ParseExpressionError;
type Result<T = ExpressionSpan, E = Error> = result::Result<T, E>;

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
        self.parse_folding(0)
    }

    fn parse_folding(&mut self, min_binding_power: u32) -> Result {
        let mut expression = self.parse_left()?;

        loop {
            let Ok(right) = self.parse_right(min_binding_power) else {
                break;
            };

            let (operator, operand) = right;
            let operands = [expression, operand];
            let binary_expression = BinaryExpression::new(operator, operands);
            expression = binary_expression.into();
        }

        Ok(expression)
    }
}

#[cfg(test)]
mod tests {
    use lexer::lex;

    use crate::expression::{Result, parse_expression};

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
