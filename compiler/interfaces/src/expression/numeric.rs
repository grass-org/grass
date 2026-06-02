use crate::{FractionLiteralKind, IntegerLiteralKind, MagnitudeLiteralKind};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum NumericLiteralArchetype {
    Integer,
    Magnitude,
    Fraction,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum NumericLiteralKind {
    Integer(IntegerLiteralKind),
    Magnitude(MagnitudeLiteralKind),
    Fraction(FractionLiteralKind),
}

impl NumericLiteralKind {
    pub fn all() -> impl Iterator<Item = Self> {
        Self::integer()
            .chain(Self::magnitude())
            .chain(Self::fraction())
    }

    pub fn of_archetype(
        archetype: NumericLiteralArchetype,
    ) -> Box<dyn Iterator<Item = NumericLiteralKind>> {
        match archetype {
            NumericLiteralArchetype::Integer => Box::new(Self::integer()),
            NumericLiteralArchetype::Magnitude => Box::new(Self::magnitude()),
            NumericLiteralArchetype::Fraction => Box::new(Self::fraction()),
        }
    }

    pub fn archetype(&self) -> NumericLiteralArchetype {
        match self {
            NumericLiteralKind::Integer(_) => NumericLiteralArchetype::Integer,
            NumericLiteralKind::Magnitude(_) => NumericLiteralArchetype::Magnitude,
            NumericLiteralKind::Fraction(_) => NumericLiteralArchetype::Fraction,
        }
    }

    pub fn integer() -> impl Iterator<Item = NumericLiteralKind> {
        IntegerLiteralKind::all().map(|kind| kind.into())
    }

    pub fn magnitude() -> impl Iterator<Item = NumericLiteralKind> {
        MagnitudeLiteralKind::all().map(|kind| kind.into())
    }

    pub fn fraction() -> impl Iterator<Item = NumericLiteralKind> {
        FractionLiteralKind::all().map(|kind| kind.into())
    }
}

macro_rules! from {
    ($from: ident, $into: ident, $variant: ident) => {
        impl From<$from> for $into {
            fn from(value: $from) -> Self {
                Self::$variant(value)
            }
        }
    };
}

pub(super) use from;

impl Display for NumericLiteralKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
