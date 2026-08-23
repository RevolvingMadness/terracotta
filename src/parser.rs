use std::fmt::{self, Display, Formatter};

use crate::{
    input::{Input, token::Token},
    result::{
        HardParseFailure, HardParseResult, ParseFailure, ParseResult, SoftParseFailure,
        SoftParseResult,
    },
    span::Span,
};

#[derive(Debug)]
pub struct FullParseResult<Output> {
    pub errors: Vec<ParseError>,
    pub output: Option<Output>,
}

impl<Output> FullParseResult<Output> {
    #[track_caller]
    pub fn unwrap(self) -> Output {
        let Some(output) = self.output else {
            panic!("called `FullParseResult::unwrap()` on a `None` output full parse result")
        };

        assert!(
            self.errors.is_empty(),
            "called `FullParseResult::unwrap()` on a non-zero error full parse result"
        );

        output
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.span, self.message)
    }
}

pub type StrParser<'input> = Parser<'input, str>;

pub type ByteParser<'input> = Parser<'input, [u8]>;

#[derive(Debug)]
pub struct Parser<'input, I: Input + ?Sized> {
    input: &'input I,
    position: usize,
    errors: Vec<ParseError>,
}

impl<'input, I: Input + ?Sized> Parser<'input, I> {
    #[must_use]
    pub const fn new(input: &'input I) -> Self {
        Self {
            input,
            position: 0,
            errors: Vec::new(),
        }
    }

    pub fn add_error_with_span<S: Into<Span>>(
        &mut self,
        span: S,
        message: String,
    ) -> HardParseFailure {
        let span = span.into();

        self.errors.push(ParseError { span, message });

        HardParseFailure(())
    }

    #[inline]
    pub fn add_error_with_span_result<T, S: Into<Span>>(
        &mut self,
        span: S,
        message: String,
    ) -> ParseResult<T> {
        Err(ParseFailure::Hard(self.add_error_with_span(span, message)))
    }

    #[inline]
    pub fn add_error_with_span_hard_result<T, S: Into<Span>>(
        &mut self,
        span: S,
        message: String,
    ) -> HardParseResult<T> {
        Err(self.add_error_with_span(span, message))
    }

    #[inline]
    pub fn add_error(&mut self, message: String) -> HardParseFailure {
        let character_len = self.peek().map_or(0, |token| token.len());

        let start = ParserPosition(self.position);

        let end = ParserPosition(self.position + character_len);

        self.add_error_with_span(start..end, message)
    }

    #[inline]
    pub fn add_error_result<T>(&mut self, message: String) -> ParseResult<T> {
        Err(ParseFailure::Hard(self.add_error(message)))
    }

    #[inline]
    pub fn add_error_hard_result<T>(&mut self, message: String) -> HardParseResult<T> {
        Err(self.add_error(message))
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
    pub fn consumed(&self) -> &I::Slice {
        self.input.slice(self.consumed_span())
    }

    #[inline]
    #[must_use]
    pub fn remaining(&self) -> &I::Slice {
        self.input.slice(self.remaining_span())
    }

    #[inline]
    #[must_use]
    pub fn peek(&self) -> Option<I::Token> {
        self.input.peek(self.position())
    }

    #[must_use]
    pub fn peek_len(&self, len: usize) -> Option<I::Token> {
        self.input.peek_len(self.position(), len)
    }

    #[inline]
    pub fn advance(&mut self) -> Option<I::Token> {
        let token = self.peek()?;

        self.advance_token(&token);

        Some(token)
    }

    #[inline]
    pub fn advance_token(&mut self, token: &impl Token) {
        self.position += token.len();
    }

    #[must_use]
    pub fn advance_len(&mut self, len: usize) -> &I::Slice {
        let start = self.position();

        for _ in 0..len {
            let Some(token) = self.peek() else {
                break;
            };

            self.advance_token(&token);
        }

        self.input.slice(start.span(self.position()))
    }

    #[must_use]
    pub fn advance_len_exact(&mut self, len: usize) -> Option<&I::Slice> {
        let start = self.position();

        for _ in 0..len {
            let Some(token) = self.peek() else {
                self.restore(start);

                return None;
            };

            self.advance_token(&token);
        }

        Some(self.input.slice(start.span(self.position())))
    }

    #[inline]
    #[must_use]
    pub const fn position(&self) -> ParserPosition {
        ParserPosition(self.position)
    }

    #[inline]
    pub const fn restore(&mut self, ParserPosition(position): ParserPosition) {
        self.position = position;
    }

    #[inline]
    #[must_use]
    pub fn slice(&self, span: Span) -> &I::Slice {
        self.input.slice(span)
    }

    #[inline]
    #[must_use]
    pub fn slice_current_to_position(&self, other: ParserPosition) -> &I::Slice {
        self.slice(self.position().span(other))
    }

    #[inline]
    #[must_use]
    pub fn slice_starts_with(&self, position: ParserPosition, pattern: &I::Slice) -> bool {
        self.input.starts_with_slice(position, pattern)
    }

    #[inline]
    #[must_use]
    pub fn slice_len(&self, len: usize) -> &I::Slice {
        self.input.slice_len(self.position(), len)
    }

    #[inline]
    #[must_use]
    pub fn finish(self) -> Vec<ParseError> {
        self.errors
    }

    #[inline]
    pub fn take_until<F>(&mut self, mut predicate: F) -> SoftParseResult<&I::Slice>
    where
        F: FnMut(I::Token) -> bool,
    {
        let start = self.position();

        while let Some(token) = self.peek() {
            let token_len = token.len();

            if predicate(token) {
                break;
            }

            self.position += token_len;
        }

        let end = self.position();

        if end == start {
            Err(SoftParseFailure)
        } else {
            Ok(self.input.slice(Span { start, end }))
        }
    }

    pub fn take_while<F>(&mut self, mut predicate: F) -> SoftParseResult<&I::Slice>
    where
        F: FnMut(I::Token) -> bool,
    {
        self.take_until(|token| !predicate(token))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn parse_take_while() {
        let mut parser = Parser::new("aaab");

        assert_eq!(parser.take_while(|character| character == 'a'), Ok("aaa"));
        assert_eq!(parser.remaining(), "b");
    }

    #[test]
    fn parse_take_until() {
        let mut parser = Parser::new("abcdef");

        assert_eq!(parser.take_until(|character| character == 'd'), Ok("abc"));
        assert_eq!(parser.remaining(), "def");
    }
}
