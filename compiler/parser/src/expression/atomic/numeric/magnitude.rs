use interfaces::MagnitudeLiteral;
use interfaces::MagnitudeLiteralKind;

use super::parse_radit;
use crate::MaxIntegerLiteralOverflowError;
use crate::NumericLiteralError;

pub(super) fn parse_magnitude_literal(
    base: u8,
    kind: MagnitudeLiteralKind,
    string: String,
) -> Result<MagnitudeLiteral, NumericLiteralError> {
    if string.is_empty() {
        return Err(NumericLiteralError::MissingIntegralPart);
    }

    let value = match kind {
        MagnitudeLiteralKind::Magnitude8 => parse_magnitude(base, string)?,
        MagnitudeLiteralKind::Magnitude16 => parse_magnitude(base, string)?,
        MagnitudeLiteralKind::Magnitude32 => parse_magnitude(base, string)?,
        MagnitudeLiteralKind::Magnitude64 => parse_magnitude(base, string)?,
        MagnitudeLiteralKind::Magnitude128 => parse_magnitude(base, string)?,
        MagnitudeLiteralKind::ArchMagnitude => parse_magnitude(base, string)?,
    };

    Ok(MagnitudeLiteral { value, kind })
}

pub(super) fn parse_magnitude(base: u8, radits: String) -> Result<u128, NumericLiteralError> {
    let mut value: u128 = 0;
    for character in radits.chars() {
        let radit = parse_radit(base, character)?;

        let Some(new_value) = value.checked_mul(10) else {
            return Err(overflow_error(base, radits));
        };

        let Some(new_value) = new_value.checked_add(radit as u128) else {
            return Err(overflow_error(base, radits));
        };

        value = new_value;
    }

    Ok(value)
}

fn overflow_error(base: u8, radits: String) -> NumericLiteralError {
    MaxIntegerLiteralOverflowError { base, radits }.into()
}
