#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct Span {
    pub start: usize,
    pub length: usize,
}

impl Span {
    /// End exclusive
    pub const fn end(self) -> usize {
        self.start + self.length
    }
}
