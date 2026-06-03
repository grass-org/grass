use crate::Cursor;

pub(super) fn skip_whitespaces(cursor: &mut Cursor) -> SkipWhitespaceResult {
    let mut result = SkipWhitespaceResult::NoneFound;

    loop {
        let Some(character) = cursor.peek() else {
            break;
        };

        if !is_horizontal_whitespace(character) {
            break;
        }

        result = SkipWhitespaceResult::Skipped;
        cursor.advance();
    }

    result
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub(super) enum SkipWhitespaceResult {
    NoneFound,
    Skipped,
}

pub(super) const fn is_horizontal_whitespace(character: char) -> bool {
    matches!(
        character,
        // `Default_Ignorable_Code_Point` characters
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Horizontal space characters
        | '\u{0009}'   // tab (\t)
        | '\u{0020}'
    )
}

pub(super) const fn is_newline(character: char) -> bool {
    matches!(
        character,
        // End-of-line characters
        | '\u{000A}' // line feed (\n)
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // carriage return (\r)
        | '\u{0085}' // next line (from latin1)
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

pub(super) const fn is_whitespace(character: char) -> bool {
    is_horizontal_whitespace(character) || is_newline(character)
}
