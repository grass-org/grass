use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub(super) struct Cursor<'source> {
    characters: Peekable<Chars<'source>>,
    index: usize,
}

impl<'source> Cursor<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            characters: source.chars().peekable(),
            index: 0,
        }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn peek(&mut self) -> Option<char> {
        self.characters.peek().cloned()
    }

    #[must_use]
    pub fn pop(&mut self) -> Option<char> {
        let character = self.characters.next()?;
        self.index += character.len_utf8();
        Some(character)
    }

    pub fn advance(&mut self) {
        _ = self.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::Cursor;

    #[test]
    fn index_starts_at_0() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let cursor = Cursor::new(source);

        let expected = 0;
        let actual = cursor.index();

        assert_eq!(expected, actual);
    }

    #[test]
    fn pop_advances_index() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        _ = cursor.pop();
        _ = cursor.pop();

        let expected = 2;
        let actual = cursor.index();

        assert_eq!(expected, actual);
    }

    #[test]
    fn advance_advances_index() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        cursor.advance();
        cursor.advance();

        let expected = 2;
        let actual = cursor.index();

        assert_eq!(expected, actual);
    }

    #[test]
    fn peeks_from_start() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        let expected = Some('1');
        let actual = cursor.peek();

        assert_eq!(expected, actual);
    }

    #[test]
    fn peek_does_not_consume() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        _ = cursor.peek();
        _ = cursor.peek();
        _ = cursor.peek();
        _ = cursor.peek();

        let expected = Some('1');
        let actual = cursor.peek();

        assert_eq!(expected, actual);
    }

    #[test]
    fn pops_from_start() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        let expected = Some('1');
        let actual = cursor.pop();

        assert_eq!(expected, actual);
    }

    #[test]
    fn pop_consumes() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        _ = cursor.pop();
        _ = cursor.pop();
        _ = cursor.pop();

        let expected = Some('2');
        let actual = cursor.pop();

        assert_eq!(expected, actual);
    }

    #[test]
    fn advance_consumes() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        cursor.advance();
        cursor.advance();
        cursor.advance();

        let expected = Some('2');
        let actual = cursor.pop();

        assert_eq!(expected, actual);
    }

    #[test]
    fn complete_source() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        for character in source.chars() {
            let expected = Some(character);
            let actual = cursor.pop();

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn no_excess() {
        let source = "1, 2, 3, 4, 5, 6, 7";
        let mut cursor = Cursor::new(source);

        for _ in source.chars() {
            cursor.advance();
        }

        let expected = None;
        let actual = cursor.pop();

        assert_eq!(expected, actual);
    }

    #[test]
    fn handles_multi_byte_unicode_indices() {
        // '🦀' is 4 bytes. 'a' is 1 byte.
        let source = "🦀a";
        let mut cursor = Cursor::new(source);

        // 1. Check start
        assert_eq!(cursor.index(), 0);

        // 2. Pop the crab (4 bytes)
        assert_eq!(cursor.pop(), Some('🦀'));

        // Crucial check: Index should leap by 4, not 1!
        let expected_index_after_crab = 4;
        assert_eq!(cursor.index(), expected_index_after_crab);

        // 3. Pop the 'a' (1 byte)
        assert_eq!(cursor.pop(), Some('a'));
        assert_eq!(cursor.index(), 5);
    }
}
