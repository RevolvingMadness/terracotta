use core::num::NonZeroU32;

use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::u32::BigEndianU32Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroU32Parser;

impl<'input, E: From<ExpectedNumberParseError>> Parsable<'input, [u8], E>
    for BigEndianNonZeroU32Parser
{
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let start = parser.position;

        let value = OptionalBigEndianNonZeroU32Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Integer {
                        signed: false,
                        non_zero: true,
                        endianness: Endianness::Big,
                    }),
                )
                .into_parse_result();
        };

        Ok(value)
    }
}

impl ParsableDefinition for BigEndianNonZeroU32Parser {
    type Output<'input, I: Input + ?Sized + 'input> = NonZeroU32;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Integer {
            signed: false,
            non_zero: true,
            endianness: Endianness::Big,
        })
    }
}

pub struct OptionalBigEndianNonZeroU32Parser;

impl<'input, E> Parsable<'input, [u8], E> for OptionalBigEndianNonZeroU32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let value = BigEndianU32Parser.parse(parser)?;

        Ok(NonZeroU32::new(value))
    }
}

impl ParsableDefinition for OptionalBigEndianNonZeroU32Parser {
    type Output<'input, I: Input + ?Sized + 'input> = Option<NonZeroU32>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Integer {
            signed: false,
            non_zero: false,
            endianness: Endianness::Big,
        })
    }
}
