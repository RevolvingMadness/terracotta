use core::fmt::{self, Display, Formatter};

use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserPosition(pub(crate) usize);

impl Display for ParserPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ParserPosition {
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> usize {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn abs_diff(self, other: Self) -> usize {
        other.0.abs_diff(self.0)
    }

    #[inline]
    #[must_use]
    pub const fn span(self, other: Self) -> Span {
        Span::new(self, other)
    }
}
