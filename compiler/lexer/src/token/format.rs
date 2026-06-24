use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

pub(super) fn format_slice<T>(token_spans: &[T], f: &mut Formatter<'_>) -> fmt::Result
where
    T: Display,
{
    write!(f, "[")?;

    for (index, token_span) in token_spans.iter().enumerate() {
        if index != 0 {
            write!(f, ", ")?;
        }

        token_span.fmt(f)?;
    }

    write!(f, "]")
}
