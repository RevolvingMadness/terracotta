use core::num::NonZeroI16;

use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::i16::BigEndianI16Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroI16Parser;

impl<'input, E: From<ExpectedNumberParseError>> Parsable<'input, [u8], E>
    for BigEndianNonZeroI16Parser
{
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let start = parser.position;

        let value = OptionalBigEndianNonZeroI16Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Short {
                        signed: true,
                        non_zero: true,
                        endianness: Endianness::Big,
                    }),
                )
                .into_parse_result();
        };

        Ok(value)
    }
}

impl ParsableDefinition for BigEndianNonZeroI16Parser {
    type Output<'input, I: Input + ?Sized + 'input> = NonZeroI16;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Short {
            signed: true,
            non_zero: true,
            endianness: Endianness::Big,
        })
    }
}

pub struct OptionalBigEndianNonZeroI16Parser;

impl ParsableDefinition for OptionalBigEndianNonZeroI16Parser {
    type Output<'input, I: Input + ?Sized + 'input> = Option<NonZeroI16>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Short {
            signed: true,
            non_zero: false,
            endianness: Endianness::Big,
        })
    }
}

impl<'input, E> Parsable<'input, [u8], E> for OptionalBigEndianNonZeroI16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let value = BigEndianI16Parser.parse(parser)?;

        Ok(NonZeroI16::new(value))
    }
}
