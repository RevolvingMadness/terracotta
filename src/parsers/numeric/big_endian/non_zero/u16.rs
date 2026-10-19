use core::num::NonZeroU16;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::u16::BigEndianU16Parser,
    result::ParseResult,
};

pub struct BigEndianNonZeroU16Parser;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for BigEndianNonZeroU16Parser {
    type Output = NonZeroU16;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU16Parser.parse(parser)?;

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

impl<E: ExpectedNumericParseError> ParsableError<E> for BigEndianNonZeroU16Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::BigEndianNonZeroU16)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<BigEndianNonZeroU16Parser> {
    type Output = Option<NonZeroU16>;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianU16Parser.parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<BigEndianNonZeroU16Parser> {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::BigEndianU16)
    }
}
