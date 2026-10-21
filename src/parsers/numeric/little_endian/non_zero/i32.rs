use core::num::NonZeroI32;

use crate::{
    parse_error::{Endianness, ExpectedNumberParseError, NumericExpectation},
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
        ExpectedNumberParseError(NumericExpectation::Integer {
            signed: true,
            non_zero: true,
            endianness: Endianness::Little,
        })
    }
}

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for LittleEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position;

        let value = OptionalLittleEndianNonZeroI32Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Integer {
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

pub struct OptionalLittleEndianNonZeroI32Parser;

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroI32Parser {
    type Output = Option<NonZeroI32>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Integer {
            signed: true,
            non_zero: false,
            endianness: Endianness::Little,
        })
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalLittleEndianNonZeroI32Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianI32Parser.parse(parser)?;

        Ok(NonZeroI32::new(value))
    }
}
