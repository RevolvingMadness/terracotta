use crate::{
    parsable_traits::Parsable,
    result::{hard::HardParseResult, regular::ParseResult, soft::SoftParseResult},
    span::Span,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseFailure(pub(crate) ());

impl ParseFailure {
    #[inline]
    #[must_use]
    pub const fn new_unchecked() -> Self {
        Self(())
    }
}

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

    #[inline]
    pub fn parse<T: Parsable<'input, Context>>(&mut self) -> ParseResult<T> {
        T::parse(self)
    }

    #[inline]
    pub fn expect<T: Parsable<'input, Context>>(&mut self) -> HardParseResult<T> {
        self.expect_named::<T>(|| T::NAME.to_owned())
    }

    #[inline]
    pub fn expect_named<T: Parsable<'input, Context>>(
        &mut self,
        expected: impl FnOnce() -> String,
    ) -> HardParseResult<T> {
        T::expect_named(self, expected)
    }

    pub fn add_error<S: Into<Span>>(&mut self, span: S, message: String) -> ParseFailure {
        let span = span.into();

        self.errors.push(ParseError { span, message });

        ParseFailure::new_unchecked()
    }

    #[inline]
    pub fn add_error_with_length_one(&mut self, message: String) -> ParseFailure {
        self.add_error(self.position..self.position + 1, message)
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
    pub fn advance_char(&mut self) {
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
    pub const fn advance(&mut self) {
        self.position += 1;
    }

    #[inline]
    pub const fn advance_len(&mut self, len: usize) {
        self.position += len;
    }

    #[inline]
    #[must_use]
    pub const fn mark(&self) -> usize {
        self.position
    }

    #[inline]
    #[must_use]
    pub fn slice_from(&self, marker: usize) -> &'input str {
        &self.input[marker..self.position]
    }
}

impl<'input, Context> Parser<'input, Context> {
    pub fn take_while<F>(&mut self, mut predicate: F) -> SoftParseResult<&'input str>
    where
        F: FnMut(char) -> bool,
    {
        let start = self.mark();

        while let Some(character) = self.peek() {
            if !predicate(character) {
                break;
            }

            self.position += character.len_utf8();
        }

        let text = self.slice_from(start);

        if text.is_empty() {
            SoftParseResult::Failure
        } else {
            SoftParseResult::Success(text)
        }
    }
}

impl<'input> Parser<'input, ()> {
    #[inline]
    #[must_use]
    pub const fn new(input: &'input str) -> Self {
        Self::new_with_context(input, ())
    }
}
