use core::fmt::{self, Display, Formatter};

use alloc::vec::Vec;

use crate::{
    input::{Input, token::Token},
    result::{HardParseFailure, HardParseResult, OptionTExt, ParseFailure, ParseResult},
    span::Span,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserPosition(pub(crate) usize);

impl Display for ParserPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ParserPosition {
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> usize {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn abs_diff(self, other: Self) -> usize {
        other.0.abs_diff(self.0)
    }

    #[inline]
    #[must_use]
    pub const fn span(self, other: Self) -> Span {
        Span::new(self, other)
    }
}

#[derive(Debug)]
pub struct Parser<'input, I: Input + ?Sized, E> {
    input: &'input I,
    position: usize,
    errors: Vec<(Span, E)>,
}

impl<'input, I: Input + ?Sized, E> Parser<'input, I, E> {
    #[must_use]
    pub const fn new(input: &'input I) -> Self {
        Self {
            input,
            position: 0,
            errors: Vec::new(),
        }
    }

    pub fn add_error_with_span<S: Into<Span>>(&mut self, span: S, error: E) -> HardParseFailure {
        let span = span.into();

        self.errors.push((span, error));

        HardParseFailure(())
    }

    #[inline]
    pub fn add_error_with_span_result<S: Into<Span>, T>(
        &mut self,
        span: S,
        error: E,
    ) -> ParseResult<T> {
        Err(ParseFailure::Hard(self.add_error_with_span(span, error)))
    }

    #[inline]
    pub fn add_error_with_span_hard_result<S: Into<Span>, T>(
        &mut self,
        span: S,
        error: E,
    ) -> HardParseResult<T> {
        Err(self.add_error_with_span(span, error))
    }

    #[inline]
    pub fn add_error(&mut self, error: E) -> HardParseFailure {
        let character_len = self.peek().map_or(0, |token| token.len());

        let start = ParserPosition(self.position);

        let end = ParserPosition(self.position + character_len);

        self.add_error_with_span(start..end, error)
    }

    #[inline]
    pub fn add_error_result<T>(&mut self, error: E) -> ParseResult<T> {
        Err(ParseFailure::Hard(self.add_error(error)))
    }

    #[inline]
    #[must_use]
    pub const fn has_no_errors(&self) -> bool {
        self.errors.is_empty()
    }

    #[inline]
    #[must_use]
    pub const fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn input_len(&self) -> usize {
        self.input.len()
    }

    #[inline]
    #[must_use]
    pub const fn start_position(&self) -> ParserPosition {
        ParserPosition(0)
    }

    #[inline]
    #[must_use]
    pub fn end_position(&self) -> ParserPosition {
        ParserPosition(self.input_len())
    }

    #[inline]
    #[must_use]
    pub const fn consumed_span(&self) -> Span {
        Span {
            start: self.start_position(),
            end: self.position(),
        }
    }

    #[inline]
    #[must_use]
    pub fn remaining_span(&self) -> Span {
        Span {
            start: self.position(),
            end: self.end_position(),
        }
    }

    #[inline]
    #[must_use]
    pub fn whole_input_span(&self) -> Span {
        Span {
            start: self.start_position(),
            end: self.end_position(),
        }
    }

    #[inline]
    #[must_use]
    pub fn consumed(&self) -> &'input I {
        self.input.slice(self.consumed_span())
    }

    #[inline]
    #[must_use]
    pub fn remaining(&self) -> &'input I {
        self.input.slice(self.remaining_span())
    }

    #[inline]
    #[must_use]
    pub fn peek(&self) -> Option<I::Token> {
        self.input.peek(self.position())
    }

    pub fn peek_len(&self, len: usize) -> ParseResult<I::Token> {
        self.input
            .peek_len(self.position(), len)
            .into_parse_result()
    }

    pub fn advance(&mut self) -> Option<I::Token> {
        let token = self.peek()?;

        self.position += token.len();

        Some(token)
    }

    pub fn advance_result(&mut self) -> ParseResult<I::Token> {
        self.advance().into_parse_result()
    }

    pub fn advance_if(
        &mut self,
        predicate: impl FnOnce(&I::Token) -> bool,
    ) -> Option<Option<I::Token>> {
        let token = self.peek()?;

        if predicate(&token) {
            self.position += token.len();

            Some(Some(token))
        } else {
            Some(None)
        }
    }

    pub fn advance_if_result(
        &mut self,
        predicate: impl FnOnce(&I::Token) -> bool,
    ) -> ParseResult<Option<I::Token>> {
        self.advance_if(predicate).into_parse_result()
    }

    #[must_use]
    pub fn advance_len(&mut self, len: usize) -> &'input I {
        let start = self.position();

        for _ in 0..len {
            if self.advance().is_none() {
                break;
            }
        }

        self.input.slice(start.span(self.position()))
    }

    pub fn advance_len_exact(&mut self, len: usize) -> ParseResult<&'input I> {
        let start = self.position();

        for _ in 0..len {
            if self.advance().is_none() {
                self.restore(start);

                return Err(ParseFailure::Soft);
            }
        }

        Ok(self.input.slice(start.span(self.position())))
    }

    #[inline]
    #[must_use]
    pub const fn position(&self) -> ParserPosition {
        ParserPosition(self.position)
    }

    #[inline]
    #[must_use]
    pub fn is_at_end(&self) -> bool {
        self.position() >= self.end_position()
    }

    #[inline]
    pub const fn restore(&mut self, ParserPosition(position): ParserPosition) {
        self.position = position;
    }

    #[inline]
    #[must_use]
    pub fn slice(&self, span: Span) -> &'input I {
        self.input.slice(span)
    }

    #[inline]
    #[must_use]
    pub fn slice_current_to_position(&self, other: ParserPosition) -> &'input I {
        self.slice(self.position().span(other))
    }

    #[inline]
    #[must_use]
    pub fn slice_starts_with(&self, position: ParserPosition, pattern: &'input I) -> bool {
        self.input.starts_with_slice(position, pattern)
    }

    #[inline]
    #[must_use]
    pub fn slice_len(&self, len: usize) -> &'input I {
        self.input.slice_len(self.position(), len)
    }

    #[inline]
    #[must_use]
    pub fn finish(self) -> Vec<(Span, E)> {
        self.errors
    }

    #[inline]
    pub fn take_until<F>(&mut self, mut predicate: F) -> ParseResult<&'input I>
    where
        F: FnMut(&I::Token) -> bool,
    {
        let start = self.position();

        while let Some(token) = self.peek() {
            if predicate(&token) {
                break;
            }

            self.advance();
        }

        let end = self.position();

        if end == start {
            Err(ParseFailure::Soft)
        } else {
            Ok(self.input.slice(Span { start, end }))
        }
    }

    pub fn take_while<F>(&mut self, mut predicate: F) -> ParseResult<&'input I>
    where
        F: FnMut(&I::Token) -> bool,
    {
        self.take_until(|token| !predicate(token))
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use crate::parser::Parser;

    #[test]
    fn parse_take_while() {
        let mut parser = Parser::<_, String>::new("aaab");

        assert_eq!(parser.take_while(|character| *character == 'a'), Ok("aaa"));
        assert_eq!(parser.remaining(), "b");
    }

    #[test]
    fn parse_take_until() {
        let mut parser = Parser::<_, String>::new("abcdef");

        assert_eq!(parser.take_until(|character| *character == 'd'), Ok("abc"));
        assert_eq!(parser.remaining(), "def");
    }
}
