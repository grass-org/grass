use std::fmt::{self, Display, Formatter};

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

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end())
    }
}
