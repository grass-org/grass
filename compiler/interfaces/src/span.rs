#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct Span {
    pub start: usize,
    pub length: usize,
}
