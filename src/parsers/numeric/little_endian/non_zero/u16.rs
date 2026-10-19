use core::num::NonZeroU16;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::u16::LittleEndianU16Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroU16Parser;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroU16Parser {
    type Output = NonZeroU16;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU16Parser.parse(parser)?;

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

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianNonZeroU16Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroU16)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroU16Parser> {
    type Output = Option<NonZeroU16>;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianU16Parser.parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<LittleEndianNonZeroU16Parser> {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianU16)
    }
}
