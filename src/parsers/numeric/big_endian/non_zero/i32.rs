use std::num::NonZeroI32;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::{i32::BigEndianI32, non_zero::BigEndianNonZero},
    result::ParseResult,
};

pub type BigEndianNonZeroI32 = BigEndianNonZero<i32>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianNonZeroI32 {
    type Output = NonZeroI32;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI32::parse(parser)?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::BigEndianNonZeroI32),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianNonZeroI32 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianNonZeroI32)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<BigEndianNonZeroI32> {
    type Output = Option<NonZeroI32>;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianI32::parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<BigEndianNonZeroI32> {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianI32)
    }
}
