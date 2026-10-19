use core::num::NonZeroU64;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::u64::BigEndianU64Parser,
    result::ParseResult,
};

pub struct BigEndianNonZeroU64Parser;

impl ParsableDefinition<'_> for BigEndianNonZeroU64Parser {
    type Output = NonZeroU64;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianNonZeroU64
    }
}

impl Parsable<'_, [u8]> for BigEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU64Parser.parse(parser)?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::BigEndianNonZeroU64,
            );
        };

        Ok(value)
    }
}

pub struct OptionalBigEndianNonZeroU64Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroU64Parser {
    type Output = Option<NonZeroU64>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianU64
    }
}

impl Parsable<'_, [u8]> for OptionalBigEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = BigEndianU64Parser.parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}
