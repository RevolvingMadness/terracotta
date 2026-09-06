use std::num::NonZeroU16;

use crate::{
    parsable_traits::Parsable,
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::{non_zero::LittleEndianNonZero, u16::LittleEndianU16},
    result::ParseResult,
};

pub type LittleEndianNonZeroU16 = LittleEndianNonZero<u16>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroU16 {
    type Output = NonZeroU16;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroU16)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU16::parse(parser)?;

        let Some(value) = NonZeroU16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::LittleEndianNonZeroU16),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroU16> {
    type Output = Option<NonZeroU16>;

    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::LittleEndianU16)
    }

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianU16::parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}
