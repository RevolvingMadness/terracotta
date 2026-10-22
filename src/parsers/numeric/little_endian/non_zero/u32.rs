use core::num::NonZeroU32;

use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::u32::LittleEndianU32Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianNonZeroU32Parser;

impl ParsableDefinition for LittleEndianNonZeroU32Parser {
    type Output<'input, I: Input + ?Sized + 'input> = NonZeroU32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Integer {
            signed: false,
            non_zero: true,
            endianness: Endianness::Little,
        })
    }
}

impl<'input, E: From<ExpectedNumberParseError>> Parsable<'input, [u8], E>
    for LittleEndianNonZeroU32Parser
{
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let start = parser.position;

        let value = OptionalLittleEndianNonZeroU32Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Integer {
                        signed: false,
                        non_zero: true,
                        endianness: Endianness::Little,
                    }),
                )
                .into_parse_result();
        };

        Ok(value)
    }
}

pub struct OptionalLittleEndianNonZeroU32Parser;

impl ParsableDefinition for OptionalLittleEndianNonZeroU32Parser {
    type Output<'input, I: Input + ?Sized + 'input> = Option<NonZeroU32>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Integer {
            signed: false,
            non_zero: false,
            endianness: Endianness::Little,
        })
    }
}

impl<'input, E> Parsable<'input, [u8], E> for OptionalLittleEndianNonZeroU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let value = LittleEndianU32Parser.parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}
