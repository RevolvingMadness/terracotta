use core::num::NonZeroI16;

use crate::{
    parsable_traits::{Parsable, ParsableDefinition},
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::i16::LittleEndianI16Parser,
    result::ParseResult,
};

pub struct LittleEndianNonZeroI16Parser;

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for LittleEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI16Parser.parse(parser)?;

        let Some(value) = NonZeroI16::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::LittleEndianNonZeroI16),
            );
        };

        Ok(value)
    }
}

impl ParsableDefinition<'_> for LittleEndianNonZeroI16Parser {
    type Output = NonZeroI16;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianNonZeroI16)
    }
}

pub struct OptionalLittleEndianNonZeroI16Parser;

impl<E> Parsable<'_, [u8], E> for OptionalLittleEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianI16Parser.parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroI16Parser {
    type Output = Option<NonZeroI16>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianI16)
    }
}
