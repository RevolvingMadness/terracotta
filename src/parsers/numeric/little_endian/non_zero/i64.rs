use core::num::NonZeroI64;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::i64::LittleEndianI64Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroI64Parser;

impl Parsable<'_, [u8]> for LittleEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI64Parser.parse(parser)?;

        let Some(value) = NonZeroI64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::LittleEndianNonZeroI64,
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for LittleEndianNonZeroI64Parser {
    type Output = NonZeroI64;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianNonZeroI64
    }
}

pub struct OptionalLittleEndianNonZeroI64Parser;

impl Parsable<'_, [u8]> for OptionalLittleEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = LittleEndianI64Parser.parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroI64Parser {
    type Output = Option<NonZeroI64>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianI64
    }
}
