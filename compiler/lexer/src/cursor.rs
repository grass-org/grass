use std::{collections::LinkedList, str::Chars};

#[derive(Debug)]
pub(crate) struct Cursor<'source> {
    characters: Chars<'source>,
    index: usize,
    buffer: LinkedList<char>,
}

impl<'source> Cursor<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            characters: source.chars(),
            index: 0,
            buffer: LinkedList::new(),
        }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn pop(&mut self) -> Option<char> {
        let character = self.buffer.pop_front().or_else(|| self.characters.next())?;
        self.index += character.len_utf8();
        Some(character)
    }

    pub fn push(&mut self, character: char) {
        self.buffer.push_front(character);
        self.index -= character.len_utf8();
    }
}
