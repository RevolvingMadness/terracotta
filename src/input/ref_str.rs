use crate::{input::Input, parser::ParserPosition, span::Span};

impl<'a> Input for &'a str {
    type Token = char;

    type Slice = &'a str;

    #[inline]
    fn len(&self) -> usize {
        str::len(self)
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
        self[position..].chars().nth(len)
    }

    fn slice_len(&self, ParserPosition(position): ParserPosition, len: usize) -> Self::Slice {
        let remaining = &self[position..];

        let end = remaining
            .char_indices()
            .nth(len)
            .map_or(remaining.len(), |(index, _)| index);

        &remaining[..end]
    }
}
