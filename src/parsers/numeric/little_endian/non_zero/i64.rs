use core::num::NonZeroI64;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::{i64::LittleEndianI64, non_zero::LittleEndianNonZero},
    result::ParseResult,
};

pub type LittleEndianNonZeroI64 = LittleEndianNonZero<i64>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroI64 {
    type Output = NonZeroI64;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI64::parse(parser)?;

        let Some(value) = NonZeroI64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::LittleEndianNonZeroI64),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianNonZeroI64 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroI64)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroI64> {
    type Output = Option<NonZeroI64>;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianI64::parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<LittleEndianNonZeroI64> {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianI64)
    }
}
