use crate::Token;

pub(crate) trait TokenBuilder: Sized {
    #[must_use]
    fn push(&mut self, character: char) -> PushTokenCharacterResult;

    fn build(self) -> Token;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub(crate) enum PushTokenCharacterResult {
    Successful,
    Failed,
}
