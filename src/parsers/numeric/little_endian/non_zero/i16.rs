use std::num::NonZeroI16;

use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::{i16::LittleEndianI16, non_zero::LittleEndianNonZero},
    result::ParseResult,
};

pub type LittleEndianNonZeroI16 = LittleEndianNonZero<i16>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroI16 {
    type Output = NonZeroI16;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroI16)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI16::parse(parser)?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::LittleEndianNonZeroI16),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroI16> {
    type Output = Option<NonZeroI16>;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianI16)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianI16::parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}
