use crate::InvalidRaditError;

pub(super) fn parse_radit(base: u8, character: char) -> Result<u8, InvalidRaditError> {
    if let Some(radit) = character.to_digit(base as u32) {
        return Ok(radit as u8);
    };

    let error = InvalidRaditError {
        base,
        radit: character,
        max_radit: max_radit(base as u32),
    };

    Err(error)
}

fn max_radit(base: u32) -> char {
    if (2..=9).contains(&base) {
        return char::from_digit(base, 10)
            .expect("char::from_digit for radix within 2..=9 somehow failed");
    }

    if (10..=36).contains(&base) {
        let offset = base - 10;
        return char::from_u32('a' as u32 + offset)
            .expect("char::from_u32 for char within 'a'..='z' somehow failed");
    }

    panic!("invalid base; base must be between 2..=32");
}
