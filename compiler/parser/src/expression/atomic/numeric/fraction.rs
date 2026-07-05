use super::parse_magnitude;
use crate::NumericLiteralError;
use interfaces::{FractionLiteral, FractionLiteralKind};

pub(super) fn parse_fraction_literal(
    base: u8,
    kind: FractionLiteralKind,
    integral_part: String,
    fractional_part: Option<String>,
) -> Result<FractionLiteral, NumericLiteralError> {
    if integral_part.is_empty() {
        return Err(NumericLiteralError::MissingIntegralPart);
    }

    if let Some(fractional_part) = &fractional_part
        && fractional_part.is_empty()
    {
        return Err(NumericLiteralError::MissingFractionalPart);
    };

    let fractional_part = fractional_part.unwrap_or_else(|| "0".into());

    let value = match kind {
        FractionLiteralKind::Fraction32 => parse_fraction(base, integral_part, fractional_part)?,
        FractionLiteralKind::Fraction64 => parse_fraction(base, integral_part, fractional_part)?,
    };

    Ok(FractionLiteral { value, kind })
}

pub(super) fn parse_fraction(
    base: u8,
    integral_part: String,
    fractional_part: String,
) -> Result<f64, NumericLiteralError> {
    let integral_part = parse_magnitude(base, integral_part)? as f64;

    let fractional_radit_count = fractional_part.len() as u32;
    let fractional_numerator = parse_magnitude(base, fractional_part)? as f64;
    let fractional_denominator = (base as u128).pow(fractional_radit_count) as f64;

    let fractional_part = fractional_numerator / fractional_denominator;

    Ok(integral_part + fractional_part)
}
