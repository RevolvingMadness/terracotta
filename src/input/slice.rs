use crate::{
    input::{Input, token::Token},
    parser::ParserPosition,
    span::Span,
};

impl<T: Token + Clone> Input for [T] {
    type Token = T;

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    #[inline]
    fn slice(
        &self,
        Span {
            start: ParserPosition(start),
            end: ParserPosition(end),
        }: Span,
    ) -> &Self {
        &self[start..end]
    }

    #[inline]
    fn starts_with_slice(&self, ParserPosition(position): ParserPosition, pattern: &Self) -> bool {
        self[position..].starts_with(pattern)
    }

    fn peek_len(
        &self,
        ParserPosition(position): ParserPosition,
        len: usize,
    ) -> Option<Self::Token> {
        self.get(position + len).cloned()
    }

    fn slice_len(&self, ParserPosition(position): ParserPosition, len: usize) -> &Self {
        let end = (position + len).min(self.len());

        &self[position..end]
    }
}
