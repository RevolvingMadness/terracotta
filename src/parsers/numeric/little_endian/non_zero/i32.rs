use core::num::NonZeroI32;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::i32::LittleEndianI32Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroI32Parser;

impl ParsableDefinition<'_> for LittleEndianNonZeroI32Parser {
    type Output = NonZeroI32;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianNonZeroI32
    }
}

impl Parsable<'_, [u8]> for LittleEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI32Parser.parse(parser)?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::LittleEndianNonZeroI32,
            );
        };

        Ok(value)
    }
}

pub struct OptionalLittleEndianNonZeroI32Parser;

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroI32Parser {
    type Output = Option<NonZeroI32>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianI32
    }
}

impl Parsable<'_, [u8]> for OptionalLittleEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = LittleEndianI32Parser.parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}
