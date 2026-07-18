use std::ops::{Bound, Range, RangeBounds};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        value.start..value.end
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl RangeBounds<usize> for Span {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Excluded(&self.end)
    }
}

impl Span {
    #[inline]
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Option<Self> {
        if start > end {
            return None;
        }

        Some(Self { start, end })
    }

    #[inline]
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }

    #[inline]
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.end == self.start
    }
}
