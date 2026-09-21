use alloc::{borrow::ToOwned, format, string::String};

use crate::{
    parsable_traits::{ParsableInstance, ParsableInstanceError},
    parse_error::DefaultParseError,
    parser::Parser,
    result::{ParseFailure, ParseResult},
};

pub trait ExpectedStringParseError {
    #[must_use]
    fn expected_string(expected: &str) -> Self;
}

impl ExpectedStringParseError for String {
    #[inline]
    fn expected_string(expected: &str) -> Self {
        format!("expected `{}`", expected)
    }
}

impl ExpectedStringParseError for () {
    #[inline]
    fn expected_string(_: &str) -> Self {}
}

impl ExpectedStringParseError for DefaultParseError {
    #[inline]
    fn expected_string(expected: &str) -> Self {
        Self::ExpectedString(expected.to_owned())
    }
}

impl<'input, E> ParsableInstance<'input, str, E> for &'input str {
    type Output = Self;

    fn instance_parse(&self, parser: &mut Parser<'input, str, E>) -> ParseResult<Self::Output> {
        if !parser.slice_starts_with(parser.position(), self) {
            return Err(ParseFailure::Soft);
        }

        let string = parser.advance_len(self.len());

        Ok(string)
    }
}

impl<E: ExpectedStringParseError> ParsableInstanceError<E> for &str {
    #[inline]
    fn error(&self) -> E {
        E::expected_string(self)
    }
}
