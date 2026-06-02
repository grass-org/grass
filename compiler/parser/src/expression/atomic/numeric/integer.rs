use crate::expression::atomic::numeric::magnitude::parse_magnitude;
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

    let literal = match kind {
        IntegerLiteralKind::Integer8 => parse_magnitude::<i8>(base, kind, string)?.into(),
        IntegerLiteralKind::Integer16 => parse_magnitude::<i16>(base, kind, string)?.into(),
        IntegerLiteralKind::Integer32 => parse_magnitude::<i32>(base, kind, string)?.into(),
        IntegerLiteralKind::Integer64 => parse_magnitude::<i64>(base, kind, string)?.into(),
        IntegerLiteralKind::Integer128 => parse_magnitude::<i128>(base, kind, string)?.into(),
        IntegerLiteralKind::ArchInteger => parse_magnitude::<isize>(base, kind, string)?.into(),
    };

    Ok(literal)
}
