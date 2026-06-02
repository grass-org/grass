use super::parse_radit;
use crate::{IntegerLiteralOverflowError, NumericLiteralError};
use interfaces::{MagnitudeLiteral, MagnitudeLiteralKind, NumericLiteralKind};

pub(super) fn parse_magnitude_literal(
    base: u8,
    kind: MagnitudeLiteralKind,
    string: String,
) -> Result<MagnitudeLiteral, NumericLiteralError> {
    if string.is_empty() {
        return Err(NumericLiteralError::MissingIntegralPart);
    }

    let literal = match kind {
        MagnitudeLiteralKind::Magnitude8 => parse_magnitude::<u8>(base, kind, string)?.into(),
        MagnitudeLiteralKind::Magnitude16 => parse_magnitude::<u16>(base, kind, string)?.into(),
        MagnitudeLiteralKind::Magnitude32 => parse_magnitude::<u32>(base, kind, string)?.into(),
        MagnitudeLiteralKind::Magnitude64 => parse_magnitude::<u64>(base, kind, string)?.into(),
        MagnitudeLiteralKind::Magnitude128 => parse_magnitude::<u128>(base, kind, string)?.into(),
        MagnitudeLiteralKind::ArchMagnitude => parse_magnitude::<usize>(base, kind, string)?.into(),
    };

    Ok(literal)
}

pub(super) fn parse_magnitude<T>(
    base: u8,
    kind: impl Into<NumericLiteralKind>,
    radits: String,
) -> Result<T, NumericLiteralError>
where
    T: TryFrom<u128> + Default,
{
    let mut value: u128 = 0;
    let kind = kind.into();

    for character in radits.chars() {
        let radit = parse_radit(base, character)?;

        let Some(new_value) = value.checked_mul(10) else {
            return Err(overflow_error(base, kind, radits));
        };

        let Some(new_value) = new_value.checked_add(radit as u128) else {
            return Err(overflow_error(base, kind, radits));
        };

        value = new_value;
    }

    value
        .try_into()
        .map_err(|_| overflow_error(base, kind, radits))
}

fn overflow_error(base: u8, kind: NumericLiteralKind, radits: String) -> NumericLiteralError {
    IntegerLiteralOverflowError { base, radits, kind }.into()
}
