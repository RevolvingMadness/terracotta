use core::num::NonZeroU16;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::u16::BigEndianU16Parser,
    result::ParseResult,
};

pub struct BigEndianNonZeroU16Parser;

impl ParsableDefinition<'_> for BigEndianNonZeroU16Parser {
    type Output = NonZeroU16;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianNonZeroU16
    }
}

impl Parsable<'_, [u8]> for BigEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU16Parser.parse(parser)?;

        let Some(value) = NonZeroU16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::BigEndianNonZeroU16,
            );
        };

        Ok(value)
    }
}

pub struct OptionalBigEndianNonZeroU16Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroU16Parser {
    type Output = Option<NonZeroU16>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianU16
    }
}

impl Parsable<'_, [u8]> for OptionalBigEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = BigEndianU16Parser.parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}
