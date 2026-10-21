use core::num::NonZeroI64;

use crate::{
    parse_error::{Endianness, ExpectedNumberParseError, NumericExpectation},
    parser::Parser,
    parsers::numeric::little_endian::i64::LittleEndianI64Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct LittleEndianNonZeroI64Parser;

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for LittleEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
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

impl ParsableDefinition<'_> for LittleEndianNonZeroI64Parser {
    type Output = NonZeroI64;

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

impl<E> Parsable<'_, [u8], E> for OptionalLittleEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = LittleEndianI64Parser.parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}

impl ParsableDefinition<'_> for OptionalLittleEndianNonZeroI64Parser {
    type Output = Option<NonZeroI64>;

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
