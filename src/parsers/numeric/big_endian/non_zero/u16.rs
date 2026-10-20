use core::num::NonZeroU16;

use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::u16::BigEndianU16Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroU16Parser;

impl ParsableDefinition<'_> for BigEndianNonZeroU16Parser {
    type Output = NonZeroU16;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroU16)
    }
}

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for BigEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianU16Parser.parse(parser)?;

        let Some(value) = NonZeroU16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroU16),
            );
        };

        Ok(value)
    }
}

pub struct OptionalBigEndianNonZeroU16Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroU16Parser {
    type Output = Option<NonZeroU16>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianU16)
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalBigEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianU16Parser.parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}
