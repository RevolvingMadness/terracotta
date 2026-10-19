use core::num::NonZeroI64;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::i64::BigEndianI64Parser,
    result::ParseResult,
};

pub struct BigEndianNonZeroI64Parser;

impl Parsable<'_, [u8]> for BigEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI64Parser.parse(parser)?;

        let Some(value) = NonZeroI64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::BigEndianNonZeroI64,
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for BigEndianNonZeroI64Parser {
    type Output = NonZeroI64;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianNonZeroI64
    }
}

pub struct OptionalBigEndianNonZeroI64Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroI64Parser {
    type Output = Option<NonZeroI64>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianI64
    }
}

impl Parsable<'_, [u8]> for OptionalBigEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = BigEndianI64Parser.parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}
