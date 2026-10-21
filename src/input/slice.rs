use crate::{
    input::{Input, token::Token},
    parser::position::ParserPosition,
    span::Span,
};

impl<T: Token> Input for [T] {
    type Token = T;

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
        self.get(position.advanced_unchecked(len).position())
            .cloned()
    }

    fn slice_len(&self, position: ParserPosition, len: usize) -> &Self {
        let end = (position.advanced_unchecked(len).position()).min(self.len());

        &self[position.position()..end]
    }
}
