use core::num::NonZeroU16;

use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    input::Input,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::u16::BigEndianU16Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroU16Parser;

impl ParsableDefinition for BigEndianNonZeroU16Parser {
    type Output<'input, I: Input + ?Sized + 'input> = NonZeroU16;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Short {
            signed: false,
            non_zero: true,
            endianness: Endianness::Big,
        })
    }
}

impl<'input, E: From<ExpectedNumberParseError>> Parsable<'input, [u8], E>
    for BigEndianNonZeroU16Parser
{
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let start = parser.position;

        let value = OptionalBigEndianNonZeroU16Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Short {
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

pub struct OptionalBigEndianNonZeroU16Parser;

impl ParsableDefinition for OptionalBigEndianNonZeroU16Parser {
    type Output<'input, I: Input + ?Sized + 'input> = Option<NonZeroU16>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Short {
            signed: false,
            non_zero: false,
            endianness: Endianness::Big,
        })
    }
}

impl<'input, E> Parsable<'input, [u8], E> for OptionalBigEndianNonZeroU16Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output<'input, [u8]>> {
        let value = BigEndianU16Parser.parse(parser)?;

        Ok(NonZeroU16::new(value))
    }
}
