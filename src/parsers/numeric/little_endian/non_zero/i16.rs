use core::num::NonZeroI16;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::i16::LittleEndianI16Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroI16Parser;

impl Parsable<'_, [u8]> for LittleEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI16Parser.parse(parser)?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::LittleEndianNonZeroI16,
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for LittleEndianNonZeroI16Parser {
    type Output = NonZeroI16;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianNonZeroI16
    }
}

pub struct OptionalLittleEndianNonZeroI16Parser;

impl Parsable<'_, [u8]> for OptionalLittleEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = LittleEndianI16Parser.parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroI16Parser {
    type Output = Option<NonZeroI16>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianI16
    }
}
