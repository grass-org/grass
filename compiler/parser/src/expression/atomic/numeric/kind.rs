use crate::NumericLiteralError;
use interfaces::{
    FractionLiteralKind, IntegerLiteralKind, MagnitudeLiteralKind, NumericLiteralKind,
};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub(super) fn kind(
    kind: Option<String>,
    has_fractional_part: bool,
) -> Result<NumericLiteralKind, InvalidNumericKindError> {
    let Some(kind) = kind else {
        let kind = match has_fractional_part {
            false => NumericLiteralKind::Integer(IntegerLiteralKind::Integer32),
            true => NumericLiteralKind::Fraction(FractionLiteralKind::Fraction32),
        };

        return Ok(kind);
    };

    parse_kind(kind, has_fractional_part)
}

fn parse_kind(
    string: String,
    has_fractional_part: bool,
) -> Result<NumericLiteralKind, InvalidNumericKindError> {
    let kind = match string.as_str() {
        "I8" => NumericLiteralKind::Integer(IntegerLiteralKind::Integer8),
        "I16" => NumericLiteralKind::Integer(IntegerLiteralKind::Integer16),
        "I32" => NumericLiteralKind::Integer(IntegerLiteralKind::Integer32),
        "I64" => NumericLiteralKind::Integer(IntegerLiteralKind::Integer64),
        "I128" => NumericLiteralKind::Integer(IntegerLiteralKind::Integer128),
        "IArch" => NumericLiteralKind::Integer(IntegerLiteralKind::ArchInteger),
        "M8" => NumericLiteralKind::Magnitude(MagnitudeLiteralKind::Magnitude8),
        "M16" => NumericLiteralKind::Magnitude(MagnitudeLiteralKind::Magnitude16),
        "M32" => NumericLiteralKind::Magnitude(MagnitudeLiteralKind::Magnitude32),
        "M64" => NumericLiteralKind::Magnitude(MagnitudeLiteralKind::Magnitude64),
        "M128" => NumericLiteralKind::Magnitude(MagnitudeLiteralKind::Magnitude128),
        "MArch" => NumericLiteralKind::Magnitude(MagnitudeLiteralKind::ArchMagnitude),
        "F32" => NumericLiteralKind::Fraction(FractionLiteralKind::Fraction32),
        "F64" => NumericLiteralKind::Fraction(FractionLiteralKind::Fraction64),
        _ => return Err(kind_error(string, has_fractional_part)),
    };

    if has_fractional_part
        && !NumericLiteralKind::fraction().any(|fraction_kind| kind == fraction_kind)
    {
        return Err(kind_error(string, has_fractional_part));
    }

    Ok(kind)
}

fn kind_error(string: String, has_fractional_part: bool) -> InvalidNumericKindError {
    let expected_kinds = match has_fractional_part {
        true => NumericLiteralKind::fraction().collect(),
        false => NumericLiteralKind::all().collect(),
    };

    InvalidNumericKindError {
        kind: string,
        expected_kinds,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct InvalidNumericKindError {
    pub kind: String,
    pub expected_kinds: Vec<NumericLiteralKind>,
}

impl From<InvalidNumericKindError> for NumericLiteralError {
    fn from(value: InvalidNumericKindError) -> Self {
        Self::InvalidKind(value)
    }
}

impl Display for InvalidNumericKindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let InvalidNumericKindError {
            kind,
            expected_kinds,
        } = self;

        write!(
            f,
            "unexpected literal literal kind {kind}; kind must be one of ["
        )?;

        for (index, expected_kind) in expected_kinds.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{expected_kind}")?;
        }

        Ok(())
    }
}

impl Error for InvalidNumericKindError {}
