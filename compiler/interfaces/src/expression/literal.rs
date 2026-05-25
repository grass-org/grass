use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum LiteralExpression {
    Integer(IntegerLiteral),
    Fraction(FractionLiteral),
    String(StringLiteral),
}

impl LiteralExpression {
    pub const fn integer(integer: i32) -> LiteralExpression {
        Self::Integer(IntegerLiteral(integer))
    }

    pub const fn fraction(fraction: f64) -> LiteralExpression {
        Self::Fraction(FractionLiteral(fraction))
    }

    pub const fn string(string: String) -> LiteralExpression {
        Self::String(StringLiteral(string))
    }
}

impl Display for LiteralExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LiteralExpression::Integer(integer_literal) => integer_literal.fmt(f),
            LiteralExpression::Fraction(fraction_literal) => fraction_literal.fmt(f),
            LiteralExpression::String(string_literal) => string_literal.fmt(f),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct IntegerLiteral(pub i32);

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let IntegerLiteral(integer) = self;
        write!(f, "{integer}i")
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct FractionLiteral(pub f64);

impl Display for FractionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let FractionLiteral(fraction) = self;
        write!(f, "{fraction}f")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct StringLiteral(pub String);

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let StringLiteral(string) = self;
        write!(f, r#""{string}""#)
    }
}
