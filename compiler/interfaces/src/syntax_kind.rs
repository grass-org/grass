#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum SyntaxKind {
    Expression,
    Operator,
    OpenParenthesis,
    CloseParenthesis,
}
