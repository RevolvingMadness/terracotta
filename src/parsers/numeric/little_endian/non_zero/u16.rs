use core::num::NonZeroU16;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::u16::LittleEndianU16Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroU16Parser;

impl ParsableDefinition<'_> for LittleEndianNonZeroU16Parser {
    type Output = NonZeroU16;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianNonZeroU16
    }
}

impl Parsable<'_, [u8]> for LittleEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU16Parser.parse(parser)?;

        let Some(value) = NonZeroU16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::LittleEndianNonZeroU16,
            );
        };

        Ok(value)
    }
}

pub struct OptionalLittleEndianNonZeroU16Parser;

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroU16Parser {
    type Output = Option<NonZeroU16>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianU16
    }
}

impl Parsable<'_, [u8]> for OptionalLittleEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = LittleEndianU16Parser.parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}
