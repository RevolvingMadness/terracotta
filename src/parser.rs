use std::fmt::{self, Display, Formatter};

use crate::{
    input::{Input, token::Token},
    parsable_traits::Parsable,
    result::{
        HardParseFailure, HardParseResult, OptionParseResult, ParseFailure, ParseResult,
        SoftParseFailure, SoftParseResult,
    },
    span::Span,
};

#[derive(Debug)]
pub struct FullParseResult<Context, Output> {
    pub errors: Vec<ParseError>,
    pub context: Context,

    pub output: Option<Output>,
}

impl<Context, Output> FullParseResult<Context, Output> {
    #[track_caller]
    pub fn unwrap(self) -> (Context, Output) {
        let Some(output) = self.output else {
            panic!("called `FullParseResult::unwrap()` on a `None` output value")
        };

        assert!(
            self.errors.is_empty(),
            "Errors should be empty if output is `None`"
        );

        (self.context, output)
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

pub type StrParser<'input, Context> = Parser<&'input str, Context>;
pub type StrParserNoContext<'input> = Parser<&'input str>;

pub type ByteParser<'input, Context> = Parser<&'input [u8], Context>;
pub type ByteParserNoContext<'input> = Parser<&'input [u8]>;

#[derive(Debug)]
pub struct Parser<I: Input, Context = ()> {
    pub(crate) input: I,
    pub(crate) position: usize,
    pub(crate) errors: Vec<ParseError>,

    pub context: Context,
}

impl<I: Input, Context> Parser<I, Context> {
    #[must_use]
    pub const fn new_with_context(input: I, context: Context) -> Self {
        Self {
            input,
            position: 0,
            errors: Vec::new(),
            context,
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
           name = "parse",
           level = "debug",
           skip(self),
           fields(
               rule = P::NAME,
               position = self.position().0,
           ),
       ))]
    #[inline]
    pub fn parse<P: Parsable<I, Context>>(&mut self) -> ParseResult<P::Output> {
        P::parse(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
           name = "try_parse",
           level = "debug",
           skip(self),
           fields(
               rule = P::NAME,
               position = self.position().0,
           ),
       ))]
    #[inline]
    pub fn try_parse<P: Parsable<I, Context>>(&mut self) -> OptionParseResult<P::Output> {
        P::try_parse(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
           name = "expect",
           level = "debug",
           skip(self),
           fields(
               rule = P::NAME,
               position = self.position().0,
           ),
       ))]
    #[inline]
    pub fn expect<P: Parsable<I, Context>>(&mut self) -> HardParseResult<P::Output> {
        P::expect(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
           name = "expect_named",
           level = "debug",
           skip(self),
           fields(
               rule = P::NAME,
               expected = expected,
               position = self.position().0,
           ),
       ))]
    #[inline]
    pub fn expect_named<P: Parsable<I, Context>>(
        &mut self,
        expected: &str,
    ) -> HardParseResult<P::Output> {
        P::expect_named(self, expected)
    }

    pub fn parse_fully<P: Parsable<I, Context>>(
        mut self,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output> {
        let result = self.try_parse::<P>();

        if !allow_trailing && self.errors.is_empty() && self.position < self.input.len() {
            self.add_error("Expected end of input".to_owned());
        }

        let output = result.ok().flatten();

        FullParseResult {
            context: self.context,
            errors: self.errors,
            output,
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
    pub fn consumed(&self) -> I::Slice {
        self.input.slice(Span {
            start: ParserPosition(0),
            end: self.position(),
        })
    }

    #[inline]
    #[must_use]
    pub fn remaining(&self) -> I::Slice {
        self.input.slice(self.whole_input_span())
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
    pub fn advance_len(&mut self, len: usize) -> I::Slice {
        let start = self.position();

        for _ in 0..len {
            let Some(token) = self.peek() else {
                break;
            };

            self.advance_token(&token);
        }

        let end = self.position();

        self.input.slice(Span { start, end })
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
    pub fn slice(&self, span: Span) -> I::Slice {
        self.input.slice(span)
    }

    #[inline]
    #[must_use]
    pub fn slice_position(&self, other: ParserPosition) -> I::Slice {
        self.slice(self.position().span(other))
    }

    #[inline]
    #[must_use]
    pub fn slice_ahead(&self, len: usize) -> I::Slice {
        self.input.slice_len(self.position(), len)
    }

    #[inline]
    pub fn take_until<F>(&mut self, mut predicate: F) -> SoftParseResult<I::Slice>
    where
        F: FnMut(I::Token) -> bool,
    {
        let start = self.position();

        while let Some(token) = self.peek() {
            let token_len = token.len();

            if !predicate(token) {
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

    pub fn take_while<F>(&mut self, mut predicate: F) -> SoftParseResult<I::Slice>
    where
        F: FnMut(I::Token) -> bool,
    {
        self.take_until(|token| !predicate(token))
    }
}

impl<'input, T: Token, Context> Parser<&'input [T], Context> {
    #[must_use]
    pub fn advance_len_exact<const LEN: usize>(&mut self) -> Option<[T; LEN]>
    where
        [T; LEN]: TryFrom<&'input [T]>,
    {
        let start = self.position();

        for _ in 0..LEN {
            let Some(token) = self.peek() else {
                self.restore(start);

                return None;
            };

            self.advance_token(&token);
        }

        let end = self.position();

        let slice = self.input.slice(Span { start, end });

        Some(<[T; LEN]>::try_from(slice).ok().unwrap())
    }
}

impl<I: Input> Parser<I, ()> {
    #[inline]
    #[must_use]
    pub const fn new(input: I) -> Self {
        Self::new_with_context(input, ())
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
