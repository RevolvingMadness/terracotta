use std::num::NonZeroU64;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::{non_zero::BigEndianNonZero, u64::BigEndianU64},
    result::ParseResult,
};

pub type BigEndianNonZeroU64 = BigEndianNonZero<u64>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianNonZeroU64 {
    type Output = NonZeroU64;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU64::parse(parser)?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::BigEndianNonZeroU64),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianNonZeroU64 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianNonZeroU64)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<BigEndianNonZeroU64> {
    type Output = Option<NonZeroU64>;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianU64::parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<BigEndianNonZeroU64> {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianU64)
    }
}
