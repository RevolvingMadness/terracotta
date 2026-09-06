use std::num::NonZeroI64;

use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::{i64::BigEndianI64, non_zero::BigEndianNonZero},
    result::ParseResult,
};

pub type BigEndianNonZeroI64 = BigEndianNonZero<i64>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianNonZeroI64 {
    type Output = NonZeroI64;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianNonZeroI64)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI64::parse(parser)?;

        let Some(value) = NonZeroI64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::BigEndianNonZeroI64),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for Option<BigEndianNonZeroI64> {
    type Output = Option<NonZeroI64>;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianI64)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianI64::parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}
