use core::num::NonZeroU64;

use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::u64::BigEndianU64Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroU64Parser;

impl ParsableDefinition<'_> for BigEndianNonZeroU64Parser {
    type Output = NonZeroU64;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroU64)
    }
}

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for BigEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU64Parser.parse(parser)?;

        let Some(value) = NonZeroU64::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroU64),
            );
        };

        Ok(value)
    }
}

pub struct OptionalBigEndianNonZeroU64Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroU64Parser {
    type Output = Option<NonZeroU64>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianU64)
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalBigEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianU64Parser.parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}
