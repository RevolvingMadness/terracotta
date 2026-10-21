use alloc::vec::Vec;

use crate::{
    failures::{ParseFailure, hard::HardParseFailure},
    input::{Input, token::Token},
    parser::position::ParserPosition,
    results::{ParseResult, option::OptionTExt},
    span::Span,
    spanned::{Spanned, SpannedExt},
};

pub mod position;

#[derive(Debug)]
pub struct Parser<'input, I, E>
where
    I: Input + ?Sized + 'input,
{
    pub input: &'input I,
    pub position: ParserPosition,
    errors: Vec<Spanned<E>>,
}

impl<'input, I, E> Parser<'input, I, E>
where
    I: Input + ?Sized + 'input,
{
    #[must_use]
    pub const fn new(input: &'input I) -> Self {
        Self {
            input,
            position: ParserPosition::start(),
            errors: Vec::new(),
        }
    }

    pub fn add_error_with_span<S: Into<Span>, E2: Into<E>>(
        &mut self,
        span: S,
        error: E2,
    ) -> HardParseFailure {
        let span = span.into();

        let error = error.into();

        self.errors.push(error.with_span(span));

        HardParseFailure(())
    }

    #[inline]
    pub fn add_error<E2: Into<E>>(&mut self, error: E2) -> HardParseFailure {
        let character_len = self.peek().map_or(0, |token| token.len());

        let start = self.position;

        let end = start.advanced_unchecked(character_len);

        self.add_error_with_span(start..end, error)
    }

    #[inline]
    pub fn add_error_result<E2: Into<E>, T>(&mut self, error: E2) -> ParseResult<T> {
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
        ParserPosition::start()
    }

    #[inline]
    #[must_use]
    pub fn end_position(&self) -> ParserPosition {
        ParserPosition::new_unchecked(self.input_len())
    }

    #[inline]
    #[must_use]
    pub const fn consumed_span(&self) -> Span {
        Span::new_unchecked(self.start_position(), self.position)
    }

    #[inline]
    #[must_use]
    pub fn remaining_span(&self) -> Span {
        Span::new(self.position, self.end_position())
    }

    #[inline]
    #[must_use]
    pub fn whole_input_span(&self) -> Span {
        Span::new_unchecked(self.start_position(), self.end_position())
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
        self.input.peek(self.position)
    }

    pub fn peek_len(&self, len: usize) -> ParseResult<I::Token> {
        self.input.peek_len(self.position, len).into_parse_result()
    }

    pub fn advance(&mut self) -> Option<I::Token> {
        let token = self.peek()?;

        self.position.advance_unchecked(token.len());

        Some(token)
    }

    pub fn advance_if(&mut self, predicate: impl FnOnce(&I::Token) -> bool) -> Option<I::Token> {
        let token = self.peek()?;

        if predicate(&token) {
            self.position.advance_unchecked(token.len());

            Some(token)
        } else {
            None
        }
    }

    #[must_use]
    pub fn advance_len(&mut self, len: usize) -> &'input I {
        let start = self.position;

        for _ in 0..len {
            if self.advance().is_none() {
                break;
            }
        }

        self.input.slice(start.span(self.position))
    }

    pub fn advance_len_exact(&mut self, len: usize) -> ParseResult<&'input I> {
        let start = self.position;

        for _ in 0..len {
            if self.advance().is_none() {
                self.position = start;

                return Err(ParseFailure::Soft);
            }
        }

        Ok(self.input.slice(start.span(self.position)))
    }

    #[inline]
    #[must_use]
    pub fn is_at_end(&self) -> bool {
        self.position >= self.end_position()
    }

    #[inline]
    #[must_use]
    pub fn slice(&self, span: Span) -> &'input I {
        self.input.slice(span)
    }

    #[inline]
    #[must_use]
    pub fn slice_current_to_position(&self, other: ParserPosition) -> &'input I {
        self.slice(self.position.span(other))
    }

    #[inline]
    #[must_use]
    pub fn slice_starts_with(&self, position: ParserPosition, pattern: &'input I) -> bool {
        self.input.starts_with_slice(position, pattern)
    }

    #[inline]
    #[must_use]
    pub fn slice_len(&self, len: usize) -> &'input I {
        self.input.slice_len(self.position, len)
    }

    #[inline]
    #[must_use]
    pub fn finish(self) -> Vec<Spanned<E>> {
        self.errors
    }

    pub fn take_until<F>(&mut self, mut predicate: F) -> ParseResult<&'input I>
    where
        F: FnMut(&I::Token) -> bool,
    {
        let start = self.position;

        while let Some(token) = self.peek() {
            if predicate(&token) {
                break;
            }

            self.position.advance_unchecked(token.len());
        }

        let end = self.position;

        if end == start {
            Err(ParseFailure::Soft)
        } else {
            Ok(self.slice(Span::new(start, end)))
        }
    }

    #[inline]
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
