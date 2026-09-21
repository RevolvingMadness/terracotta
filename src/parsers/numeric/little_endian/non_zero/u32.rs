use core::num::NonZeroU32;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::{non_zero::LittleEndianNonZero, u32::LittleEndianU32},
    result::ParseResult,
};

pub type LittleEndianNonZeroU32 = LittleEndianNonZero<u32>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroU32 {
    type Output = NonZeroU32;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU32::parse(parser)?;

        let Some(value) = NonZeroU32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::LittleEndianNonZeroU32),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianNonZeroU32 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroU32)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroU32> {
    type Output = Option<NonZeroU32>;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianU32::parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<LittleEndianNonZeroU32> {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianU32)
    }
}
