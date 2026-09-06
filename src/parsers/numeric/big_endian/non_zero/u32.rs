use std::num::NonZeroU32;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::{non_zero::BigEndianNonZero, u32::BigEndianU32},
    result::ParseResult,
};

pub type BigEndianNonZeroU32 = BigEndianNonZero<u32>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianNonZeroU32 {
    type Output = NonZeroU32;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU32::parse(parser)?;

        let Some(value) = NonZeroU32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::BigEndianNonZeroU32),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianNonZeroU32 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianNonZeroU32)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<BigEndianNonZeroU32> {
    type Output = Option<NonZeroU32>;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianU32::parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<BigEndianNonZeroU32> {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianU32)
    }
}
