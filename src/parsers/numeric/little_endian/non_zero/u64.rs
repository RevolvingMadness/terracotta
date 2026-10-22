use core::num::NonZeroU64;

use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::little_endian::u64::LittleEndianU64Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianNonZeroU64Parser;

impl<'input, E: From<ExpectedNumberParseError>> Parsable<'input, [u8], E>
    for LittleEndianNonZeroU64Parser
{
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input>> {
        let start = parser.position;

        let value = OptionalLittleEndianNonZeroU64Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Long {
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

impl ParsableDefinition for LittleEndianNonZeroU64Parser {
    type Output<'input> = NonZeroU64;

    type Error = ExpectedNumberParseError;

    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: false,
            non_zero: true,
            endianness: Endianness::Little,
        })
    }
}

pub struct OptionalLittleEndianNonZeroU64Parser;

impl<'input, E> Parsable<'input, [u8], E> for OptionalLittleEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input>> {
        let value = LittleEndianU64Parser.parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}

impl ParsableDefinition for OptionalLittleEndianNonZeroU64Parser {
    type Output<'input> = Option<NonZeroU64>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: false,
            non_zero: false,
            endianness: Endianness::Little,
        })
    }
}
