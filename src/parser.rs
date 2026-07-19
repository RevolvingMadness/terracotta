use crate::{
    parsable_traits::Parsable,
    result::{HardParseFailure, HardParseResult, ParseResult, SoftParseFailure, SoftParseResult},
    span::Span,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserPosition(usize);

impl ParserPosition {
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> usize {
        self.0
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
    pub fn parse<P: Parsable<'input, Context>>(&mut self) -> ParseResult<P::Output> {
        P::parse(self)
    }

    #[inline]
    pub fn expect<P: Parsable<'input, Context>>(&mut self) -> HardParseResult<P::Output> {
        self.expect_named::<P>(|| P::NAME.to_owned())
    }

    #[inline]
    pub fn expect_named<P: Parsable<'input, Context>>(
        &mut self,
        expected: impl FnOnce() -> String,
    ) -> HardParseResult<P::Output> {
        P::expect_named(self, expected)
    }

    pub fn add_error<S: Into<Span>>(&mut self, span: S, message: String) -> HardParseFailure {
        let span = span.into();

        self.errors.push(ParseError { span, message });

        HardParseFailure::new_unchecked()
    }

    #[inline]
    pub fn add_error_with_length_one(&mut self, message: String) -> HardParseFailure {
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
    pub const fn mark(&self) -> ParserPosition {
        ParserPosition(self.position)
    }

    #[inline]
    pub const fn restore(&mut self, ParserPosition(position): ParserPosition) {
        self.position = position;
    }

    #[inline]
    #[must_use]
    pub fn slice_from(&self, ParserPosition(position): ParserPosition) -> &'input str {
        &self.input[position..self.position]
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
