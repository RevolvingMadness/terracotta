use core::num::NonZeroU64;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::u64::LittleEndianU64Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroU64Parser;

impl Parsable<'_, [u8]> for LittleEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianU64Parser.parse(parser)?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::LittleEndianNonZeroU64,
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for LittleEndianNonZeroU64Parser {
    type Output = NonZeroU64;

    type Error = NumericExpectation;

    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianNonZeroU64
    }
}

pub struct OptionalLittleEndianNonZeroU64Parser;

impl Parsable<'_, [u8]> for OptionalLittleEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = LittleEndianU64Parser.parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroU64Parser {
    type Output = Option<NonZeroU64>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::LittleEndianU64
    }
}
