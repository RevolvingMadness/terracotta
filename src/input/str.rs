use crate::{input::Input, parser::ParserPosition, span::Span};

impl Input for str {
    type Token = char;

    #[inline]
    fn len(&self) -> usize {
        Self::len(self)
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
        self[position..].chars().nth(len)
    }

    fn slice_len(&self, ParserPosition(position): ParserPosition, len: usize) -> &Self {
        let remaining = &self[position..];

        let end = remaining
            .char_indices()
            .nth(len)
            .map_or(remaining.len(), |(index, _)| index);

        &remaining[..end]
    }
}
