use core::num::NonZeroI32;

use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::big_endian::i32::BigEndianI32Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroI32Parser;

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for BigEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = BigEndianI32Parser.parse(parser)?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroI32),
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for BigEndianNonZeroI32Parser {
    type Output = NonZeroI32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianNonZeroI32)
    }
}

pub struct OptionalBigEndianNonZeroI32Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroI32Parser {
    type Output = Option<NonZeroI32>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::BigEndianI32)
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalBigEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianI32Parser.parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}
