use core::num::NonZeroU32;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::u32::LittleEndianU32Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroU32Parser;

impl ParsableDefinition<'_> for LittleEndianNonZeroU32Parser {
    type Output = NonZeroU32;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianNonZeroU32
    }
}

impl Parsable<'_, [u8]> for LittleEndianNonZeroU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU32Parser.parse(parser)?;

        let Some(value) = NonZeroU32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::LittleEndianNonZeroU32,
            );
        };

        Ok(value)
    }
}

pub struct OptionalLittleEndianNonZeroU32Parser;

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroU32Parser {
    type Output = Option<NonZeroU32>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianU32
    }
}

impl Parsable<'_, [u8]> for OptionalLittleEndianNonZeroU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = LittleEndianU32Parser.parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}
