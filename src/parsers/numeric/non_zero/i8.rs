use core::num::NonZeroI8;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    result::ParseResult,
};

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for NonZeroI8 {
    type Output = Self;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = i8::parse(parser)?;

        let Some(value) = Self::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::NonZeroI8),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for NonZeroI8 {
    fn error() -> E {
        E::expected_numeric(NumericExpectation::NonZeroI8)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<NonZeroI8> {
    type Output = Self;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = i8::parse(parser)?;

        Ok(NonZeroI8::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<NonZeroI8> {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::I8)
    }
}
