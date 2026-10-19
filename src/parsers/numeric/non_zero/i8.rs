use core::num::NonZeroI8;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::i8::I8Parser,
    result::ParseResult,
};

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for NonZeroI8 {
    type Output = Self;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = I8Parser.parse(parser)?;

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
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::NonZeroI8)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<NonZeroI8> {
    type Output = Self;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = I8Parser.parse(parser)?;

        Ok(NonZeroI8::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<NonZeroI8> {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::I8)
    }
}
