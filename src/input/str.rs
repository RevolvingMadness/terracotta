use crate::{input::Input, parser::position::ParserPosition, span::Span};

impl Input for str {
    type Token = char;

    #[inline]
    fn len(&self) -> usize {
        Self::len(self)
    }

    #[inline]
    fn slice(&self, span: Span) -> &Self {
        &self[span.start().position()..span.end().position()]
    }

    #[inline]
    fn starts_with_slice(&self, position: ParserPosition, pattern: &Self) -> bool {
        self[position.position()..].starts_with(pattern)
    }

    fn peek_len(&self, position: ParserPosition, len: usize) -> Option<Self::Token> {
        self[position.position()..].chars().nth(len)
    }

    fn slice_len(&self, position: ParserPosition, len: usize) -> &Self {
        let remaining = &self[position.position()..];

        let end = remaining
            .char_indices()
            .nth(len)
            .map_or(remaining.len(), |(index, _)| index);

        &remaining[..end]
    }
}
