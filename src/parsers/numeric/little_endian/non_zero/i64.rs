use core::num::NonZeroI64;

use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::i64::LittleEndianI64Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianNonZeroI64Parser;

impl<'input, E: From<ExpectedNumberParseError>> Parsable<'input, [u8], E>
    for LittleEndianNonZeroI64Parser
{
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let start = parser.position;

        let value = OptionalLittleEndianNonZeroI64Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Long {
                        signed: true,
                        non_zero: true,
                        endianness: Endianness::Little,
                    }),
                )
                .into_parse_result();
        };

        Ok(value)
    }
}

impl ParsableDefinition for LittleEndianNonZeroI64Parser {
    type Output<'input, I: Input + ?Sized + 'input> = NonZeroI64;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: true,
            non_zero: true,
            endianness: Endianness::Little,
        })
    }
}

pub struct OptionalLittleEndianNonZeroI64Parser;

impl<'input, E> Parsable<'input, [u8], E> for OptionalLittleEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let value = LittleEndianI64Parser.parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}

impl ParsableDefinition for OptionalLittleEndianNonZeroI64Parser {
    type Output<'input, I: Input + ?Sized + 'input> = Option<NonZeroI64>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: true,
            non_zero: false,
            endianness: Endianness::Little,
        })
    }
}
