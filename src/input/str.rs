use crate::{input::Input, parser::ParserPosition, span::Span};

impl<'a> Input for &'a str {
    type Token = char;

    type Slice = &'a str;

    #[inline]
    fn len(&self) -> usize {
        str::len(self)
    }

    fn peek_len(
        &self,
        ParserPosition(position): ParserPosition,
        len: usize,
    ) -> Option<Self::Token> {
        self[position..].chars().nth(len)
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
}
