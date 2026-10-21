use core::fmt::{self, Display, Formatter};

use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserPosition(usize);

impl Display for ParserPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ParserPosition {
    #[inline]
    #[must_use]
    pub const fn new_unchecked(position: usize) -> Self {
        Self(position)
    }

    #[inline]
    pub const fn advance_unchecked(&mut self, offset: usize) {
        self.0 += offset;
    }

    #[inline]
    #[must_use]
    pub const fn advanced_unchecked(self, offset: usize) -> Self {
        Self::new_unchecked(self.0 + offset)
    }

    #[inline]
    #[must_use]
    pub const fn start() -> Self {
        Self::new_unchecked(0)
    }

    #[inline]
    #[must_use]
    pub const fn position_ref(&self) -> &usize {
        &self.0
    }

    #[inline]
    #[must_use]
    pub const fn position(self) -> usize {
        self.0
    }

    #[must_use]
    pub fn min(self, other: Self) -> Self {
        if other < self {
            other
        } else {
            self
        }
    }

    #[must_use]
    pub fn max(self, other: Self) -> Self {
        if other < self {
            self
        } else {
            other
        }
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
