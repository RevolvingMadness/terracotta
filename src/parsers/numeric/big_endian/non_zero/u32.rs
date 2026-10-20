use core::num::NonZeroU32;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::u32::BigEndianU32Parser,
    result::ParseResult,
};

pub struct BigEndianNonZeroU32Parser;

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for BigEndianNonZeroU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU32Parser.parse(parser)?;

        let Some(value) = NonZeroU32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroU32),
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for BigEndianNonZeroU32Parser {
    type Output = NonZeroU32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroU32)
    }
}

pub struct OptionalBigEndianNonZeroU32Parser;

impl<E> Parsable<'_, [u8], E> for OptionalBigEndianNonZeroU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianU32Parser.parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroU32Parser {
    type Output = Option<NonZeroU32>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianU32)
    }
}
