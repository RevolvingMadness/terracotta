use core::num::NonZeroI16;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::i16::BigEndianI16Parser,
    result::ParseResult,
};

pub struct BigEndianNonZeroI16Parser;

impl Parsable<'_, [u8]> for BigEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI16Parser.parse(parser)?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::BigEndianNonZeroI16,
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for BigEndianNonZeroI16Parser {
    type Output = NonZeroI16;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianNonZeroI16
    }
}

pub struct OptionalBigEndianNonZeroI16Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroI16Parser {
    type Output = Option<NonZeroI16>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianI16
    }
}

impl Parsable<'_, [u8]> for OptionalBigEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = BigEndianI16Parser.parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}
