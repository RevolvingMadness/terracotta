use crate::{
    parsable_traits::Parsable,
    result::{
        HardParseFailure, HardParseResult, OptionParseResult, ParseResult, SoftParseFailure,
        SoftParseResult,
    },
    span::Span,
};

#[derive(Debug)]
pub struct FullParseResult<Context, Output> {
    pub errors: Vec<ParseError>,
    pub context: Context,

    pub output: Option<Output>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserPosition(pub(crate) usize);

impl ParserPosition {
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> usize {
        self.0
    }

    #[inline]
    #[must_use]
    pub const fn distance_between_self_and(self, other: Self) -> usize {
        other.0.abs_diff(self.0)
    }

    #[inline]
    #[must_use]
    pub const fn span(self, other: Self) -> Span {
        Span::new(self, other)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CalledFromParser(pub(crate) ());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
}

#[derive(Debug)]
pub struct Parser<'input, Context = ()> {
    input: &'input str,
    position: usize,
    errors: Vec<ParseError>,

    pub context: Context,
}

impl<'input, Context> Parser<'input, Context> {
    #[must_use]
    pub const fn new_with_context(input: &'input str, context: Context) -> Self {
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
    pub fn parse<P: Parsable<'input, Context>>(&mut self) -> ParseResult<P::Output> {
        P::parse(self, CalledFromParser(()))
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
    pub fn try_parse<P: Parsable<'input, Context>>(&mut self) -> OptionParseResult<P::Output> {
        P::try_parse(self, CalledFromParser(()))
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
    pub fn expect<P: Parsable<'input, Context>>(&mut self) -> HardParseResult<P::Output> {
        P::expect(self, CalledFromParser(()))
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
    pub fn expect_named<P: Parsable<'input, Context>>(
        &mut self,
        expected: &str,
    ) -> HardParseResult<P::Output> {
        P::expect_named(self, CalledFromParser(()), expected)
    }

    pub fn parse_fully<P: Parsable<'input, Context>>(
        mut self,
        allow_trailing: bool,
    ) -> FullParseResult<Context, P::Output> {
        let result = self.try_parse::<P>();

        if !allow_trailing && !self.remaining().is_empty() {
            self.add_error_with_length_one("Expected end of input".to_owned());
        }

        let output = result.ok().flatten();

        FullParseResult {
            context: self.context,
            errors: self.errors,
            output,
        }
    }

    pub fn add_error<S: Into<Span>>(&mut self, span: S, message: String) -> HardParseFailure {
        let span = span.into();

        self.errors.push(ParseError { span, message });

        HardParseFailure(())
    }

    #[inline]
    pub fn add_error_with_length_one(&mut self, message: String) -> HardParseFailure {
        self.add_error(
            ParserPosition(self.position)..ParserPosition(self.position + 1),
            message,
        )
    }

    #[inline]
    #[must_use]
    pub fn consumed(&self) -> &'input str {
        &self.input[..self.position]
    }

    #[inline]
    #[must_use]
    pub fn remaining(&self) -> &'input str {
        &self.input[self.position..]
    }

    #[must_use]
    pub fn peek(&self) -> Option<char> {
        if self.position > self.input.len() {
            return None;
        }

        self.input[self.position..].chars().next()
    }

    #[must_use]
    pub fn peek_len(&self, len: usize) -> Option<char> {
        self.input[self.position..].chars().nth(len)
    }

    #[inline]
    pub fn advance(&mut self) {
        let Some(character) = self.peek() else {
            return;
        };

        self.position += character.len_utf8();
    }

    pub fn advance_chars(&mut self, number_of_chars: usize) {
        let mut chars = self.input.chars();

        for _ in 0..number_of_chars {
            let Some(character) = chars.next() else {
                break;
            };

            self.position += character.len_utf8();
        }
    }

    #[inline]
    pub const fn advance_char(&mut self, character: char) {
        self.position += character.len_utf8();
    }

    #[inline]
    pub const fn advance_str(&mut self, str: &str) {
        self.position += str.len();
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
    pub fn slice(
        &self,
        Span {
            start: ParserPosition(start),
            end: ParserPosition(end),
        }: Span,
    ) -> &'input str {
        &self.input[start..end]
    }

    #[inline]
    #[must_use]
    pub fn slice_position(&self, other: ParserPosition) -> &'input str {
        self.slice(self.position().span(other))
    }
}

impl<'input, Context> Parser<'input, Context> {
    pub fn take_while<F>(&mut self, mut predicate: F) -> SoftParseResult<&'input str>
    where
        F: FnMut(char) -> bool,
    {
        let start = self.position();

        while let Some(character) = self.peek() {
            if !predicate(character) {
                break;
            }

            self.position += character.len_utf8();
        }

        let text = self.slice_position(start);

        if text.is_empty() {
            Err(SoftParseFailure)
        } else {
            Ok(text)
        }
    }

    #[inline]
    pub fn take_until<F>(&mut self, mut predicate: F) -> SoftParseResult<&'input str>
    where
        F: FnMut(char) -> bool,
    {
        self.take_while(|character| !predicate(character))
    }
}

impl<'input> Parser<'input, ()> {
    #[inline]
    #[must_use]
    pub const fn new(input: &'input str) -> Self {
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
