use core::num::NonZeroU8;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::u8::U8Parser,
    result::ParseResult,
};

pub struct NonZeroU8Parser;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for NonZeroU8Parser {
    type Output = NonZeroU8;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = U8Parser.parse(parser)?;

        let Some(value) = NonZeroU8::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::NonZeroU8),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for NonZeroU8Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::NonZeroU8)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<NonZeroU8> {
    type Output = Self;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = U8Parser.parse(parser)?;

        Ok(NonZeroU8::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<NonZeroU8> {
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::U8)
    }
}
