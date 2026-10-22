use core::num::NonZeroI64;

use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::i64::BigEndianI64Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroI64Parser;

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for BigEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position;

        let value = OptionalBigEndianNonZeroI64Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Long {
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

impl ParsableDefinition<'_> for BigEndianNonZeroI64Parser {
    type Output = NonZeroI64;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: true,
            non_zero: true,
            endianness: Endianness::Big,
        })
    }
}

pub struct OptionalBigEndianNonZeroI64Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroI64Parser {
    type Output = Option<NonZeroI64>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: true,
            non_zero: false,
            endianness: Endianness::Big,
        })
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalBigEndianNonZeroI64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianI64Parser.parse(parser)?;

        Ok(NonZeroI64::new(value))
    }
}
