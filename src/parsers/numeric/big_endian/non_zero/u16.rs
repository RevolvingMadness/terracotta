use std::num::NonZeroU16;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::{non_zero::BigEndianNonZero, u16::BigEndianU16},
    result::ParseResult,
};

pub type BigEndianNonZeroU16 = BigEndianNonZero<u16>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianNonZeroU16 {
    type Output = NonZeroU16;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU16::parse(parser)?;

        let Some(value) = NonZeroU16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::BigEndianNonZeroU16),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianNonZeroU16 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianNonZeroU16)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<BigEndianNonZeroU16> {
    type Output = Option<NonZeroU16>;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianU16::parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<BigEndianNonZeroU16> {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianU16)
    }
}
