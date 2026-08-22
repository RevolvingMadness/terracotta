use crate::{
    input::{Input, token::Token},
    parser::ParserPosition,
    span::Span,
};

impl<'input, T: Token> Input for &'input [T] {
    type Token = &'input T;

    type Slice = &'input [T];

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
    ) -> Self::Slice {
        &self[start..end]
    }

    #[inline]
    fn starts_with_slice(
        &self,
        ParserPosition(position): ParserPosition,
        pattern: Self::Slice,
    ) -> bool {
        self[position..].starts_with(pattern)
    }

    fn peek_len(
        &self,
        ParserPosition(position): ParserPosition,
        len: usize,
    ) -> Option<Self::Token> {
        self.get(position + len)
    }

    fn slice_len(&self, ParserPosition(position): ParserPosition, len: usize) -> Self::Slice {
        let end = (position + len).min(self.len());

        &self[position..end]
    }
}
