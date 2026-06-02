use crate::expression::atomic::numeric::magnitude::parse_magnitude;
use crate::NumericLiteralError;
use interfaces::{FractionLiteral, FractionLiteralKind};
use std::fmt::Debug;
use std::str::FromStr;

pub(super) fn parse_fraction_literal(
    base: u8,
    kind: FractionLiteralKind,
    integral_part: String,
    fractional_part: Option<String>,
) -> Result<FractionLiteral, NumericLiteralError> {
    if integral_part.is_empty() {
        return Err(NumericLiteralError::MissingIntegralPart);
    }

    if let Some(fractional_part) = &fractional_part {
        if fractional_part.is_empty() {
            return Err(NumericLiteralError::MissingFractionalPart);
        }
    };

    let fractional_part = fractional_part.unwrap_or_else(|| String::from("0"));

    let literal = match kind {
        FractionLiteralKind::Fraction32 => {
            parse_fraction::<f32>(base, kind, integral_part, fractional_part)?.into()
        }
        FractionLiteralKind::Fraction64 => {
            parse_fraction::<f64>(base, kind, integral_part, fractional_part)?.into()
        }
    };

    Ok(literal)
}

pub(super) fn parse_fraction<T>(
    base: u8,
    kind: FractionLiteralKind,
    integral_part: String,
    fractional_part: String,
) -> Result<T, NumericLiteralError>
where
    T: FromStr<Err: Debug> + Default,
{
    let integral_part = parse_magnitude::<u128>(base, kind, integral_part)?;
    let fractional_part = parse_magnitude::<u128>(base, kind, fractional_part)?;

    let fraction = format!("{integral_part}.{fractional_part}");
    let fraction = str::parse::<T>(&fraction).expect("fraction must be a valid float");

    Ok(fraction)
}
