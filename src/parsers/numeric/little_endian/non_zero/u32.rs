use core::num::NonZeroU32;

use crate::{
    parsable_traits::{Parsable, ParsableError},
    parse_error::{ExpectedNumericParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::u32::LittleEndianU32Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroU32Parser;

impl<E: ExpectedNumericParseError> Parsable<'_, [u8], E> for LittleEndianNonZeroU32Parser {
    type Output = NonZeroU32;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU32Parser.parse(parser)?;

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

impl<E: ExpectedNumericParseError> ParsableError<E> for LittleEndianNonZeroU32Parser {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianNonZeroU32)
    }
}

impl<E> Parsable<'_, [u8], E> for Option<LittleEndianNonZeroU32Parser> {
    type Output = Option<NonZeroU32>;

    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianU32Parser.parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}

impl<E: ExpectedNumericParseError> ParsableError<E> for Option<LittleEndianNonZeroU32Parser> {
    #[inline]
    fn error(&self) -> E {
        E::expected_numeric(NumericExpectation::LittleEndianU32)
    }
}
