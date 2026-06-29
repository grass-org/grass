use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct Spanned<T> {
    pub content: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(content: impl Into<T>, span: Span) -> Self {
        Self {
            content: content.into(),
            span,
        }
    }
}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.content.fmt(f)
    }
}

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
