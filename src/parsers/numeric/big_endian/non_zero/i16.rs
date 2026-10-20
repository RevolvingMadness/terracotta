use core::num::NonZeroI16;

use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::i16::BigEndianI16Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroI16Parser;

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for BigEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI16Parser.parse(parser)?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroI16),
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for BigEndianNonZeroI16Parser {
    type Output = NonZeroI16;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroI16)
    }
}

pub struct OptionalBigEndianNonZeroI16Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroI16Parser {
    type Output = Option<NonZeroI16>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianI16)
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalBigEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianI16Parser.parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}
