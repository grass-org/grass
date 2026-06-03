use crate::{LexError, LexResult};

pub(super) trait Fallback {
    fn fallback(self, function: impl FnOnce() -> LexResult) -> LexResult;
}

impl Fallback for LexResult {
    fn fallback(self, function: impl FnOnce() -> LexResult) -> LexResult {
        match self {
            ok @ Ok(_) => ok,
            Err(error) => fallback(error, function),
        }
    }
}

fn fallback(error: LexError, function: impl FnOnce() -> LexResult) -> LexResult {
    match error {
        LexError::Skipped => function(),
        error @ LexError::EndOfSource => Err(error),
    }
}
