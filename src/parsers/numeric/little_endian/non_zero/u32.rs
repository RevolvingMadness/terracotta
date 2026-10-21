use core::num::NonZeroU32;

use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::u32::LittleEndianU32Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianNonZeroU32Parser;

impl ParsableDefinition<'_> for LittleEndianNonZeroU32Parser {
    type Output = NonZeroU32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianNonZeroU32)
    }
}

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for LittleEndianNonZeroU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position;

        let value = LittleEndianU32Parser.parse(parser)?;

        let Some(value) = NonZeroU32::new(value) else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::LittleEndianNonZeroU32),
                )
                .into_parse_result();
        };

        Ok(value)
    }
}

pub struct OptionalLittleEndianNonZeroU32Parser;

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroU32Parser {
    type Output = Option<NonZeroU32>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianU32)
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalLittleEndianNonZeroU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianU32Parser.parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}
