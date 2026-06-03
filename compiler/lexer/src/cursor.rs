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
