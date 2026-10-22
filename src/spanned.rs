use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Spanned<T: ?Sized> {
    pub span: Span,
    pub value: T,
}

impl<T> Spanned<T> {
    #[inline]
    #[must_use]
    pub const fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }
}

pub trait SpannedExt {
    #[must_use]
    fn with_span(self, span: Span) -> Spanned<Self>;
}

impl<T> SpannedExt for T {
    #[inline]
    fn with_span(self, span: Span) -> Spanned<Self> {
        Spanned::new(span, self)
    }
}
