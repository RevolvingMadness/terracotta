use core::num::NonZeroI32;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::i32::BigEndianI32Parser,
    result::ParseResult,
};

pub struct BigEndianNonZeroI32Parser;

impl Parsable<'_, [u8]> for BigEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI32Parser.parse(parser)?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                NumericExpectation::BigEndianNonZeroI32,
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for BigEndianNonZeroI32Parser {
    type Output = NonZeroI32;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianNonZeroI32
    }
}

pub struct OptionalBigEndianNonZeroI32Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroI32Parser {
    type Output = Option<NonZeroI32>;

    type Error = NumericExpectation;

    #[inline]
    fn error(&self) -> Self::Error {
        NumericExpectation::BigEndianI32
    }
}

impl Parsable<'_, [u8]> for OptionalBigEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], Self::Error>) -> ParseResult<Self::Output> {
        let value = BigEndianI32Parser.parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}
