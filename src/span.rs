use core::{
    fmt::{self, Display, Formatter},
    ops::{Bound, Range, RangeBounds},
};

use crate::parser::position::ParserPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    start: ParserPosition,
    end: ParserPosition,
}

impl From<Span> for Range<ParserPosition> {
    fn from(Span { start, end }: Span) -> Self {
        start..end
    }
}

impl From<Span> for Range<usize> {
    fn from(Span { start, end }: Span) -> Self {
        start.position()..end.position()
    }
}

impl From<Range<ParserPosition>> for Span {
    fn from(Range { start, end }: Range<ParserPosition>) -> Self {
        Self { start, end }
    }
}

impl RangeBounds<ParserPosition> for Span {
    fn start_bound(&self) -> Bound<&ParserPosition> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&ParserPosition> {
        Bound::Excluded(&self.end)
    }
}

impl RangeBounds<usize> for Span {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(self.start.position_ref())
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Excluded(self.end.position_ref())
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}..{}", self.start, self.end)
        }
    }
}

impl Span {
    #[inline]
    #[must_use]
    pub const fn new_unchecked(start: ParserPosition, end: ParserPosition) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn new(first: ParserPosition, second: ParserPosition) -> Self {
        if first.position() > second.position() {
            Self::new_unchecked(second, first)
        } else {
            Self::new_unchecked(first, second)
        }
    }

    #[inline]
    #[must_use]
    pub const fn start_ref(&self) -> &ParserPosition {
        &self.start
    }

    #[inline]
    #[must_use]
    pub const fn start(&self) -> ParserPosition {
        self.start
    }

    #[inline]
    #[must_use]
    pub const fn end_ref(&self) -> &ParserPosition {
        &self.end
    }

    #[inline]
    #[must_use]
    pub const fn end(&self) -> ParserPosition {
        self.end
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.start.abs_diff(self.end)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.end.position() == self.start.position()
    }

    #[must_use]
    pub const fn expand(mut self, other: ParserPosition) -> Self {
        if other.position() < self.start.position() {
            self.start = other;
        }

        if other.position() > self.end.position() {
            self.end = other;
        }

        self
    }

    #[must_use]
    pub const fn merge(mut self, other: Self) -> Self {
        if other.start.position() < self.start.position() {
            self.start = other.start;
        }

        if other.end.position() > self.end.position() {
            self.end = other.end;
        }

        self
    }
}
