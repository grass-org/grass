use super::parse_magnitude;
use crate::NumericLiteralError;
use interfaces::{IntegerLiteral, IntegerLiteralKind};

pub(super) fn parse_integer_literal(
    base: u8,
    kind: IntegerLiteralKind,
    string: String,
) -> Result<IntegerLiteral, NumericLiteralError> {
    if string.is_empty() {
        return Err(NumericLiteralError::MissingIntegralPart);
    }

    let value = match kind {
        IntegerLiteralKind::Integer8 => parse_magnitude(base, string)?,
        IntegerLiteralKind::Integer16 => parse_magnitude(base, string)?,
        IntegerLiteralKind::Integer32 => parse_magnitude(base, string)?,
        IntegerLiteralKind::Integer64 => parse_magnitude(base, string)?,
        IntegerLiteralKind::Integer128 => parse_magnitude(base, string)?,
        IntegerLiteralKind::ArchInteger => parse_magnitude(base, string)?,
    };

    Ok(IntegerLiteral { value, kind })
}
