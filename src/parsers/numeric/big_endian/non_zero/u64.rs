use core::num::NonZeroU64;

use crate::{
    endianness::Endianness,
    errors::number::ExpectedNumberParseError,
    numeric_expectation::NumericExpectation,
    parser::Parser,
    parsers::numeric::big_endian::u64::BigEndianU64Parser,
    results::ParseResult,
    traits::{parsable::Parsable, parsable_definition::ParsableDefinition},
};

pub struct BigEndianNonZeroU64Parser;

impl ParsableDefinition<'_> for BigEndianNonZeroU64Parser {
    type Output = NonZeroU64;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: false,
            non_zero: true,
            endianness: Endianness::Big,
        })
    }
}

impl<E: From<ExpectedNumberParseError>> Parsable<'_, [u8], E> for BigEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let start = parser.position;

        let value = OptionalBigEndianNonZeroU64Parser.parse(parser)?;

        let Some(value) = value else {
            let end = parser.position;

            return parser
                .add_error_with_span(
                    start.span(end),
                    ExpectedNumberParseError(NumericExpectation::Long {
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

pub struct OptionalBigEndianNonZeroU64Parser;

impl ParsableDefinition<'_> for OptionalBigEndianNonZeroU64Parser {
    type Output = Option<NonZeroU64>;

    type Error = ExpectedNumberParseError;

    #[inline]
    fn error(&self) -> Self::Error {
        ExpectedNumberParseError(NumericExpectation::Long {
            signed: false,
            non_zero: false,
            endianness: Endianness::Big,
        })
    }
}

impl<E> Parsable<'_, [u8], E> for OptionalBigEndianNonZeroU64Parser {
    fn parse(&self, parser: &mut Parser<[u8], E>) -> ParseResult<Self::Output> {
        let value = BigEndianU64Parser.parse(parser)?;

        Ok(NonZeroU64::new(value))
    }
}
