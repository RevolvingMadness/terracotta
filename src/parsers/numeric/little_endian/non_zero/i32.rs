use std::num::NonZeroI32;

use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::{i32::LittleEndianI32, non_zero::LittleEndianNonZero},
    result::ParseResult,
};

pub type LittleEndianNonZeroI32 = LittleEndianNonZero<i32>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroI32 {
    type Output = NonZeroI32;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroI32)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI32::parse(parser)?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::LittleEndianNonZeroI32),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroI32> {
    type Output = Option<NonZeroI32>;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianI32)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianI32::parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}
