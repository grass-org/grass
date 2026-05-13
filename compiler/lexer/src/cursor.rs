use std::str::Chars;

#[derive(Debug)]
pub(crate) struct Cursor<'source> {
    characters: Chars<'source>,
    index: usize,
    buffer: Option<char>,
}

impl<'source> Cursor<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            characters: source.chars(),
            index: 0,
            buffer: None,
        }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn pop(&mut self) -> Option<char> {
        let character = self.buffer.take().or_else(|| self.characters.next())?;
        self.index += character.len_utf8();
        Some(character)
    }

    pub const fn push(&mut self, character: char) {
        self.buffer = Some(character);
        self.index -= character.len_utf8();
    }
}
