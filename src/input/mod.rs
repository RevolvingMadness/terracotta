use crate::{input::token::Token, parser::ParserPosition, span::Span};

pub mod ext;
pub mod slice;
pub mod str;
pub mod token;

pub trait Input {
    type Token: Token + Clone;

    type Slice: ?Sized;

    #[must_use]
    fn len(&self) -> usize;

    #[inline]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    #[must_use]
    fn peek(&self, position: ParserPosition) -> Option<Self::Token> {
        self.peek_len(position, 0)
    }

    #[must_use]
    fn slice(&self, span: Span) -> &Self::Slice;

    #[must_use]
    fn starts_with_slice(&self, position: ParserPosition, pattern: &Self::Slice) -> bool;

    #[must_use]
    fn peek_len(&self, position: ParserPosition, len: usize) -> Option<Self::Token>;

    #[must_use]
    fn slice_len(&self, position: ParserPosition, len: usize) -> &Self::Slice;
}
