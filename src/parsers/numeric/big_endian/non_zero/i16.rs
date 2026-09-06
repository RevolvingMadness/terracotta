use std::num::NonZeroI16;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{
        ExpectedNumericParseError,
        NumericExpectation::{self},
    },
    parser::Parser,
    parsers::numeric::big_endian::{i16::BigEndianI16, non_zero::BigEndianNonZero},
    result::ParseResult,
};

pub type BigEndianNonZeroI16 = BigEndianNonZero<i16>;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianNonZeroI16 {
    type Output = NonZeroI16;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI16::parse(parser)?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                E::expected_numeric(NumericExpectation::BigEndianNonZeroI16),
            );
        };

        Ok(value)
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianNonZeroI16 {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianNonZeroI16)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<BigEndianNonZeroI16> {
    type Output = Option<NonZeroI16>;

    fn parse(parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianI16::parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<BigEndianNonZeroI16> {
    #[inline]
    fn error() -> E {
        E::expected_numeric(NumericExpectation::BigEndianI16)
    }
}
