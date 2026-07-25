use std::ops::{Bound, Range, RangeBounds};

use crate::parser::ParserPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub(crate) start: ParserPosition,
    pub(crate) end: ParserPosition,
}

impl From<Span> for Range<ParserPosition> {
    fn from(Span { start, end }: Span) -> Self {
        start..end
    }
}

impl From<Span> for Range<usize> {
    fn from(
        Span {
            start: ParserPosition(start),
            end: ParserPosition(end),
        }: Span,
    ) -> Self {
        start..end
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
        Bound::Included(&self.start.0)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Excluded(&self.end.0)
    }
}

impl Span {
    #[must_use]
    pub const fn new(first: ParserPosition, second: ParserPosition) -> Self {
        if first.0 > second.0 {
            Self {
                start: second,
                end: first,
            }
        } else {
            Self {
                start: first,
                end: second,
            }
        }
    }

    #[inline]
    #[must_use]
    pub const fn start(&self) -> ParserPosition {
        self.start
    }

    #[inline]
    #[must_use]
    pub const fn end(&self) -> ParserPosition {
        self.end
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.start.distance_between_self_and(self.end)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.end.0 == self.start.0
    }

    #[must_use]
    pub const fn expand(mut self, other: ParserPosition) -> Self {
        if other.0 < self.start.0 {
            self.start = other;
        }

        if other.0 > self.end.0 {
            self.end = other;
        }

        self
    }

    #[must_use]
    pub const fn merge(mut self, other: Self) -> Self {
        if other.start.0 < self.start.0 {
            self.start = other.start;
        }

        if other.end.0 > self.end.0 {
            self.end = other.end;
        }

        self
    }
}
