use core::num::NonZeroU16;

use crate::{
    parse_error::{ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::u16::LittleEndianU16Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianNonZeroU16Parser;

impl ParsableDefinition<'_> for LittleEndianNonZeroU16Parser {
    type Output = NonZeroU16;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianNonZeroU16)
    }
}

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for LittleEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position;

        let value = LittleEndianU16Parser.parse(parser)?;

        let Some(value) = NonZeroU16::new(value) else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::LittleEndianNonZeroU16),
                )
                .into_parse_result();
        };

        Ok(value)
    }
}

pub struct OptionalLittleEndianNonZeroU16Parser;

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroU16Parser {
    type Output = Option<NonZeroU16>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::LittleEndianU16)
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalLittleEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianU16Parser.parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}
