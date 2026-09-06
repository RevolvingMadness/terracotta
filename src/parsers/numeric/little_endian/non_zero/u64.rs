use std::num::NonZeroU64;

use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::{non_zero::LittleEndianNonZero, u64::LittleEndianU64},
    result::ParseResult,
};

pub type LittleEndianNonZeroU64 = LittleEndianNonZero<u64>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroU64 {
    type Output = NonZeroU64;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroU64)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU64::parse(parser)?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::LittleEndianNonZeroU64),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroU64> {
    type Output = Option<NonZeroU64>;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianU64)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianU64::parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}
