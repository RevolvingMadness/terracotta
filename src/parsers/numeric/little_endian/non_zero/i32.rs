use core::num::NonZeroI32;

use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::i32::LittleEndianI32Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianNonZeroI32Parser;

impl ParsableDefinition<'_> for LittleEndianNonZeroI32Parser {
    type Output = NonZeroI32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianNonZeroI32)
    }
}

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for LittleEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position();

        let value = LittleEndianI32Parser.parse(parser)?;

        let Some(value) = NonZeroI32::new(value) else {
            let end = parser.position();

            return parser.add_error_with_span_result(
                start.span(end),
                ExpectedNumberParseError(NumericExpectation::LittleEndianNonZeroI32),
            );
        };

        Ok(value)
    }
}

pub struct OptionalLittleEndianNonZeroI32Parser;

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroI32Parser {
    type Output = Option<NonZeroI32>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianI32)
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalLittleEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianI32Parser.parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}
